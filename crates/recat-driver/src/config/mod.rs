use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct RecatArgs {
    /// The path to the configuration file.
    #[arg(short, long, env = "RECAT_CONFIG", value_name = "FILE")]
    config: Option<PathBuf>,

    /// The format of the configuration file.
    #[arg(long, env = "RECAT_CONFIG_FORMAT", default_value = "toml")]
    config_format: ConfigFormat,
}

/// An enum representing the supported configuration file formats.
#[derive(Clone, Debug, ValueEnum)]
pub(crate) enum ConfigFormat {
    Toml,
    Json,
    Yaml,
}
