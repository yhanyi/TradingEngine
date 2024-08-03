use tokio::time::{ interval, Duration };
use crate::engine::Message;
use crate::models::TradingPair;

pub async fn run_reporting(tx: mpsc::Sender<Message>, trading_pair: TradingPair) {
    let mut interval = interval(Duration::from_secs(5));

    loop {
        interval.tick().await;

        let (price_tx, price_rx) = mpsc::channel(1);
        if tx.send(Message::GetPrice(trading_pair.clone(), price_tx)).await.is_err() {
            break;
        }

        if let Some(price) = price_rx.await.unwrap() {
            println!("Current price for {:?}: {:.2}", trading_pair, price);
        } else {
            println!("No current price available for {:?}", trading_pair);
        }

        if tx.send(Message::MatchOrders(trading_pair.clone())).await.is_err() {
            break;
        }
    }
}
