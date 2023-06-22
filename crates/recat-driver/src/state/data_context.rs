use std::sync::Arc;

use recat_adapter_persist::{pool::DbPool, repos::*};
use recat_core::repos::*;

use crate::config::RecatArgs;

pub(crate) type RecatDataContext = DataContextImpl<ProductsRepoImpl>;

pub(crate) struct DataContextImpl<Products> {
    products: Products,
}

impl<Products> recat_app::state::DataContext for DataContextImpl<Products>
where
    Products: ProductsRepo + Sync,
{
    type Products = Products;

    fn products(&self) -> &Self::Products {
        &self.products
    }
}

impl RecatDataContext {
    pub(super) fn new(config: &RecatArgs) -> anyhow::Result<Self> {
        let pool = Arc::new(DbPool::new(&config.data)?);

        Ok(Self {
            products: ProductsRepoImpl::new(pool),
        })
    }
}
