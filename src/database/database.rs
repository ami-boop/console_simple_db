use std::collections::HashMap;

use super::{
    config::DatabaseConfig,
    connection::DatabaseConnection,
    error::DatabaseError,
    table::Table,
};

#[derive(Debug)]
pub(crate) struct Database {
    name: String,
    tables: HashMap<String, Table>,
    config: DatabaseConfig,
    connection: DatabaseConnection,
}

impl Database {
    pub(crate) fn new(
        name: &str,
        max_tables: usize,
        read_only: bool,
        pool_size: usize,
    ) -> Self {
        Self {
            name: name.to_string(),
            tables: HashMap::new(),
            config: DatabaseConfig::new(max_tables, read_only),
            connection: DatabaseConnection::new(pool_size),
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn create_table(&mut self, name: String) -> Result<(), DatabaseError> {
        self.connection.check_connection()?;

        if self.config.read_only || self.tables.len() >= self.config.max_tables {
            return Err(DatabaseError::DatabaseConfigError);
        }

        if self.tables.contains_key(&name) {
            return Err(DatabaseError::InvalidData);
        }

        self.tables.insert(name, Table::new());
        Ok(())
    }

    pub(crate) fn remove_table(&mut self, table_key: &str) -> Result<(), DatabaseError> {
        match self.tables.remove(table_key) {
            Some(_) => Ok(()),
            None => Err(DatabaseError::NotFound)
        }
    }

    pub(crate) fn get_table(&self, table_name: &str) -> Option<&Table> {
        self.tables.get(table_name)
    }

    pub(crate) fn get_table_mut(&mut self, table_name: &str) -> Option<&mut Table> {
        self.tables.get_mut(table_name)
    }

    pub(crate) fn table_names(&self) -> Vec<&str> {
        let mut table_names = self.tables.keys().map(String::as_str).collect::<Vec<_>>();
        table_names.sort_unstable();
        table_names
    }

    // pub(crate) fn create_row(&mut self, current_table: &str) -> Result<(), DatabaseError> {
    //     self.connection.check_connection()?;
    //     let table = self
    //         .tables
    //         .get_mut(current_table_name)
    //         .ok_or(DatabaseError::NotFound)?;
    //     table.insert_row();
    //     Ok(())
    // }
}
