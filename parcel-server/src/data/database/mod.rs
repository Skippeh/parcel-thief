pub mod accounts;
pub mod frontend_accounts;
pub mod highway_resources;
pub mod likes;
pub mod missions;
pub mod player_profiles;
pub mod qpid_objects;
pub mod roads;
pub mod wasted_baggages;

use std::sync::Arc;

use diesel::{result::Error as DieselError, ConnectionResult};
use diesel_async::{
    scoped_futures::ScopedBoxFuture, AnsiTransactionManager, AsyncConnection, AsyncPgConnection,
    TransactionManager,
};
use futures_util::lock::{Mutex, MutexLockFuture};

use crate::db::QueryError;

use self::{
    accounts::Accounts, frontend_accounts::FrontendAccounts, highway_resources::HighwayResources,
    likes::Likes, missions::Missions, player_profiles::PlayerProfiles, qpid_objects::QpidObjects,
    roads::Roads, wasted_baggages::WastedBaggages,
};

pub struct Database {
    database_url: String,
}

impl Database {
    pub fn new(database_url: &str) -> Self {
        Self {
            database_url: database_url.into(),
        }
    }

    pub async fn connect(&self) -> ConnectionResult<DatabaseConnection> {
        let conn = AsyncPgConnection::establish(&self.database_url).await?;
        Ok(DatabaseConnection::new(self, conn))
    }
}

pub struct DatabaseConnection<'db> {
    connection: Arc<Mutex<AsyncPgConnection>>,

    /// Technically not used for anything, but it's borrowed so we can't accidentally leave a connection somewhere past the database struct's life
    _db: &'db Database,
}

impl<'db> DatabaseConnection<'db> {
    pub fn new(db: &'db Database, connection: AsyncPgConnection) -> Self {
        Self {
            _db: db,
            connection: Arc::new(Mutex::new(connection)),
        }
    }

    pub async fn transaction<'a, T, F>(&self, callback: F) -> Result<T, QueryError>
    where
        F: for<'b> FnOnce(&'b Self) -> ScopedBoxFuture<'a, 'b, Result<T, QueryError>> + Send + 'a,
        T: 'a,
    {
        let mut conn_guard = self.get_pg_connection().await;
        AnsiTransactionManager::begin_transaction(&mut *conn_guard).await?;
        std::mem::drop(conn_guard); // release mutex lock to avoid deadlocks from callback

        match callback(self).await {
            Ok(result) => {
                let conn = &mut *self.get_pg_connection().await;
                AnsiTransactionManager::commit_transaction(conn).await?;

                Ok(result)
            }
            Err(user_err) => {
                let conn = &mut *self.get_pg_connection().await;
                match AnsiTransactionManager::rollback_transaction(conn).await {
                    Ok(_) => Err(user_err),
                    Err(DieselError::BrokenTransactionManager) => Err(user_err),
                    Err(err) => Err(err.into()),
                }
            }
        }
    }

    fn get_pg_connection(&self) -> MutexLockFuture<AsyncPgConnection> {
        self.connection.lock()
    }

    pub fn accounts(&self) -> Accounts {
        Accounts::new(self)
    }

    pub fn frontend_accounts(&self) -> FrontendAccounts {
        FrontendAccounts::new(self)
    }

    pub fn player_profiles(&self) -> PlayerProfiles {
        PlayerProfiles::new(self)
    }

    pub fn qpid_objects(&self) -> QpidObjects {
        QpidObjects::new(self)
    }

    pub fn missions(&self) -> Missions {
        Missions::new(self)
    }

    pub fn likes(&self) -> Likes {
        Likes::new(self)
    }

    pub fn highway_resources(&self) -> HighwayResources {
        HighwayResources::new(self)
    }

    pub fn wasted_baggages(&self) -> WastedBaggages {
        WastedBaggages::new(self)
    }

    pub fn roads(&self) -> Roads {
        Roads::new(self)
    }
}
