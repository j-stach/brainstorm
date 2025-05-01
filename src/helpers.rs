
use std::path::Path;
use std::io::{Read, Write};
use std::net::{TcpStream, SocketAddr};
use std::time::Duration;


// Check if an animus is currently active by pinging for its version number.
pub(crate) fn animus_is_active(animus_name: &str) -> anyhow::Result<bool> {

    if !valid_animus_name(animus_name) {
        return Err(anyhow::anyhow!(
            "{} is an invalid animus name. Use only a-Z, 0-9, and underscores.", 
            animus_name
        ))
    }

    match send_animus_command(animus_name, "version")? {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

// Send command to associated IP address @ port 4048.
// Get any animus response and parse results to string.
// Returns an error if the network connection could not be established.
pub(crate) fn send_animus_command(
    animus_name: &str, 
    command: &str
) -> anyhow::Result<Option<String>> {

    // Create socket to connect to the animus's host device
    let ip_addr = read_animus_config(animus_name, "ip")?;
    let socket_addr = format!("{}:4048", ip_addr).parse::<SocketAddr>()?;
    
    let mut stream = TcpStream::connect_timeout(
        &socket_addr, 
        Duration::from_secs(5)
    )?;

    stream.set_read_timeout(Some(Duration::from_secs(5)))?;

    // Send animus command
    let message = format!("{}:{}", animus_name, command);
    stream.write_all(message.as_bytes())?;
    stream.flush()?;
    
    // Read response
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer);

    if let Some(len) = bytes_read.ok() {
        let response = String::from_utf8_lossy(&buffer[..len]).to_string();
        Ok(Some(response))
    } else {
        // Timeout on read means animus is inactive.
        // TBD: Try again?
        Ok(None)
    }
}

// Unpack the response received by send_animus_command and print it.
pub(crate) fn handle_animus_response(
    animus_name: &str, 
    response: anyhow::Result<Option<String>>
) {
    if let Err(e) = response {
        report_command_error("name", e);
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
pub(crate) fn report_command_error(command: &str, e: anyhow::Error) {
    println!(
        "WARN: An error occurred: Command {} may not have been sent properly.",
        command
    );
    eprintln!("{}", e);
}

// Read the `lib.features` field of `config.toml` into a string.
pub(crate) fn read_animus_features(animus_name: &str) -> anyhow::Result<String> {

    let mut features_string = String::new();

    let animus_config = read_config_file(animus_name)?;
    if let Some(animus_lib) = animus_config.get("lib") {
        if let Some(lib_features) = animus_lib.get("features") {
            let features_array = lib_features.as_array()
                // TODO: Configuration syntax error-type
                .ok_or(anyhow::anyhow!("`features` field must be array"))?;

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
) -> anyhow::Result<String> {

    let animus_config = read_config_file(animus_name)?;
    
    let animus_values = animus_config.get("animus")
        // TODO: Configuration syntax error-type
        .ok_or(anyhow::anyhow!("`config.toml` must include an [animus] entry."))?;
    let value = animus_values.get(field)
        // TODO: Configuration syntax error-type
        .ok_or(anyhow::anyhow!("`config.toml` is missing field '{}'.", field))?;

    Ok(format!("{}", value))
}

// Find the `config.toml` file for an animus and read it into a toml datastructure.
pub(crate) fn read_config_file(animus_name: &str) -> anyhow::Result<toml::Value> {

    let path_string = format!(
        "~/.brainstorm/{}/config.toml", 
        animus_name
    );
    let config_path = Path::new(&path_string);

    // Convert config file into Toml object:
    // TODO: Configuration syntax error-type
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

