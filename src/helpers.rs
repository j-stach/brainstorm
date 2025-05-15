
use std::path::Path;
use std::net::IpAddr;

use animusd_lib::{
    protocol::{ AnimusAction, AnimusCommand },
    error::{ CommandError, ConfigError }
};


// Check if an animus is currently active by pinging for its version number.
pub(crate) fn animus_is_active(animus_name: &str) -> Result<bool, CommandError> {

    if !valid_animus_name(animus_name) {
        return Err(CommandError::BadCommandSyntax(animus_name.to_string()))
    }

    match send_animus_command(animus_name, AnimusAction::Version)? {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

// Send command to associated IP address @ port 4048.
// Get any animus response and parse results to string.
// Returns an error if the network connection could not be established.
pub(crate) fn send_animus_command(
    animus_name: &str, 
    action: AnimusAction 
) -> Result<Option<String>, CommandError> {

    // Create socket to connect to the animus's host device
    let ip_addr = read_animus_config(animus_name, "ip")?
        .parse::<IpAddr>()?;

    let command = AnimusCommand::new(animus_name, action);
    command.send_command(ip_addr)
}

// Unpack the response received by send_animus_command and print it.
pub(crate) fn handle_animus_response(
    animus_name: &str, 
    response: Result<Option<String>, CommandError>
) {
    if let Err(e) = response {
        report_command_error(e);
    } else {
        // Unwrap is safe because we checked for errors above.
        if let Some(value) = response.unwrap() {
            println!("{}", value);
        } else {
            println!("No response from animus '{}'", animus_name);
        }
    }
}

// Log and display an error that occurred while sending an animus command.
pub(crate) fn report_command_error(e: CommandError) {
    println!("WARN: An error occurred: Command was not sent properly.");
    eprintln!("{}", e);
}

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

// Check if data exists for an animus with the given name.
// Expects that the filesystem is correctly configured and readable.
pub(crate) fn animus_exists(animus_name: &str) -> bool {
    let mut exists = false;
    for animus in read_animi() {
        let animus = animus
            .expect("Access animus metadata. If you are seeing this message, your `animi` directory contains an unrecognized filestructure or you lack permission to access it.");
        let name = animus.file_name().into_string()
            .expect("Animus name must be a valid string. If you are seeing this message, your `animi` directory contains an unrecognized filestructure or you lack permission to access it.");
        if &name == animus_name {
            exists = true;
        }
    }
    exists
}

// Check if a network binary with the given name exists in the `saved`` folder.
// Expects that the filesystem is correctly configured and readable.
pub(crate) fn network_exists(network_name: &str) -> bool {
    let mut exists = false;
    for saved in read_saved() {
        let saved = saved
            .expect("Access network metadata. If you are seeing this message, your `saved` directory contains an unrecognized filestructure or you lack permission to access it.");
        let name = saved.file_name().into_string()
            .expect("Network name must be a valid string. If you are seeing this message, your `saved` directory contains an unrecognized filestructure or you lack permission to access it.");
        if &name == network_name {
            exists = true;
        }
    }
    exists
}

// Check if a proposed animus name fits the formatting requirements. 
// (a-Z, 0-9, and underscores)
pub(crate) fn valid_animus_name(name: &str) -> bool {
    // Should unwrap a valid regular expression.
    let valid_name = regex::Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
    valid_name.is_match(name)
}

// Read the animus directory.
// Expects that the filesystem is correctly configured and readable.
pub(crate) fn read_animi() -> std::fs::ReadDir {
    std::fs::read_dir("~/.brainstorm/animi")
        .expect("Framework directory must be set up. Restart brainstorm.")
}

// Read the saved networks directory.
// Expects that the filesystem is correctly configured and readable.
pub(crate) fn read_saved() -> std::fs::ReadDir {
    std::fs::read_dir("~/.brainstorm/saved")
        .expect("Framework directory must be set up. Restart brainstorm.")
}

// Create a string representing the path to an animus's dedicated directory.
pub(crate) fn animus_dir(animus_name: &str) -> String {
    format!("~/.brainstorm/animi/{}", animus_name)
}

