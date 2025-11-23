

// TODO Rewrite config process using `ezcfg` 
/*
// Read the `lib.features` field of `config.toml` into a string.
pub(crate) fn read_animus_features(
    animus_name: &str
) -> Result<String, ConfigError> {

    let mut features_string = String::new();

    let animus_config = read_config_file(animus_name)?;
    if let Some(animus_lib) = animus_config.get("lib") {
        if let Some(lib_features) = animus_lib.get("features") {
            let features_array = lib_features.as_array()
                .ok_or(ConfigError::WrongType("features".to_string()))?;

            features_string = features_array.iter()
                .map(|v: &toml::Value| format!("{}", v))
                // TBD: Filter invalid features with a warning?
                .collect::<Vec<_>>()
                .join(" ")
                .to_string();
        }
    }

    Ok(features_string)
}

// Read a value from the core animus config into a string.
pub(crate) fn read_animus_config(
    animus_name: &str, 
    field: &str
) -> Result<String, ConfigError> {

    let animus_config = read_config_file(animus_name)?;
    
    let animus_values = animus_config.get("animus")
        .ok_or(ConfigError::MissingSection("animus".to_string()))?;
    let value = animus_values.get(field)
        .ok_or(ConfigError::MissingField(field.to_string()))?;

    Ok(format!("{}", value))
}

// Find the `config.toml` file for an animus and read it into data.
pub(crate) fn read_config_file(
    animus_name: &str
) -> Result<toml::Value, ConfigError> {

    let path_string = format!(
        "~/.brainstorm/{}/config.toml", 
        animus_name
    );
    let config_path = Path::new(&path_string);

    // Convert config file into Toml object:
    let config_file = std::fs::read_to_string(config_path)?;
    let animus_config: toml::Value = config_file.parse()?;

    Ok(animus_config)
}
*/

