use clap::{Parser, Subcommand};
mod commands;
mod helpers;

#[derive(Subcommand)]
enum Commands {
    I { packages: Option<String> },
    Install { packages: Option<String> },
    Uninstall { package: String },
    New { name: Option<String> },
    Run,
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
        Commands::I { packages } => {
            commands::install_packages(packages.clone().unwrap());
        }
        Commands::New { name } => {
            commands::create_new_project(name.clone().unwrap());
        }
        Commands::Install { packages } => commands::install_packages(packages.clone().unwrap()),
        Commands::Run => {
            println!("Currently run command is not properly supported");
            let data = helpers::read_toml();

            let executable = data["executable"].as_str().unwrap();
            let main_file = data["main_file"].as_str().unwrap();

            // Run the python script in current shell
            let out = std::process::Command::new(executable)
                .arg(main_file)
                .output()
                .expect("Failed to execute process");

            println!("{}", String::from_utf8_lossy(&out.stdout));
        }
        Commands::Uninstall { package } => {
            std::process::Command::new("pip")
                .arg("uninstall")
                .arg("-y")
                .arg(package)
                .output()
                .expect("Failed to execute process");
        },
    }
}
