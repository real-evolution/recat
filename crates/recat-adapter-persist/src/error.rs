use diesel::result::{DatabaseErrorKind, Error};
use reddd::domain::error::{RepoError, RepoResult};

pub(crate) trait DieselResultExt<T> {
    fn into_repo_result(self) -> RepoResult<T>;
}

impl<T> DieselResultExt<T> for Result<T, Error> {
    fn into_repo_result(self) -> RepoResult<T> {
        let err = match self {
            Ok(v) => return Ok(v),
            Err(err) => err,
        };

        Err(match err {
            Error::InvalidCString(err) => RepoError::InvalidParameter(err.to_string()),
            Error::NotFound => RepoError::NotFound("no such item".to_owned()),

            Error::DatabaseError(kind, _) => match kind {
                DatabaseErrorKind::UniqueViolation => {
                    RepoError::DuplicateValue("duplicate identifier".to_owned())
                }
                DatabaseErrorKind::ForeignKeyViolation => {
                    RepoError::InvalidParameter("invalid reference".to_owned())
                }
                DatabaseErrorKind::UnableToSendCommand => RepoError::Io(std::io::Error::new(
                    std::io::ErrorKind::BrokenPipe,
                    "could not send database command",
                )),
                DatabaseErrorKind::SerializationFailure => {
                    RepoError::InvalidParameter("could not serialize data".to_owned())
                }
                DatabaseErrorKind::ReadOnlyTransaction => RepoError::Io(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    "read-only transaction",
                )),
                DatabaseErrorKind::NotNullViolation => {
                    RepoError::InvalidParameter("missing required value".to_owned())
                }
                DatabaseErrorKind::CheckViolation => {
                    RepoError::InvalidParameter("violated check constraint".to_owned())
                }
                DatabaseErrorKind::ClosedConnection => RepoError::Io(std::io::Error::new(
                    std::io::ErrorKind::ConnectionReset,
                    "connection to database was closed",
                )),
                _ => RepoError::Io(std::io::Error::new(
                    std::io::ErrorKind::Unsupported,
                    "unknown database error",
                )),
            },
            Error::QueryBuilderError(err)
            | Error::DeserializationError(err)
            | Error::SerializationError(err) => RepoError::Other(err),

            Error::RollbackErrorOnCommit {
                rollback_error: _,
                commit_error: _,
            }
            | Error::RollbackTransaction
            | Error::AlreadyInTransaction
            | Error::NotInTransaction
            | Error::BrokenTransactionManager => RepoError::Io(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "transaction error",
            )),
            _ => RepoError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "unknown error",
            )),
        })
    }
}
