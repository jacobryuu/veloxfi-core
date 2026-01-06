use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type AccountId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: AccountId,
    pub balances: HashMap<String, u64>, // asset -> balance
}

impl Account {
    pub fn new(id: AccountId) -> Self {
        Self {
            id,
            balances: HashMap::new(),
        }
    }

    pub fn deposit(&mut self, asset: &str, amount: u64) {
        *self.balances.entry(asset.to_string()).or_insert(0) += amount;
    }

    pub fn withdraw(&mut self, asset: &str, amount: u64) -> Result<(), String> {
        let balance = self.balances.entry(asset.to_string()).or_insert(0);
        if *balance >= amount {
            *balance -= amount;
            Ok(())
        } else {
            Err(format!("Insufficient balance for asset {}", asset))
        }
    }

    pub fn get_balance(&self, asset: &str) -> u64 {
        *self.balances.get(asset).unwrap_or(&0)
    }
}
