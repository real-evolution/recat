use reddd::domain::{Entity, UseCase};
use reddd_macros::UseCase;

use crate::{
    entities::{Principle, Product, Token},
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
    pub owner_id: <Principle as Entity>::Key,

    /// The unique identifier of the price's token/currency.
    pub price_token_id: <Token as Entity>::Key,

    /// The amount of the price's token/currency.
    pub price_amount: i32,
}
