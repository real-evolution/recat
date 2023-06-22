use recat_core::{entities::Product, usecases::products::GetProductPage};
use reddd::domain::{Pagination, ReadRepo, UseCase, UseCaseHandler};

use crate::state::{AppState, DataContext};

#[derive(Debug)]
pub struct GetProductPageHandler;

#[async_trait::async_trait]
impl<S: AppState> UseCaseHandler<GetProductPage, S>
    for GetProductPageHandler
{
    async fn execute(
        input: Pagination<Product>,
        state: &S,
    ) -> Result<Vec<Product>, <GetProductPage as UseCase>::Error> {
        Ok(state.data().products().get_page(input).await?)
    }
}
