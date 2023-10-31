use diesel::prelude::*;

use super::DatabaseConnection;

pub struct CustomMissions<'db> {
    connection: &'db DatabaseConnection<'db>,
}

impl<'db> CustomMissions<'db> {
    pub fn new(connection: &'db DatabaseConnection) -> Self {
        Self { connection }
    }
}
