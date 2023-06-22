use chrono::Duration;
use clap::{Parser, ValueEnum};
use recat_adapter_persist::config::DataConfig;

/// The configuration values that are used by the persistence layer.
#[derive(Debug, Parser)]
pub(crate) struct RecatDataConfig {
    /// The driver to use.
    #[arg(
        long = "db-driver",
        env = "RECAT_DB_DRIVER",
        default_value = "postgres"
    )]
    driver: DataDriver,

    /// The address to connect to.
    #[arg(
        long = "db-host",
        env = "RECAT_DB_HOST",
        default_value = "localhost"
    )]
    host: String,

    /// The port to connect to.
    #[arg(
        long = "db-port",
        env = "RECAT_DB_PORT",
        value_parser = clap::value_parser!(u16).range(1..)
    )]
    port: Option<u16>,

    /// The username to connect with.
    #[arg(long = "db-username", env = "RECAT_DB_USERNAME")]
    username: String,

    /// The password to connect with.
    #[arg(long = "db-password", env = "RECAT_DB_PASSWORD")]
    password: String,

    /// The database to connect to.
    #[arg(long = "db-name", env = "RECAT_DB_NAME", default_value = "recat")]
    database: String,

    /// The maximum number of pooled connections.
    #[arg(
        long = "db-pool-size",
        env = "RECAT_DB_POOL_SIZE",
        value_parser = clap::value_parser!(u32).range(1..1024)
    )]
    max_pool_size: Option<u32>,

    /// The timeout for idle pooled connections.
    #[arg(
        long = "db-idle-timeout",
        env = "RECAT_DB_IDLE_TIMEOUT",
        value_parser = super::parse::parse_duration
    )]
    idle_timeout: Option<Duration>,

    /// Whether to lazily create pooled connections.
    #[arg(long = "db-lazy", env = "RECAT_DB_LAZY")]
    lazy: bool,
}

/// An enum representing the supported database drivers.
#[derive(Clone, Debug, ValueEnum)]
enum DataDriver {
    Postgres,
}

impl DataDriver {
    /// Returns the name of the driver as a string.
    fn as_str(&self) -> &str {
        match self {
            | DataDriver::Postgres => "postgres",
        }
    }

    /// Returns the default port for the driver.
    fn default_port(&self) -> u16 {
        match self {
            | DataDriver::Postgres => 5432,
        }
    }
}

impl DataConfig for RecatDataConfig {
    fn driver(&self) -> &str {
        self.driver.as_str()
    }

    fn host(&self) -> &str {
        &self.host
    }

    fn port(&self) -> u16 {
        self.port.unwrap_or_else(|| self.driver.default_port())
    }

    fn username(&self) -> &str {
        &self.username
    }

    fn password(&self) -> &str {
        &self.password
    }

    fn database(&self) -> &str {
        &self.database
    }

    fn pool_max_connections(&self) -> Option<usize> {
        self.max_pool_size.map(|max| max as usize)
    }

    fn pool_timeout(&self) -> Option<Duration> {
        self.idle_timeout
    }

    fn pool_lazy(&self) -> bool {
        self.lazy
    }
}
