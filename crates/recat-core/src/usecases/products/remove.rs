use reddd::domain::{Entity, UseCase};
use reddd_macros::UseCase;

use crate::{entities::Product, error::AppError};

/// A use case to remove a [`Product`].
#[derive(Debug, UseCase)]
#[usecase(
    input = "<Product as Entity>::Key",
    output = "()",
    error = "AppError"
)]
pub struct RemoveProduct;
