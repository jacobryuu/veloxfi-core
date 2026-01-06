use crate::core::account::{Account, AccountId};
use crate::core::events::{Event, TradeEvent};
use crate::core::market::Market;
use crate::core::order::{Order, OrderId};
use crate::core::orderbook::{OrderBook, Trade};
use crate::core::persister::Persister;
use crate::core::positions::PositionsStore;
use std::collections::HashMap;

#[derive(Debug)]
pub enum EngineError {
    NotFound,
    PersistenceError,
    #[allow(dead_code)]
    InsufficientBalance,
}

pub struct Engine {
    orderbooks: HashMap<String, OrderBook>,
    orders_index: HashMap<OrderId, String>, // order_id -> market
    orders_map: HashMap<OrderId, Order>,    // order_id -> Order (to find account)
    positions: PositionsStore,
    accounts: HashMap<AccountId, Account>,
    persister: Persister,
}

impl Engine {
    pub fn new(persister: Persister) -> Self {
        Self {
            orderbooks: HashMap::new(),
            orders_index: HashMap::new(),
            orders_map: HashMap::new(),
            positions: PositionsStore::new(),
            accounts: HashMap::new(),
            persister,
        }
    }

    pub fn create_market(&mut self, market: Market) {
        let symbol = market.symbol.clone();
        self.orderbooks
            .insert(symbol.clone(), OrderBook::new(market));
    }

    pub fn create_account(&mut self, account_id: AccountId) {
        self.accounts
            .entry(account_id.clone())
            .or_insert_with(|| Account::new(account_id));
    }

    pub fn deposit(
        &mut self,
        account_id: AccountId,
        asset: &str,
        amount: u64,
    ) -> Result<(), EngineError> {
        if let Some(account) = self.accounts.get_mut(&account_id) {
            account.deposit(asset, amount);
            let ev = Event::Deposit {
                account_id,
                asset: asset.to_string(),
                amount,
            };
            self.persister
                .append_event(&ev)
                .map_err(|_| EngineError::PersistenceError)?;
            Ok(())
        } else {
            Err(EngineError::NotFound)
        }
    }

    #[allow(dead_code)]
    pub fn withdraw(
        &mut self,
        account_id: AccountId,
        asset: &str,
        amount: u64,
    ) -> Result<(), EngineError> {
        if let Some(account) = self.accounts.get_mut(&account_id) {
            account
                .withdraw(asset, amount)
                .map_err(|_| EngineError::InsufficientBalance)?;
            let ev = Event::Withdraw {
                account_id,
                asset: asset.to_string(),
                amount,
            };
            self.persister
                .append_event(&ev)
                .map_err(|_| EngineError::PersistenceError)?;
            Ok(())
        } else {
            Err(EngineError::NotFound)
        }
    }

    pub fn get_account_balance(&self, account_id: &AccountId, asset: &str) -> u64 {
        self.accounts
            .get(account_id)
            .map_or(0, |acc| acc.get_balance(asset))
    }

    pub fn place_limit_order(&mut self, order: Order) -> Result<Vec<TradeEvent>, EngineError> {
        // persist event
        let ev = Event::OrderPlaced(order.clone());
        self.persister
            .append_event(&ev)
            .map_err(|_| EngineError::PersistenceError)?;
        // insert
        let market = order.market.clone();
        if let Some(ob) = self.orderbooks.get_mut(&market) {
            // record index and store order
            self.orders_index.insert(order.id, market.clone());
            self.orders_map.insert(order.id, order.clone());
            let trades = ob.insert_order(order.clone());
            let mut out: Vec<TradeEvent> = Vec::new();
            for t in &trades {
                let te = TradeEvent {
                    trade_id: t.trade_id,
                    buy_order_id: t.buy_order_id,
                    sell_order_id: t.sell_order_id,
                    price: t.price,
                    qty: t.qty,
                    timestamp: t.timestamp,
                };
                let ev2 = Event::Trade(te.clone());
                self.persister
                    .append_event(&ev2)
                    .map_err(|_| EngineError::PersistenceError)?;
                // apply to positions by looking up accounts from orders_map
                let buy_acc_id = self
                    .orders_map
                    .get(&t.buy_order_id)
                    .map(|o| o.account.clone())
                    .unwrap_or_else(|| "buyer".to_string());
                let sell_acc_id = self
                    .orders_map
                    .get(&t.sell_order_id)
                    .map(|o| o.account.clone())
                    .unwrap_or_else(|| "seller".to_string());

                // Update balances for buyer and seller
                if let Some(buy_account) = self.accounts.get_mut(&buy_acc_id) {
                    let _ = buy_account.withdraw(
                        order.market.split('-').next_back().unwrap(),
                        t.price * t.qty,
                    );
                    buy_account.deposit(order.market.split('-').next().unwrap(), t.qty);
                }
                if let Some(sell_account) = self.accounts.get_mut(&sell_acc_id) {
                    let _ = sell_account.withdraw(order.market.split('-').next().unwrap(), t.qty);
                    sell_account.deposit(
                        order.market.split('-').next_back().unwrap(),
                        t.price * t.qty,
                    );
                }

                self.positions
                    .apply_trade(t, &market, &buy_acc_id, &sell_acc_id);
                out.push(te);
            }
            Ok(out)
        } else {
            Err(EngineError::NotFound)
        }
    }

    #[allow(dead_code)]
    pub fn cancel_order(&mut self, order_id: OrderId) -> Result<(), EngineError> {
        // find market
        if let Some(market) = self.orders_index.get(&order_id) {
            if let Some(ob) = self.orderbooks.get_mut(market) {
                if let Some(_order) = ob.cancel_order(order_id) {
                    let ev = Event::OrderCancelled { order_id };
                    self.persister
                        .append_event(&ev)
                        .map_err(|_| EngineError::PersistenceError)?;
                    self.orders_map.remove(&order_id);
                    self.orders_index.remove(&order_id);
                    return Ok(());
                }
            }
            Err(EngineError::NotFound)
        } else {
            Err(EngineError::NotFound)
        }
    }

    pub fn replay_events_from_file(&mut self) -> Result<(), EngineError> {
        let events = self
            .persister
            .read_events()
            .map_err(|_| EngineError::PersistenceError)?;
        for ev in events {
            match ev {
                Event::OrderPlaced(o) => {
                    // re-insert without persisting and store order
                    if let Some(ob) = self.orderbooks.get_mut(&o.market) {
                        ob.insert_order(o.clone());
                        self.orders_map.insert(o.id, o.clone());
                        self.orders_index.insert(o.id, o.market.clone());
                    }
                }
                Event::Trade(t) => {
                    // apply to positions using accounts from orders_map if available
                    let trade = Trade {
                        trade_id: t.trade_id,
                        buy_order_id: t.buy_order_id,
                        sell_order_id: t.sell_order_id,
                        price: t.price,
                        qty: t.qty,
                        timestamp: t.timestamp,
                    };
                    let buy_acc_id = self
                        .orders_map
                        .get(&t.buy_order_id)
                        .map(|o| o.account.clone())
                        .unwrap_or_else(|| "buyer".to_string());
                    let sell_acc_id = self
                        .orders_map
                        .get(&t.sell_order_id)
                        .map(|o| o.account.clone())
                        .unwrap_or_else(|| "seller".to_string());

                    // Replay balance updates
                    if let Some(buy_account) = self.accounts.get_mut(&buy_acc_id) {
                        let _ = buy_account.withdraw("USD", t.price * t.qty);
                        buy_account.deposit("BTC", t.qty);
                    }
                    if let Some(sell_account) = self.accounts.get_mut(&sell_acc_id) {
                        let _ = sell_account.withdraw("BTC", t.qty);
                        sell_account.deposit("USD", t.price * t.qty);
                    }

                    self.positions
                        .apply_trade(&trade, "", &buy_acc_id, &sell_acc_id);
                }
                Event::OrderCancelled { order_id } => {
                    // remove from index
                    self.orders_map.remove(&order_id);
                    self.orders_index.remove(&order_id);
                }
                Event::Snapshot => {}
                Event::Deposit {
                    account_id,
                    asset,
                    amount,
                } => {
                    if let Some(account) = self.accounts.get_mut(&account_id) {
                        account.deposit(&asset, amount);
                    }
                }
                Event::Withdraw {
                    account_id,
                    asset,
                    amount,
                } => {
                    if let Some(account) = self.accounts.get_mut(&account_id) {
                        account.withdraw(&asset, amount).unwrap(); // Assuming withdraw during replay won't fail
                    }
                }
            }
        }
        Ok(())
    }

    pub fn get_position(
        &self,
        account: &str,
        market: &str,
    ) -> Option<&crate::core::positions::Position> {
        self.positions.get(account, market)
    }
}
