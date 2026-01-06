use crate::core::market::Market;
use crate::core::order::{Order, OrderId, Side};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub trade_id: u64,
    pub buy_order_id: OrderId,
    pub sell_order_id: OrderId,
    pub price: u64,
    pub qty: u64,
    pub timestamp: i64,
}

#[derive(Debug)]
struct PriceLevel {
    orders: VecDeque<Order>,
}

impl PriceLevel {
    fn new() -> Self {
        Self {
            orders: VecDeque::new(),
        }
    }
}

#[derive(Debug)]
pub struct OrderBook {
    #[allow(dead_code)]
    pub market: Market,
    // buys: price -> orders (highest price first). We'll use BTreeMap and iterate in reverse for buys.
    buys: BTreeMap<u64, PriceLevel>,
    // sells: price -> orders (lowest price first)
    sells: BTreeMap<u64, PriceLevel>,
    next_trade_id: u64,
}

impl OrderBook {
    pub fn new(market: Market) -> Self {
        Self {
            market,
            buys: BTreeMap::new(),
            sells: BTreeMap::new(),
            next_trade_id: 1,
        }
    }

    fn process_match_level(
        level: &mut PriceLevel,
        order: &mut Order,
        price: u64,
        next_trade_id: &mut u64,
        trades: &mut Vec<Trade>,
        timestamp: i64,
        is_buy: bool,
    ) {
        while order.remaining > 0 {
            if let Some(opp_order) = level.orders.front_mut() {
                let trade_qty = std::cmp::min(order.remaining, opp_order.remaining);
                let trade = if is_buy {
                    Trade {
                        trade_id: *next_trade_id,
                        buy_order_id: order.id,
                        sell_order_id: opp_order.id,
                        price,
                        qty: trade_qty,
                        timestamp,
                    }
                } else {
                    Trade {
                        trade_id: *next_trade_id,
                        buy_order_id: opp_order.id,
                        sell_order_id: order.id,
                        price,
                        qty: trade_qty,
                        timestamp,
                    }
                };
                *next_trade_id += 1;
                trades.push(trade);
                order.remaining -= trade_qty;
                opp_order.remaining -= trade_qty;
                if opp_order.remaining == 0 {
                    level.orders.pop_front();
                }
                if order.remaining == 0 {
                    break;
                }
            } else {
                break;
            }
        }
    }

    pub fn insert_order(&mut self, mut order: Order) -> Vec<Trade> {
        let mut trades: Vec<Trade> = Vec::new();
        let timestamp = order.timestamp;

        match order.side {
            Side::Buy => {
                // match against lowest sell prices
                let mut to_remove_prices = Vec::new();
                let mut prices: Vec<u64> = self.sells.keys().cloned().collect();
                prices.sort();
                for price in prices {
                    if order.remaining == 0 {
                        break;
                    }
                    if price > order.price {
                        break;
                    }
                    if let Some(level) = self.sells.get_mut(&price) {
                        Self::process_match_level(
                            level,
                            &mut order,
                            price,
                            &mut self.next_trade_id,
                            &mut trades,
                            timestamp,
                            true,
                        );
                        if level.orders.is_empty() {
                            to_remove_prices.push(price);
                        }
                    }
                }
                for p in to_remove_prices {
                    self.sells.remove(&p);
                }
                if order.remaining > 0 {
                    self.buys
                        .entry(order.price)
                        .or_insert_with(PriceLevel::new)
                        .orders
                        .push_back(order);
                }
            }
            Side::Sell => {
                // match against highest buy prices
                let mut to_remove_prices = Vec::new();
                let mut prices: Vec<u64> = self.buys.keys().cloned().collect();
                prices.sort_by(|a, b| b.cmp(a));
                for price in prices {
                    if order.remaining == 0 {
                        break;
                    }
                    if price < order.price {
                        break;
                    }
                    if let Some(level) = self.buys.get_mut(&price) {
                        Self::process_match_level(
                            level,
                            &mut order,
                            price,
                            &mut self.next_trade_id,
                            &mut trades,
                            timestamp,
                            false,
                        );
                        if level.orders.is_empty() {
                            to_remove_prices.push(price);
                        }
                    }
                }
                for p in to_remove_prices {
                    self.buys.remove(&p);
                }
                if order.remaining > 0 {
                    self.sells
                        .entry(order.price)
                        .or_insert_with(PriceLevel::new)
                        .orders
                        .push_back(order);
                }
            }
        }
        trades
    }

    #[allow(dead_code)]
    pub fn cancel_order(&mut self, order_id: OrderId) -> Option<Order> {
        // search buys
        if let Some(removed) = Self::find_and_remove_order(&mut self.buys, order_id) {
            return Some(removed);
        }
        // search sells
        Self::find_and_remove_order(&mut self.sells, order_id)
    }

    fn find_and_remove_order(
        book: &mut BTreeMap<u64, PriceLevel>,
        order_id: OrderId,
    ) -> Option<Order> {
        for (_price, level) in book.iter_mut() {
            for i in 0..level.orders.len() {
                if level.orders[i].id == order_id {
                    let removed = level.orders.remove(i).unwrap();
                    return Some(removed);
                }
            }
        }
        None
    }
}
