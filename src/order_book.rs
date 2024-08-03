use async_trait::async_trait;
use tokio::sync::Mutex;
use std::collections::BTreeMap;
use crate::models::{ Order, Trade, TradingPair };

pub trait OrderBook {
    fn add_order(&mut self, order: Order);
    fn match_orders(&mut self) -> Vec<Trade>;
    async fn get_current_price(&self) -> Option<f64>;
}

pub struct SimpleOrderBook {
    trading_pair: TradingPair,
    buy_orders: Mutex<BTreeMap<f64, Vec<Order>>>,
    sell_orders: Mutex<BTreeMap<f64, Vec<Order>>>,
}

impl SimpleOrderBook {
    pub fn new(trading_pair: TradingPair) -> Self {
        SimpleOrderBook {
            trading_pair,
            buy_orders: Mutex::new(BTreeMap::new()),
            sell_orders: Mutex::new(BTreeMap::new()),
        }
    }
}

#[async_trait]
impl OrderBook for SimpleOrderBook {
    async fn add_order(&self, order: Order) {
        let orders = match order.order_type {
            crate::models::OrderType::Buy => &self.buy_orders,
            crate::models::OrderType::Sell => &self.sell_orders,
        };

        let mut orders = orders.lock().await;
        orders.entry(order.price).or_insert_with(Vec::new).push(order);
    }

    async fn match_orders(&self) -> Vec<Trade> {
        let mut trades = Vec::new();
        let mut buy_orders = self.buy_orders.lock().await;
        let mut sell_orders = self.sell_orders.lock().await;
        while
            let (Some((&buy_price, buy_list)), Some((&sell_price, sell_list))) = (
                buy_orders.iter_mut().next_back(),
                sell_orders.iter_mut().next(),
            )
        {
            if buy_price < sell_price {
                break;
            }
            let mut i = 0;
            let mut j = 0;
            while i < buy_list.len() && j < sell_list.len() {
                let buy = &mut buy_list[i];
                let sell = &mut sell_list[j];
                let trade_quantity = buy.quantity.min(sell.quantity);

                trades.push(Trade {
                    id: (trades.len() as u64) + 1,
                    trading_pair: self.trading_pair.clone(),
                    buy_order_id: buy.id,
                    sell_order_id: sell.id,
                    price: sell_price,
                    quantity: trade_quantity,
                    timestamp: chrono::Utc::now(),
                });

                buy.quantity -= trade_quantity;
                sell.quantity -= trade_quantity;

                if buy.quantity == 0.0 {
                    i += 1;
                }
                if sell.quantity == 0.0 {
                    j += 1;
                }
            }
            buy_list.drain(0..i);
            sell_list.drain(0..j);
            if buy_list.is_empty() {
                buy_orders.remove(&buy_price);
            }
            if sell_list.is_empty() {
                sell_orders.remove(&sell_price);
            }
        }
        trades
    }

    async fn get_current_price(&self) -> Option<f64> {
        let buy_orders = self.buy_orders.lock().await;
        let sell_orders = self.sell_orders.lock().await;
        match (buy_orders.keys().next_back(), sell_orders.keys().next()) {
            (Some(&bid), Some(&ask)) => Some((bid + ask) / 2.0),
            _ => None,
        }
    }
}
