mod app;
mod python;

use crate::app::App;
use app::Commands;
use clap::Parser;

fn main() {
    let cli = App::parse();
    match cli.command {
        Commands::Generate { language, path } => {
            let path = path.unwrap_or_else(|| {
                std::env::current_dir().expect("Unable to get current directory")
            });
            match language {
                app::Language::Python => {
                    python::run(&path);
                }
            }
        }
    }
}
