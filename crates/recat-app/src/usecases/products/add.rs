use chrono::Utc;
use recat_core::{
    entities::Product,
    usecases::products::{AddProduct, AddProductInput},
};
use reddd::domain::{Key, UseCase, UseCaseHandler, WriteRepo};

use crate::state::{AppState, DataContext};

#[derive(Debug)]
pub struct AddProductHandler;

#[async_trait::async_trait]
impl<S: AppState> UseCaseHandler<AddProduct, S> for AddProductHandler {
    #[inline]
    async fn execute(
        input: AddProductInput,
        state: &S,
    ) -> Result<Product, <AddProduct as UseCase>::Error> {
        let product = state
            .data()
            .products()
            .add(Product {
                id: Key::new(uuid::Uuid::now_v7()),
                title: input.title,
                quantity: input.quantity,
                owner_id: input.owner_id,
                price_token_id: input.price_token_id,
                description: input.description,
                price_amount: input.price_amount,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
            .await?;
        Ok(product)
    }
}
