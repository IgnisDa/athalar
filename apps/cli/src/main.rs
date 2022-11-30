use athalar_cli::{
    app::{App, Commands},
    run,
};
use clap::Parser;
use std::env;

fn main() -> anyhow::Result<()> {
    let cli = App::parse();
    match cli.command {
        Commands::Generate { path } => {
            let path = path
                .unwrap_or_else(|| env::current_dir().expect("Unable to get current directory"));
            run(path)?;
        }
    };
    Ok(())
}
