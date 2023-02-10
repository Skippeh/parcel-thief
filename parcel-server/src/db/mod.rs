use std::fmt::Display;

use diesel::ConnectionError;

pub mod models;
pub mod schema;

#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    ConnectionError(diesel::ConnectionError),
    QueryError(diesel::result::Error),
}

impl Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryError::ConnectionError(err) => write!(f, "Could not connect to database: {}", err),
            QueryError::QueryError(err) => write!(f, "Invalid query: {}", err),
        }
    }
}

impl From<ConnectionError> for QueryError {
    fn from(value: ConnectionError) -> Self {
        QueryError::ConnectionError(value)
    }
}

impl From<diesel::result::Error> for QueryError {
    fn from(value: diesel::result::Error) -> Self {
        QueryError::QueryError(value)
    }
}
