use postgres::error::Error as DbError;
#[derive(Debug)]
pub enum Error {
    Postgres(DbError),
    NotFound(String),
}

impl From<DbError> for Error {
    fn from(err: DbError) -> Self {
        Error::Postgres(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Postgres(err) => write!(f, "PostgreSQL error: {}", err),
            Error::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl std::error::Error for Error {}
