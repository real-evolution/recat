use reddd::domain::error::RepoError;

pub type AppResult<T> = Result<T, AppError>;

/// An enum of possilbe errors that can occur in the application.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// An unknown error occurred that does not belong to any other error type.
    #[error("unknown error: {0}")]
    Unknown(#[from] anyhow::Error),

    /// An error occured during data manipulation operations.
    #[error("data error: {0}")]
    Data(#[from] RepoError),
}
