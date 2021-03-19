use derive_more::Display;
use serde::Serialize;
use thiserror::Error as ThisError;

#[derive(Serialize, Debug, Display, ThisError)]
pub enum TrendError {
    #[display(fmt = "{} not found", {0})]
    NotFound(String),
    #[display(fmt = "parse error")]
    ParseError,
}
