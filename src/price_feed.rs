use tokio::time::{ interval, Duration };
use rand::Rng;
use crate::models::{ TradingPair, PriceUpdate };
use crate::engine::Message;

pub async fn run_price_feed(
    tx: mpsc::Sender<Message>,
    trading_pair: TraddingPair,
    initial_price: f64
) {
    let mut interval = interval(Duration::from_secs(1));
    let mut price = initial_price;
    let mut rng = rand::thread_rng();

    loop {
        interval.tick().await;
        let change_percentage = rng.gen_range(-0.01, 0.01);
        price *= 1.0 + change_percentage;
        let update = PriceUpdate {
            trading_pair: trading_pair.clone(),
            price,
            timestamp: chrono::Utc::now(),
        };
        if tx.send(Message::PriceUpdate(update)).await.is_err() {
            break;
        }
    }
}
