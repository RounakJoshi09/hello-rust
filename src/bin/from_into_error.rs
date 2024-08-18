use thiserror::Error;

#[derive(Debug, Error)]
enum NetworkError {
    #[error("connection timed out")]
    Timeout,
}

#[derive(Debug, Error)]
enum DatabaseError {
    #[error("Conncetion Lost with the Database")]
    ConnectionLost,
}

#[derive(Debug, Error)]
enum APIError {
    #[error("Network Error , {0}")]
    Network(#[from] NetworkError),
    #[error("DB Error , {0}")]
    Database(#[from] DatabaseError),
}

// impl From<NetworkError> for APIError {
//     fn from(value: NetworkError) -> Self {
//         return Self::Network(value);
//     }
// }

// impl From<DatabaseError> for APIError {
//     fn from(value: DatabaseError) -> Self {
//         return Self::Database(value);
//     }
// }

fn do_stuff() -> Result<(), APIError> {
    // Err(NetworkError::Timeout)?
    Err(DatabaseError::ConnectionLost)?
}

fn main() {}
