use chrono::Duration;

/// A trait to represent the configuration of the database connection.
pub trait DataConfig {
    /// Gets the database driver identifier.
    fn driver(&self) -> &str;

    /// Gets the database server host address.
    fn host(&self) -> &str;

    /// Gets the database server port.
    fn port(&self) -> u16;

    /// Gets the database server username.
    fn username(&self) -> &str;

    /// Gets the database server password.
    fn password(&self) -> &str;

    /// Gets the database name.
    fn database(&self) -> &str;

    /// Gets the maximum number of connections in the connection pool.
    fn pool_max_connections(&self) -> Option<usize>;

    /// Gets the maximum amount of time a connection can be idle before it is
    /// closed.
    fn pool_timeout(&self) -> Option<Duration>;

    /// Gets whether the connection pool connections creation should be lazy.
    fn pool_lazy(&self) -> bool;
}

/// A trait to extend the [`DataConfig`] trait.
pub(super) trait DataConfigExt {
    /// Gets the connection string for the database.
    fn connection_string(&self) -> String;

    /// Gets the connection string for the database with the password concealed.
    fn concealed_connection_string(&self) -> String;
}

impl<T> DataConfigExt for T
where
    T: DataConfig,
{
    fn connection_string(&self) -> String {
        format!(
            "{}://{}:{}@{}:{}/{}",
            self.driver(),
            self.username(),
            self.password(),
            self.host(),
            self.port(),
            self.database(),
        )
    }

    fn concealed_connection_string(&self) -> String {
        format!(
            "{}://{}:***@{}:{}/{}",
            self.driver(),
            self.username(),
            self.host(),
            self.port(),
            self.database(),
        )
    }
}
