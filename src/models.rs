#[derive(Debug, Clone, PartialEq)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub struct Order {
    id: u64,
    order_type: OrderType,
    price: f64,
    quantity: u32,
}

#[derive(Debug)]
pub struct Trade {
    buy_order_id: u64,
    sell_order_id: u64,
    price: f64,
    quantity: u32,
}
