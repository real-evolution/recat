mod data;
mod config;

pub use config::AppConfig;
pub use data::DataContext;
use recat_core::error::AppResult;
use reddd::domain::UseCase;

/// A trait to represent the state of the application.
pub trait AppState: Sync {
    type Data: DataContext;
    type Config: AppConfig;

    fn config(&self) -> &Self::Config;

    /// Gets the application data context.
    fn data(&self) -> &Self::Data;
}

#[async_trait::async_trait]
pub trait UseCaseRunner<U: UseCase> {
    /// Asynchronously executes runs a usecase of type [`U`].
    ///
    /// # Parameters
    /// - `input` - The input to the use case.
    ///
    /// # Returns
    /// The output of the use case.
    async fn run_usecase(&self, input: U::Input) -> AppResult<U::Output>;
}

#[async_trait::async_trait]
pub trait UseCaseRunnerExt {
    /// Asynchronously executes runs a usecase of type [`U`].
    ///
    /// # Parameters
    /// - `input` - The input to the use case.
    ///
    /// # Returns
    /// The output of the use case.
    async fn run<U>(&self, input: U::Input) -> AppResult<U::Output>
    where
        U: UseCase,
        U::Input: Send,
        Self: UseCaseRunner<U>;
}

#[async_trait::async_trait]
impl<S: Sync> UseCaseRunnerExt for S {
    async fn run<U>(&self, input: U::Input) -> AppResult<U::Output>
    where
        U: UseCase,
        U::Input: Send,
        Self: UseCaseRunner<U>,
    {
        <Self as UseCaseRunner<U>>::run_usecase(self, input).await
    }
}
