use bigdecimal::BigDecimal;
use reddd::domain::{UseCase, Entity};
use reddd_macros::UseCase;

use crate::{
    entities::Product,
    error::AppError,
};

/// A use case to update a [`Product`].
#[derive(Debug, UseCase)]
#[usecase(input = "UpdateProductInput", output = "Product", error = "AppError")]
pub struct UpdateProduct;

#[derive(Debug)]
pub struct UpdateProductInput {
    /// The unique identifier of the product.
    pub id: <Product as Entity>::Key,

    /// The title of the product.
    pub title: Option<String>,

    /// The quantity of the product.
    pub quantity: Option<i32>,

    /// The description of the product.
    pub description: Option<String>,

    /// The unique identifier of the price's token/currency.
    pub price_token_id: Option<uuid::Uuid>,

    /// The amount of the price's token/currency.
    pub price_amount: Option<BigDecimal>,
}
