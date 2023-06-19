use recat_core::repos;

/// A trait to abstract data repositories' implementations.
pub trait DataContext: Sync {
    type Products: repos::ProductsRepo;

    /// Gets the principals repository.
    fn products(&self) -> &Self::Products;
}
