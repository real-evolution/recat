pub type AppResult<T> = Result<T, AppError>;

#[derive(Clone, Debug)]
pub enum AppError {}
