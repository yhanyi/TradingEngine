use crate::models::{ Order, Trade };

pub trait OrderBook {
    fn add_order(&mut self, order: Order);
    fn match_orders(&mut self) -> Vec<Trade>;
}

pub struct SimpleOrderBook {
    pub buy_orders: Vec<Order>,
    pub sell_orders: Vec<Order>,
}

impl OrderBook for SimpleOrderBook {
    fn add_order(&mut self, order: Order) {
        match order.order_type {
            crate::models::OrderType::Buy => self.buy_orders.push(order),
            crate::models::OrderType::Sell => self.sell_orders.push(order),
        }
    }

    fn match_orders(&mut self) -> Vec<Trade> {
        let mut trades = Vec::new();
        self.buy_orders.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
        self.sell_orders.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

        while let (Some(buy), Some(sell)) = (self.buy_orders.first(), self.sell_orders.first()) {
            if buy.price >= sell.price {
                let quantity = buy.quantity.min(sell.quantity);
                trades.push(Trade {
                    buy_order_id: buy.id,
                    sell_order_id: sell.id,
                    price: sell.price,
                    quantity,
                });

                self.buy_orders[0].quantity -= quantity;
                self.sell_orders[0].quantity -= quantity;

                if self.buy_orders[0].quantity == 0 {
                    self.buy_orders.remove(0);
                }
                if self.sell_orders[0].quantity == 0 {
                    self.sell_orders.remove(0);
                }
            } else {
                break;
            }
        }
        trades
    }
}
