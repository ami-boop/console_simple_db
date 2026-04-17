#[derive(Debug)]
pub(crate) enum DatabaseError {
    DatabaseConfigError,
    NotFound,
    NoConnection,
    InvalidData,
}
