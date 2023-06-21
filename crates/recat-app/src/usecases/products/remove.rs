use recat_core::{entities::Product, usecases::products::RemoveProduct};
use reddd::domain::{Entity, UseCase, UseCaseHandler, WriteRepo};

use crate::state::{AppState, DataContext};

#[derive(Debug)]
pub struct RemoveProductHandler;

#[async_trait::async_trait]
impl<S: AppState> UseCaseHandler<RemoveProduct, S> for RemoveProductHandler {
    async fn execute(
        input: <Product as Entity>::Key,
        state: &S,
    ) -> Result<(), <RemoveProduct as UseCase>::Error> {
        Ok(state.data().products().remove(&input).await?)
    }
}
