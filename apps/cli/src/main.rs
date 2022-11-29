mod app;

use crate::app::App;
use app::Commands;
use athalar_py::run as python_run;
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
                    python_run(&path);
                }
            }
        }
    }
}
