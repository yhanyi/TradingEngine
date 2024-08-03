use tokio;
use crate::v3::engine::start_engine;
use crate::v3::models::TradingPair;
use crate::v3::price_feed::run_price_feed;
use crate::v3::order_generator::run_order_generator;
use crate::v3::reporting::run_reporting;

#[tokio::main]
pub async fn testv3() {
    let tx = start_engine();

    let trading_pair = TradingPair {
        base: "BTC".to_string(),
        quote: "USD".to_string(),
    };

    // Start price feed
    let price_feed_tx = tx.clone();
    let price_feed_pair = trading_pair.clone();
    tokio::spawn(async move {
        run_price_feed(price_feed_tx, price_feed_pair, 50000.0).await;
    });

    // Start order generator
    let order_gen_tx = tx.clone();
    let order_gen_pair = trading_pair.clone();
    tokio::spawn(async move {
        run_order_generator(order_gen_tx, order_gen_pair).await;
    });

    // Start reporting
    let reporting_tx = tx.clone();
    let reporting_pair = trading_pair.clone();
    tokio::spawn(async move {
        run_reporting(reporting_tx, reporting_pair).await;
    });

    // Run for 1 minute then shutdown
    tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    tx.send(crate::v3::engine::Message::Shutdown).await.unwrap();
}
