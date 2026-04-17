use std::collections::HashMap;

use serde_json::Value;

use super::error::DatabaseError;

#[derive(Debug)]
pub(crate) struct Row {
    id: usize,
    data: HashMap<String, Value>,
}

impl Row {
    pub(crate) fn new(id: usize) -> Self {
        Self {
            id,
            data: HashMap::new(),
        }
    }

    pub(crate) fn get_value(&self, field_name: &str) -> Option<&Value> {
        self.data.get(field_name)
    }

    pub(crate) fn field_names(&self) -> impl Iterator<Item = &str> {
        self.data.keys().map(String::as_str)
    }

    pub(crate) fn insert_value(
        &mut self,
        field_name: String,
        new_value: Value,
    ) -> Result<(), DatabaseError> {
        self.data.insert(field_name, new_value);
        Ok(())
    }
}
