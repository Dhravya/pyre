use clap::{Parser, Subcommand};
mod commands;

#[derive(Subcommand)]
enum Commands {
    I { name: Option<String> },
    New { name: Option<String> },
}

// Pyre: The cargo for python
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::I { name } => {
            commands::install_packages(name.clone().unwrap());
        }
        Commands::New { name } => {
            commands::create_new_project(name.clone().unwrap());
        }
    }
}
