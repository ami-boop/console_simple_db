use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use super::{error::DatabaseError, row::Row};

#[derive(Debug)]
pub(crate) struct Table {
    rows: BTreeMap<usize, Row>,
    next_id: usize,
}

impl Table {
    pub(crate) fn new() -> Self {
        Self {
            rows: BTreeMap::new(),
            next_id: 0,
        }
    }

    pub(crate) fn get_row(&self, key: usize) -> Option<&Row> {
        self.rows.get(&key)
    }

    pub(crate) fn get_value(
        &self,
        row_id: usize,
        field_name: &str,
    ) -> Result<Value, DatabaseError> {
        match self.rows.get(&row_id).and_then(|row| row.get_value(field_name)) {
            Some(value) => Ok(value.clone()),
            None => Err(DatabaseError::NotFound),
        }
    }

    pub(crate) fn insert_row(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        self.rows.insert(id, Row::new(id));
        id
    }

    pub(crate) fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub(crate) fn field_names(&self) -> Vec<String> {
        let mut field_names = BTreeSet::new();

        for row in self.rows.values() {
            for field_name in row.field_names() {
                field_names.insert(field_name.to_string());
            }
        }

        field_names.into_iter().collect()
    }

    pub(crate) fn set_value(
        &mut self,
        row_id: usize,
        field_name: String,
        new_value: Value,
    ) -> Result<(), DatabaseError> {
        let row = self.rows.get_mut(&row_id).ok_or(DatabaseError::NotFound)?;
        row.insert_value(field_name, new_value)
    }
}
