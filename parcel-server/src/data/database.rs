use diesel::{Connection, ConnectionResult, PgConnection};

pub struct Database {
    database_url: String,
}

impl Database {
    pub fn new(database_url: &str) -> Self {
        Self {
            database_url: database_url.into(),
        }
    }

    pub fn connect(&self) -> ConnectionResult<PgConnection> {
        PgConnection::establish(&self.database_url)
    }
}
