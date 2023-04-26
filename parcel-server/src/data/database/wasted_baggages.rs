use super::DatabaseConnection;

pub struct WastedBaggages<'db> {
    connection: &'db DatabaseConnection<'db>,
}

impl<'db> WastedBaggages<'db> {
    pub fn new(connection: &'db DatabaseConnection) -> Self {
        Self { connection }
    }
}
