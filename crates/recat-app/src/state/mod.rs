mod data;

pub use data::DataContext;

/// A trait to represent the state of the application.
pub trait AppState: Sync {
    type Data: DataContext;

    /// Gets the application data context.
    fn data(&self) -> &Self::Data;
}
