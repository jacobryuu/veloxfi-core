use crate::core::order::AccountId;
use crate::core::orderbook::Trade;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub account: AccountId,
    pub market: String,
    pub quantity: i128,
    pub avg_entry_price: Option<u64>,
    pub realized_pnl: i128,
}

impl Position {
    pub fn new(account: &str, market: &str) -> Self {
        Self {
            account: account.to_string(),
            market: market.to_string(),
            quantity: 0,
            avg_entry_price: None,
            realized_pnl: 0,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PositionsStore {
    // key: (account, market)
    pub positions: HashMap<String, Position>,
}

impl PositionsStore {
    pub fn new() -> Self {
        Self {
            positions: HashMap::new(),
        }
    }

    fn key(account: &str, market: &str) -> String {
        format!("{}:{}", account, market)
    }

    pub fn get(&self, account: &str, market: &str) -> Option<&Position> {
        self.positions.get(&Self::key(account, market))
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self, account: &str, market: &str) -> Option<&mut Position> {
        self.positions.get_mut(&Self::key(account, market))
    }

    pub fn ensure(&mut self, account: &str, market: &str) -> &mut Position {
        let k = Self::key(account, market);
        self.positions
            .entry(k.clone())
            .or_insert_with(|| Position::new(account, market))
    }

    pub fn apply_trade(
        &mut self,
        trade: &Trade,
        market: &str,
        buy_account: &str,
        sell_account: &str,
    ) {
        // buy side
        let buy_pos = self.ensure(buy_account, market);
        // Increase buy quantity
        let _prev_qty = buy_pos.quantity;
        let trade_qty_i128 = trade.qty as i128;
        let trade_price_i128 = trade.price as i128;
        // Update avg price
        if buy_pos.quantity <= 0 {
            buy_pos.avg_entry_price = Some(trade.price);
        } else if let Some(prev_price) = buy_pos.avg_entry_price {
            let new_qty = buy_pos.quantity + trade_qty_i128;
            if new_qty != 0 {
                let new_price = ((buy_pos.quantity * prev_price as i128)
                    + (trade_qty_i128 * trade_price_i128))
                    / new_qty;
                buy_pos.avg_entry_price = Some(new_price as u64);
            }
        }
        buy_pos.quantity += trade_qty_i128;

        // sell side
        let sell_pos = self.ensure(sell_account, market);
        // Realized PnL when reducing a position on sell (naive using avg_entry_price)
        if sell_pos.quantity > 0 {
            // selling from long
            if let Some(avg) = sell_pos.avg_entry_price {
                let pnl = (trade.price as i128 - avg as i128) * trade_qty_i128;
                sell_pos.realized_pnl += pnl;
            }
        }
        sell_pos.quantity -= trade_qty_i128;
    }
}
