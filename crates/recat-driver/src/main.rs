use clap::Parser;

mod config;
mod state;

fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let args = config::RecatArgs::parse();

    println!("{args:#?}");

    let _state = self::state::RecatAppState::new(args)?;

    Ok(())
}
