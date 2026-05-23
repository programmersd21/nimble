mod commands;
mod config;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "anvil", version, about = "Nimble project tooling & package manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        path: PathBuf,
        #[arg(long)]
        name: Option<String>,
    },
    Build {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short = 'r', long)]
        run: bool,
        #[arg(short = 'c', long)]
        clean: bool,
    },
    Run {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Init { path, name } => {
            let project_name = name
                .clone()
                .unwrap_or_else(|| path.file_name().unwrap_or_default().to_string_lossy().to_string());
            commands::init_project(path, &project_name)
        }
        Commands::Build { path, run, clean } => commands::build_project(path, *run, *clean),
        Commands::Run { path } => commands::run_project(path),
    };

    if let Err(e) = result {
        eprintln!("anvil: error: {}", e);
        std::process::exit(1);
    }
}
