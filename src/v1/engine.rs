use std::sync::mpsc;
use std::thread;

use crate::v1::models::Order;
use crate::v1::order_book::{ OrderBook, SimpleOrderBook };

pub enum Message {
    NewOrder(Order),
    MatchOrders,
    Shutdown,
}

pub fn run_order_book(rx: mpsc::Receiver<Message>) {
    let mut order_book = SimpleOrderBook {
        buy_orders: Vec::new(),
        sell_orders: Vec::new(),
    };

    for message in rx {
        match message {
            Message::NewOrder(order) => {
                order_book.add_order(order);
            }

            Message::MatchOrders => {
                let trades = order_book.match_orders();
                println!("Executed trades: {:?}", trades);
            }

            Message::Shutdown => {
                break;
            }
        }
    }
}

pub fn start_engine() -> mpsc::Sender<Message> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        run_order_book(rx);
    });

    tx
}
