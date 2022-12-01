use athalar_cli::{
    app::{App, Commands},
    run,
};
use clap::Parser;
use log::info;
use std::env;

fn main() -> anyhow::Result<()> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
    let cli = App::parse();
    match cli.command {
        Commands::Generate { path } => {
            let path = path.unwrap_or_else(|| {
                info!("No path provided, using current directory...");
                env::current_dir().expect("Unable to get current directory")
            });
            run(path)?;
        }
    };
    Ok(())
}
