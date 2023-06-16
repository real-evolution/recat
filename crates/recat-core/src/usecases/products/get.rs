use reddd::domain::{Entity, UseCase};
use reddd_macros::UseCase;

use crate::{entities::Product, error::AppError};

/// A use case to get a [`Product`] by its id.
#[derive(Debug, UseCase)]
#[usecase(
    input = "<Product as Entity>::Key",
    output = "Product",
    error = "AppError"
)]
pub struct GetProduct;
