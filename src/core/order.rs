use serde::{Deserialize, Serialize};

pub type OrderId = u64;
pub type AccountId = String;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrderType {
    Limit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: OrderId,
    pub account: AccountId,
    pub market: String,
    pub side: Side,
    pub order_type: OrderType,
    pub price: u64,
    pub qty: u64,
    pub remaining: u64,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[allow(dead_code)]
pub enum OrderStatus {
    Open,
    PartiallyFilled,
    Filled,
    Cancelled,
}

impl Order {
    pub fn new(
        id: OrderId,
        account: &str,
        market: &str,
        side: Side,
        price: u64,
        qty: u64,
        timestamp: i64,
    ) -> Self {
        Self {
            id,
            account: account.to_string(),
            market: market.to_string(),
            side,
            order_type: OrderType::Limit,
            price,
            qty,
            remaining: qty,
            timestamp,
        }
    }
}
