use reddd::domain::{Pagination, UseCase};
use reddd_macros::UseCase;

use crate::{entities::Product, error::AppError};

/// A use case to get a page of [`Product`]s.
#[derive(Debug, UseCase)]
#[usecase(
    input = "Pagination<Product>",
    output = "Vec<Product>",
    error = "AppError"
)]
pub struct GetProductPage<'a> {
    phantom: std::marker::PhantomData<&'a ()>,
}
