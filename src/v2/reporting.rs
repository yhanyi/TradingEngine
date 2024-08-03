use tokio::time::{ interval, Duration };
use tokio::sync::mpsc;
use crate::v2::engine::Message;
use crate::v2::models::TradingPair;

pub async fn run_reporting(tx: mpsc::Sender<Message>, trading_pair: TradingPair) {
    let mut interval = interval(Duration::from_secs(5));

    loop {
        interval.tick().await;

        let (price_tx, mut price_rx) = mpsc::channel(1);
        if tx.send(Message::GetPrice(trading_pair.clone(), price_tx)).await.is_err() {
            break;
        }

        match price_rx.recv().await {
            Some(Some(price)) => {
                println!("Current price for {:?}: {:.2}", trading_pair, price);
            }
            Some(None) => {
                println!("No current price available for {:?}", trading_pair);
            }
            None => {
                println!("Failed to receive price update");
                break;
            }
        }

        if tx.send(Message::MatchOrders(trading_pair.clone())).await.is_err() {
            break;
        }
    }
}
