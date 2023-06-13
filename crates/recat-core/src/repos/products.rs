use reddd::domain::{ReadRepo, WriteRepo};

use crate::entities::Product;

/// A trait to be implemented by [`Product`]s repositories.
pub trait ProductsRepo: ReadRepo<Entity = Product> + WriteRepo<Entity = Product> {}
