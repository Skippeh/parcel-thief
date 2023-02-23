pub mod accounts;
pub mod player_profiles;

use std::sync::Arc;

use diesel::{Connection, ConnectionResult, PgConnection};
use futures_util::lock::{Mutex, MutexLockFuture};

use self::{accounts::Accounts, player_profiles::PlayerProfiles};

pub struct Database {
    database_url: String,
}

impl Database {
    pub fn new(database_url: &str) -> Self {
        Self {
            database_url: database_url.into(),
        }
    }

    pub fn connect(&self) -> ConnectionResult<DatabaseConnection> {
        let conn = PgConnection::establish(&self.database_url)?;
        Ok(DatabaseConnection::new(self, conn))
    }
}

pub struct DatabaseConnection<'a> {
    connection: Arc<Mutex<PgConnection>>,

    /// Technically not used for anything, but it's borrowed so we can't accidentally leave a connection somewhere past the database struct's life
    db: &'a Database,
}

impl<'db> DatabaseConnection<'db> {
    pub fn new(db: &'db Database, connection: PgConnection) -> Self {
        Self {
            db,
            connection: Arc::new(Mutex::new(connection)),
        }
    }

    pub fn accounts(&self) -> Accounts {
        Accounts::new(self)
    }

    pub fn player_profiles(&self) -> PlayerProfiles {
        PlayerProfiles::new(self)
    }

    fn get_pg_connection(&self) -> MutexLockFuture<PgConnection> {
        self.connection.lock()
    }
}
