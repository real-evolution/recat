use std::sync::Arc;

use derive_new::new;
use recat_core::{entities::Product, repos::ProductsRepo};

#[derive(new)]
pub struct ProductsRepoImpl(Arc<crate::pool::DbPool>);

#[async_trait::async_trait]
impl ProductsRepo for ProductsRepoImpl {}

crate::impl_full_repo! {
    Product => ProductsRepoImpl;
    crate::models::ProductModel => products
}
