use std::path::PathBuf;
use veloxfi_core::core::engine::Engine;
use veloxfi_core::core::market::Market;
use veloxfi_core::core::order::Order;
use veloxfi_core::core::order::Side;
use veloxfi_core::core::persister::Persister;

#[test]
fn matching_happy_path() {
    let events = PathBuf::from("target/test-events.log");
    let snapshot = PathBuf::from("target/test-snapshot.json");
    // clean up if exist
    let _ = std::fs::remove_file(&events);
    let _ = std::fs::remove_file(&snapshot);

    let persister = Persister::new(events.clone(), snapshot.clone());
    let mut engine = Engine::new(persister);
    engine.create_market(Market::new("BTC-USD", "BTC", "USD", 1));

    let buy = Order::new(1, "alice", "BTC-USD", Side::Buy, 1000, 10, 1);
    let sell = Order::new(2, "bob", "BTC-USD", Side::Sell, 1000, 10, 2);

    let t1 = engine.place_limit_order(buy).expect("place buy");
    assert!(t1.is_empty()); // buy inserted, no match yet
    let t2 = engine.place_limit_order(sell).expect("place sell");
    assert!(!t2.is_empty());
    // positions may use placeholder accounts in PoC; ensure no panic and events persisted
    engine.replay_events_from_file().unwrap();
}
