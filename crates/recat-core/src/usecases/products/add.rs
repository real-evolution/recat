use reddd::domain::UseCase;
use reddd_macros::UseCase;
use bigdecimal::BigDecimal;

use crate::{
    entities::Product,
    error::AppError,
};

/// A use case to add a [`Product`].
#[derive(Debug, UseCase)]
#[usecase(input = "AddProductInput", output = "Product", error = "AppError")]
pub struct AddProduct;

#[derive(Debug)]
pub struct AddProductInput {
    /// The title of the product.
    pub title: String,

    /// The quantity of the product.
    pub quantity: u32,

    /// The description of the product.
    pub description: Option<String>,

    /// The unique identifier of the owner of the product.
    pub owner_id: uuid::Uuid,

    /// The unique identifier of the price's token/currency.
    pub price_token_id: uuid::Uuid,

    /// The amount of the price's token/currency.
    pub price_amount: BigDecimal,
}
