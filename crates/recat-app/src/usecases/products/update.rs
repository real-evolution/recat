use chrono::Utc;
use recat_core::{
    entities::Product,
    usecases::products::{UpdateProduct, UpdateProductInput},
};
use reddd::domain::{ReadRepo, UseCase, UseCaseHandler, WriteRepo};

use crate::state::{AppState, DataContext};

#[derive(Debug)]
pub struct UpdateProductHandler;

#[async_trait::async_trait]
impl<S: AppState> UseCaseHandler<UpdateProduct, S> for UpdateProductHandler {
    async fn execute(
        input: UpdateProductInput,
        state: &S,
    ) -> Result<Product, <UpdateProduct as UseCase>::Error> {
        let old = state.data().products().get(&input.id).await?;
        let product = Product {
            id: old.id,
            owner_id: old.owner_id,
            price_token_id: old.price_token_id,
            created_at: old.created_at,
            updated_at: Utc::now(),
            title: match input.title {
                | Some(i) => i,
                | None => old.title,
            },
            quantity: match input.quantity {
                | Some(i) => i,
                | None => old.quantity,
            },
            price_amount: match input.price_amount {
                | Some(i) => i,
                | None => old.price_amount,
            },
            description: match input.description {
                | Some(i) => Some(i),
                | None => old.description,
            },
        };
        Ok(state.data().products().update(product).await?)
    }
}
