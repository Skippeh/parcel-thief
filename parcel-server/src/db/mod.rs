use std::fmt::Display;

pub mod models;
pub mod schema;

#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    QueryError(diesel::result::Error),
}

impl Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            //QueryError::ConnectionError(err) => write!(f, "Could not connect to database: {}", err),
            QueryError::QueryError(err) => write!(f, "Invalid query: {}", err),
        }
    }
}

impl From<diesel::result::Error> for QueryError {
    fn from(value: diesel::result::Error) -> Self {
        QueryError::QueryError(value)
    }
}
