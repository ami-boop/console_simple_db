use super::error::DatabaseError;

#[derive(Debug)]
pub(crate) struct DatabaseConnection {
    pool_size: usize,
    active_connections: usize,
}

impl DatabaseConnection {
    pub(crate) fn new(pool_size: usize) -> Self {
        Self {
            pool_size,
            active_connections: 1,
        }
    }

    pub(crate) fn check_connection(&self) -> Result<(), DatabaseError> {
        if self.active_connections == 0 {
            Err(DatabaseError::NoConnection)
        } else {
            Ok(())
        }
    }

    pub(crate) fn add_connection(&mut self) {
        if self.active_connections < self.pool_size {
            self.active_connections += 1;
        }
    }

    pub(crate) fn end_connection(&mut self) {
        if self.active_connections > 0 {
            self.active_connections -= 1;
        }
    }
}
