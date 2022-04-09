use dialoguer::{theme::ColorfulTheme, Select};
use std::{io::Write, process::Command, vec};



pub fn install_packages(packages: String) {
    println!("Preparing : {}", packages);

    // Split the string into a vector of strings
    let packages: Vec<&str> = packages.split(" ").collect();

    // Run pip install package1 package2
    let output = Command::new("pip")
        .arg("install")
        .args(&packages)
        .output()
        .expect("Failed to execute process");

    // Print the output of the command
    println!("{}", String::from_utf8_lossy(&output.stdout));

    // Checks if requirements.txt exists
    if !std::path::Path::new("requirements.txt").exists() {
        println!("No requirements.txt file found, creating");

        // Create a new file
        std::fs::File::create("requirements.txt").expect("Unable to create file");
    }

    // Open the file
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open("requirements.txt")
        .expect("Unable to open file");

    // Write the packages to the file
    for package in packages {
        file.write_all(package.as_bytes())
            .expect("Unable to write to file");
        file.write_all("\n".as_bytes())
            .expect("Unable to write to file");
    }
}

pub fn create_new_project(name: String) {
    println!("Creating new project: {}", name);

    // Create the directory
    std::fs::create_dir(&name).expect("Unable to create directory");

    std::env::set_current_dir(name).expect("Unable to change directory");

    Command::new("python")
        .arg("-m")
        .arg("venv")
        .arg("env")
        .output()
        .expect("Failed to create virtual environment");

    Command::new("git")
        .arg("init")
        .output()
        .expect("Failed to initialize git");

    let mut gitignore_file =
        std::fs::File::create(".gitignore").expect("Failed to create gitignore file");

    let gitignore_content = String::from("env/\n.env");
    gitignore_file
        .write_all(gitignore_content.as_bytes())
        .expect("Failed to write to gitignore file");

    std::fs::File::create("requirements.txt").expect("Failed to create requirements file");

    std::fs::create_dir("src").expect("Failed to create src directory");
    std::fs::File::create("src/main.py").expect("Failed to create main file");

    println!("Successfully initialised python project");

    let ides = vec!["code", "sublime", "code-insiders", "atom", "idea"];

    let ide = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select IDE")
        .items(&ides)
        .interact()
        .unwrap();

    let ide = ides[ide];
    println!("Opening {}", ide);

    match ide {
        "code" => {
            Command::new("code.cmd")
                .arg(".")
                .output()
                .expect("Failed to open code");
        }
        "sublime" => {
            Command::new("subl")
                .arg(".")
                .output()
                .expect("Failed to open sublime");
        }
        "code-insiders" => {
            Command::new("code-insiders.cmd")
                .arg(".")
                .output()
                .expect("Failed to open code-insiders");
        }
        "atom" => {
            Command::new("atom")
                .arg(".")
                .output()
                .expect("Failed to open atom");
        }
        "idea" => {
            Command::new("idea")
                .arg(".")
                .output()
                .expect("Failed to open idea");
        }
        _ => {
            println!("Unable to open IDE");
        }
    }

    println!("All done! If you liked this, please star the repo on github - https://github.com/dhravya/pyre. If you faced any issues, Feel free to raise an issue on github");
}
