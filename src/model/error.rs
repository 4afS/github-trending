use derive_more::Display;
use thiserror::Error as ThisError;

#[derive(Debug, Display, ThisError)]
pub enum TrendError {
    #[display(fmt = "{} not found", _0)]
    NotFound(String),
    #[display(fmt = "given language is not exist")]
    GivenLanguageIsNotExist,
}
