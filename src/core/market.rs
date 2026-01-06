use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub symbol: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub tick_size: u64,
}

impl Market {
    pub fn new(symbol: &str, base: &str, quote: &str, tick_size: u64) -> Self {
        Self {
            symbol: symbol.to_string(),
            base_asset: base.to_string(),
            quote_asset: quote.to_string(),
            tick_size,
        }
    }
}
