use serde::Serialize;
use thiserror::Error as ThisError;

#[derive(Serialize, Debug, ThisError)]
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

impl std::fmt::Display for TrendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
