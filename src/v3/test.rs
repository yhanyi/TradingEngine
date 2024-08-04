use crate::v3::cli::run_cli;
use crate::v3::engine::start_engine;
use crate::v3::models::TradingPair;
use crate::v3::price_feed::run_price_feed;
use crate::v3::order_generator::run_order_generator;
use crate::v3::reporting::run_reporting;

pub async fn testv3() {
    println!("Starting Trading Engine v3...");

    let tx = start_engine();
    println!("Engine started.");

    let trading_pair = TradingPair {
        base: "BTC".to_string(),
        quote: "USD".to_string(),
    };

    // Start price feed
    let price_feed_tx = tx.clone();
    let price_feed_pair = trading_pair.clone();
    tokio::spawn(async move {
        println!("Starting price feed...");
        run_price_feed(price_feed_tx, price_feed_pair, 50000.0).await;
    });

    // Start order generator
    let order_gen_tx = tx.clone();
    let order_gen_pair = trading_pair.clone();
    tokio::spawn(async move {
        println!("Starting order generator...");
        run_order_generator(order_gen_tx, order_gen_pair).await;
    });

    // Start reporting
    let reporting_tx = tx.clone();
    let reporting_pair = trading_pair.clone();
    tokio::spawn(async move {
        println!("Starting reporting...");
        run_reporting(reporting_tx, reporting_pair).await;
    });

    println!("All components started. Running CLI...");

    // Run CLI
    run_cli().await;

    println!("Trading Engine v3 shutting down.");
}
