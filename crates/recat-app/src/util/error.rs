use reddd::domain::error::{RepoError, RepoResult};

pub(crate) trait RepoResultExt<T> {
    fn optional(self) -> RepoResult<Option<T>>;
}

impl<T> RepoResultExt<T> for RepoResult<T> {
    fn optional(self) -> RepoResult<Option<T>> {
        if matches!(self, Err(RepoError::NotFound(_))) {
            return Ok(None);
        }

        self.map(Some)
    }
}
