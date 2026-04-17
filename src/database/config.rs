#[derive(Debug)]
pub(crate) struct DatabaseConfig {
    pub(crate) max_tables: usize,
    pub(crate) read_only: bool,
}

impl DatabaseConfig {
    pub(crate) fn new(max_tables: usize, read_only: bool) -> Self {
        Self {
            max_tables,
            read_only,
        }
    }
}
