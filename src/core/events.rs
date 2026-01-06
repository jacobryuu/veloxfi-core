use crate::core::account::AccountId;
use crate::core::order::Order;
use crate::core::order::OrderId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeEvent {
    pub trade_id: u64,
    pub buy_order_id: OrderId,
    pub sell_order_id: OrderId,
    pub price: u64,
    pub qty: u64,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    OrderPlaced(Order),
    OrderCancelled {
        order_id: OrderId,
    },
    Trade(TradeEvent),
    Deposit {
        account_id: AccountId,
        asset: String,
        amount: u64,
    },
    Withdraw {
        account_id: AccountId,
        asset: String,
        amount: u64,
    },
    Snapshot, // placeholder
}

impl Event {
    #[allow(dead_code)]
    pub fn to_json_line(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    #[allow(dead_code)]
    pub fn from_json_line(s: &str) -> serde_json::Result<Self> {
        serde_json::from_str(s)
    }
}
