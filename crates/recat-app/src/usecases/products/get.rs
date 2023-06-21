use recat_core::{entities::Product, usecases::products::GetProduct};
use reddd::domain::{Entity, ReadRepo, UseCase, UseCaseHandler};

use crate::state::{AppState, DataContext};

#[derive(Debug)]
pub struct GetProductHandler;

#[async_trait::async_trait]
impl<S: AppState> UseCaseHandler<GetProduct, S> for GetProductHandler {
    async fn execute(
        input: <Product as Entity>::Key,
        state: &S,
    ) -> Result<Product, <GetProduct as UseCase>::Error> {
        Ok(state.data().products().get(&input).await?)
    }
}
