use super::DatabaseConnection;

pub struct Missions<'db> {
    connection: &'db DatabaseConnection<'db>,
}

impl<'db> Missions<'db> {
    pub fn new(connection: &'db DatabaseConnection) -> Self {
        Self { connection }
    }
}
