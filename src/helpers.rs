use toml::Value;

pub fn read_toml() -> Value {
    let toml: String = std::fs::read_to_string("Pyre.toml").expect("Unable to read pyre.toml");

    let toml = match toml.parse::<toml::Value>() {
        Ok(toml) => toml,
        Err(e) => panic!("Could not parse the Pyre.toml file: {}", e),
    };

    let data = match toml.get("data") {
        Some(data) => data,
        None => panic!("Could not find the data section in Pyre.toml"),
    };

    data.clone()
}


