use clap::{Parser, Subcommand};
use std::path::PathBuf;

const BIN_NAME: &str = "athalar";

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(name = "generate", about = "Generate the bindings for a project")]
    Generate {
        #[arg(help = "The path where the project is present, defaults to $PWD")]
        path: Option<PathBuf>,
    },
}

#[derive(Debug, Parser)]
#[command(
    bin_name = BIN_NAME,
    name = "athalar",
    about = "Unify your configuration!",
    version,
    disable_colored_help = true,
    disable_help_subcommand = true,
    propagate_version = true,
    next_line_help = false,
    rename_all = "kebab case"
)]
pub struct App {
    #[command(subcommand)]
    pub command: Commands,
}
