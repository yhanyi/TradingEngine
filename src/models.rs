use chrono::{ DateTime, Utc };

#[derive(Debug, Clone, PartialEq)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: u64,
    pub trading_pair: TradingPair,
    pub order_type: OrderType,
    pub price: f64,
    pub quantity: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TradingPair {
    pub base: String,
    pub quote: String,
}

#[derive(Debug)]
pub struct Trade {
    pub id: u64,
    pub trading_pair: TradingPair,
    pub buy_order_id: u64,
    pub sell_order_id: u64,
    pub price: f64,
    pub quantity: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug)]
pub struct PriceUpdate {
    pub trading_pair: TradingPair,
    pub price: f64,
    pub timestamp: DateTime<Utc>,
}
