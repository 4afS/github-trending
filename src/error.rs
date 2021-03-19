use derive_more::Display;
use serde::Serialize;
use thiserror::Error as ThisError;

#[derive(Serialize, Debug, Display, ThisError)]
#[display(fmt = "{}", message)]
pub struct TrendError {
    message: String,
}

impl TrendError {
    pub fn new(message: &str) -> Self {
        TrendError {
            message: message.to_string(),
        }
    }
}
