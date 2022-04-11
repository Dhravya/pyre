use clap::{Parser, Subcommand};
mod commands;
mod helpers;
mod manager;

#[derive(Subcommand)]
enum Commands {
    /// Install Packages - pass packages seperated by spaces
    I {
        packages: Option<String>,
    },
    Install {
        packages: Option<String>,
    },
    /// Uninstalls a package, well, runs pip uninstall
    Uninstall {
        package: String,
    },
    /// Creates a new python project
    New {
        name: Option<String>,
    },
    /// Runs the python script. To configure this, make a Pyre.toml file and add the script to the data section
    Run,
    /// Configuration the open editor command
    ConfigEditor {
        editor_command: Option<String>,
    },
    /// Opens the project specified by the project name. Run without arguments to see all projects and select
    Open {
        project_name: Option<String>,
    },
    /// Lists all the projects
    List,

    /// Adds a project to the list of projects
    Add {
        project_name: String,
        project_path: String,
    },
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
        }
        Commands::ConfigEditor { editor_command } => {
            manager::set_editor(editor_command.clone().unwrap());
        }
        Commands::Open { project_name } => {
            if project_name.is_none() {
                manager::project_selector();
            } else {
                let editor = manager::get_editor();
                let project_path = manager::get_project_path(&project_name.clone().unwrap());

                std::process::Command::new(editor)
                    .arg(project_path)
                    .output()
                    .expect("Failed to execute process");
            }
        }
        Commands::List => {
            manager::project_selector_list();
        }
        Commands::Add { project_name, project_path } => {
            manager::add_project(project_name, project_path);
        }
    }
}
