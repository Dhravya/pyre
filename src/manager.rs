use dialoguer::{theme::ColorfulTheme, Select};
use dirs::config_dir;
use json;
use std::{fs::write, path::Path};

pub fn set_editor(editor_command: String) {
    // read data.json
    let data_path = config_dir().unwrap().join("pyre.json");
    // if data.json doesn't exist, create it
    if !Path::new(&data_path).exists() {
        let data = json::parse(
            r#"
{
    "projects":{

    },
    "default_open_command": null
}"#,
        )
        .unwrap();
        write(data_path, data.dump()).expect("Unable to write to data.json");
    }

    let data_file = std::fs::read_to_string(config_dir().unwrap().join("pyre.json"))
        .expect("Unable to read pyre.json");

    // parse data.json
    let mut data: json::JsonValue = json::parse(&data_file).unwrap();

    // update default_open_command
    data["default_open_command"] = json::JsonValue::String(editor_command);

    // write data.json
    write(config_dir().unwrap().join("pyre.json"), data.dump())
        .expect("Unable to write to data.json");
}

pub fn get_editor() -> String {
    let data_file = std::fs::read_to_string(config_dir().unwrap().join("pyre.json")).expect(
        "Unable to read pyre.json. You need to run pyre config-editor <editor command> first",
    );

    let data: json::JsonValue = json::parse(&data_file).unwrap();

    match data["default_open_command"].as_str() {
        Some(command) => command.to_string(),
        None => "".to_string(),
    }
}

pub fn get_json_path() -> String {
    let data_path = config_dir().unwrap().join("pyre.json");
    data_path.to_str().unwrap().to_string()
}

pub fn get_project_path(project_name: &str) -> String {
    let data_file = std::fs::read_to_string(config_dir().unwrap().join("pyre.json")).expect(
        "Unable to read pyre.json. You need to run pyre config-editor <editor command> first",
    );

    let data: json::JsonValue = json::parse(&data_file).unwrap();

    match data["projects"][project_name].as_str() {
        Some(path) => path.to_string(),
        None => "".to_string(),
    }
}

pub fn add_project(project_name: &str, project_path: &str) {
    // read data.json
    let data_path = config_dir().unwrap().join("pyre.json");
    println!("{:?}", data_path);
    // if data.json doesn't exist, create it
    if !Path::new(&data_path).exists() {
        let data = json::parse(
            r#"
{
    "projects":{

    },
    "default_open_command": null
}"#,
        )
        .unwrap();
        write(data_path, data.dump()).expect("Unable to write to data.json");
    }

    let data_file = std::fs::read_to_string(config_dir().unwrap().join("pyre.json"))
        .expect("Unable to read pyre.json");

    // parse data.json
    let mut data: json::JsonValue = json::parse(&data_file).unwrap();

    // update projects
    data["projects"][project_name] = json::JsonValue::String(project_path.to_string());

    // write data.json
    write(config_dir().unwrap().join("pyre.json"), data.dump())
        .expect("Unable to write to pyre.json");
}

pub fn project_selector() {
    let data_file = std::fs::read_to_string(config_dir().unwrap().join("pyre.json")).expect(
        "Unable to read pyre.json. You need to run pyre config-editor <editor command> first",
    );

    let data: json::JsonValue = json::parse(&data_file).unwrap();
    let projects = data["projects"].entries();

    // Convert the iterator into a vector
    let mut project_names: Vec<String> = Vec::new();
    for (project_name, _) in projects {
        project_names.push(project_name.to_string());
    }

    let project = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a project")
        .default(0)
        .items(&project_names)
        .interact()
        .unwrap();

    println!("Opening {}", project_names[project]);

    let project_path = get_project_path(&project_names[project]);
    let editor_command = get_editor();

    std::process::Command::new(editor_command)
        .arg(&project_path)
        .spawn()
        .expect("Unable to open project");
}

pub fn project_selector_list() {
    let data_file = std::fs::read_to_string(config_dir().unwrap().join("pyre.json")).expect(
        "Unable to read pyre.json. You need to run pyre config-editor <editor command> first",
    );

    let data: json::JsonValue = json::parse(&data_file).unwrap();
    let projects = data["projects"].entries();

    // Convert the iterator into a vector
    let mut project_names: Vec<String> = Vec::new();
    for (project_name, loc) in projects {
        project_names.push(project_name.to_string() + "   -    " + &loc.to_string());
    }

    let project = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a project")
        .default(0)
        .items(&project_names)
        .interact()
        .unwrap();

    println!("Opening {}", project_names[project]);

    let project_path = get_project_path(&project_names[project]);
    let editor_command = get_editor();

    std::process::Command::new(editor_command)
        .arg(&project_path)
        .spawn()
        .expect("Unable to open project");
}
