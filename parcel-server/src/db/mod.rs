use std::{fmt::Display, num::TryFromIntError};

pub mod models;
pub mod schema;

#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    QueryError(diesel::result::Error),
    OutOfRangeValue,
}

impl Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryError::QueryError(err) => write!(f, "Invalid query: {}", err),
            QueryError::OutOfRangeValue => write!(
                f,
                "A column's value whose type is wider than the rust representation was out of range"
            ),
        }
    }
}

impl From<diesel::result::Error> for QueryError {
    fn from(value: diesel::result::Error) -> Self {
        QueryError::QueryError(value)
    }
}

impl From<TryFromIntError> for QueryError {
    fn from(_value: TryFromIntError) -> Self {
        QueryError::OutOfRangeValue
    }
}
