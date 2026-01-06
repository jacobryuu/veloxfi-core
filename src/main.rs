mod api;
mod core;

use crate::api::start_server;
use crate::core::engine::Engine;
use crate::core::market::Market;
use crate::core::order::{Order, Side};
use crate::core::persister::Persister;
use log::info;
use std::fs;
use std::path::PathBuf;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Create data directory if it doesn't exist
    fs::create_dir_all("data")?;

    let events = PathBuf::from("data/events.log");
    let snapshot = PathBuf::from("data/snapshot.json");
    let persister = Persister::new(events, snapshot);
    let mut engine = Engine::new(persister);

    // Setup initial data
    let market = Market::new("BTC-USD", "BTC", "USD", 1);
    engine.create_market(market);

    // Create accounts
    engine.create_account("alice".to_string());
    engine.create_account("bob".to_string());

    // Deposit initial balances
    engine
        .deposit("alice".to_string(), "USD", 100_000_000)
        .unwrap();
    engine
        .deposit("bob".to_string(), "BTC", 100_000_000)
        .unwrap();

    info!(
        "Alice USD balance: {}",
        engine.get_account_balance(&"alice".to_string(), "USD")
    );
    info!(
        "Bob BTC balance: {}",
        engine.get_account_balance(&"bob".to_string(), "BTC")
    );

    let buy = Order::new(1, "alice", "BTC-USD", Side::Buy, 100_000, 1, 1);
    let sell = Order::new(2, "bob", "BTC-USD", Side::Sell, 100_000, 1, 2);

    match engine.place_limit_order(buy) {
        Ok(trades) => info!("Placed buy, trades: {:?}", trades),
        Err(e) => info!("Error placing buy: {:?}", e),
    }

    match engine.place_limit_order(sell) {
        Ok(trades) => info!("Placed sell, trades: {:?}", trades),
        Err(e) => info!("Error placing sell: {:?}", e),
    }

    info!(
        "Alice USD balance after trade: {}",
        engine.get_account_balance(&"alice".to_string(), "USD")
    );
    info!(
        "Alice BTC balance after trade: {}",
        engine.get_account_balance(&"alice".to_string(), "BTC")
    );
    info!(
        "Bob USD balance after trade: {}",
        engine.get_account_balance(&"bob".to_string(), "USD")
    );
    info!(
        "Bob BTC balance after trade: {}",
        engine.get_account_balance(&"bob".to_string(), "BTC")
    );

    // replay and print orders
    engine.replay_events_from_file().unwrap();
    info!("Replay complete");

    info!(
        "Alice position: {:?}",
        engine.get_position("alice", "BTC-USD")
    );
    info!("Bob position: {:?}", engine.get_position("bob", "BTC-USD"));

    // Start HTTP server
    info!("Starting veloxfi-core service");
    match std::env::var("SERVER_HOST") {
        Ok(val) => info!("SERVER_HOST is {}", val),
        Err(e) => info!("Error reading SERVER_HOST: {}", e),
    }
    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port_str = std::env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let port = port_str.parse::<u16>().expect("Invalid port number");
    start_server(engine, &host, port).await
}
