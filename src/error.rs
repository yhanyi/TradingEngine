use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum TradingError {
    InvalidOrder,
    ChannelSendError,
}

impl fmt::Display for TradingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TradingError::InvalidOrder => write!(f, "Invalid order"),
            TradingError::ChannelSendError =>
                write!(f, "Failed to send message through the channel"),
        }
    }
}

impl Error for TradingError {}
