mod models;
mod order_book;
mod engine;
mod errors;

use std::error::Error;
use std::sync::mpsc;
use models::{ Order, OrderType };
use engine::{ Message, start_engine };
use errors::TradingError;

fn send_order(tx: &mpsc::Sender<Message>, order: Order) -> Result<(), Box<dyn Error>> {
    if order.quantity == 0 {
        return Err(Box::new(TradingError::InvalidOrder));
    }
    Ok(tx.send(Message::NewOrder(order)).map_err(|_| TradingError::ChannelSendError)?)
}

// fn main() {
//     println!("Hello, world!");
// }

fn main() -> Result<(), Box<dyn Error>> {
    let tx = start_engine();
    send_order(&tx, Order { id: 1, order_type: OrderType::Buy, price: 100.0, quantity: 10 })?;
    send_order(&tx, Order { id: 2, order_type: OrderType::Sell, price: 90.0, quantity: 5 })?;
    tx.send(Message::MatchOrders)?;
    tx.send(Message::Shutdown)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::order_book::{ OrderBook, SimpleOrderBook };

    #[test]
    fn test_order_matching() {
        let mut order_book = SimpleOrderBook {
            buy_orders: Vec::new(),
            sell_orders: Vec::new(),
        };

        order_book.add_order(Order {
            id: 1,
            order_type: OrderType::Buy,
            price: 100.0,
            quantity: 10,
        });
        order_book.add_order(Order {
            id: 2,
            order_type: OrderType::Sell,
            price: 99.0,
            quantity: 5,
        });

        let trades = order_book.match_orders();
        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].buy_order_id, 1);
        assert_eq!(trades[0].sell_order_id, 2);
        assert_eq!(trades[0].price, 99.0);
        assert_eq!(trades[0].quantity, 5);
    }

    #[test]
    fn test_send_order() {
        let tx = start_engine();
        let result = send_order(&tx, Order {
            id: 1,
            order_type: OrderType::Buy,
            price: 100.0,
            quantity: 10,
        });
        assert!(result.is_ok());

        let result = send_order(&tx, Order {
            id: 2,
            order_type: OrderType::Sell,
            price: 99.0,
            quantity: 0,
        });
        assert!(result.is_err());
    }
}
