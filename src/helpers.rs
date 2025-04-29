
use std::path::{ Path, PathBuf };
use std::process::Command;

// NOTE: Beware injection
pub(crate) fn animus_is_active(animus_name: &str) -> bool {

    match send_animus_command(animus_name, "version") {
        Some(_) => true,
        None => false,
    }
}

//
pub(crate) fn send_animus_command(
    animus_name: &str, 
    command: &str
) -> Option<()> {

    let ip_addr = read_animus_config(animus_name, "ip");

    // Send command to associated IP addr @ port 4048
    // Get animus response and parse results

    // TODO: 
    // If there is a response before the timeout,
    // say yes,
    // otherwise ping ip address to check connection

    todo!("Return response or None")
}

//
pub(crate) fn read_animus_features(animus_name: &str) -> String {

    let mut features = String::new();

    let animus_config = read_config_file(animus_name);
    let animus_lib = animus_config.get("lib")
        .expect("Find [animus] section");

    let lib_features = animus_lib.get("features")
        .expect("Find animus features");

    let features_string = lib_features.as_array()
        .expect("Retrieve `features` array")
        .iter()
        .map(|v: &toml::Value| format!("{}", v))
        .collect::<Vec<_>>()
        .join(" ")
        .to_string();

    features_string
}

//
pub(crate) fn read_animus_config(animus_name: &str, field: &str) -> String {

    let animus_config = read_config_file(animus_name);
    let animus_values = animus_config.get("animus")
        .expect("Find [animus] section");

    let value = animus_values.get(field)
        .expect("Find config field value");

    format!("{}", value)
}

//
pub(crate) fn read_config_file(animus_name: &str) -> toml::Value {

    let path_string = format!(
        "~/.brainstorm/{}/config.toml", 
        animus_name
    );
    let config_path = Path::new(&path_string);

    /*
    // If no animus config exists:
    if !config_path.exists() { 
        return Err(anyhow::anyhow!("No configuration file found."))
    };
    */

    // Convert config file into Toml object:
    let config_file = std::fs::read_to_string(config_path)
        .expect("Read config.toml contents");

    let animus_config: toml::Value = config_file.parse()
        .expect("Parse config.toml contents");

    animus_config
}

//
pub(crate) fn animus_exists(animus_name: &str) -> bool {
    let mut exists = false;
    for animus in read_animi() {
        let animus = animus.expect("Access animus metadata.");
        let name = animus.file_name().into_string().unwrap();
        if &name == animus_name {
            exists = true;
        }
    }
    exists
}

//
pub(crate) fn network_exists(network_name: &str) -> bool {
    let mut exists = false;
    for saved in read_saved() {
        let saved = saved.expect("Access animus metadata.");
        let name = saved.file_name().into_string().unwrap();
        if &name == network_name {
            exists = true;
        }
    }
    exists
}

//
pub(crate) fn read_animi() -> std::fs::ReadDir {
    std::fs::read_dir("~/.brainstorm/animi")
        .expect("Framework directory must be set up. Restart brainstorm.")
}

//
pub(crate) fn read_saved() -> std::fs::ReadDir {
    std::fs::read_dir("~/.brainstorm/saved")
        .expect("Framework directory must be set up. Restart brainstorm.")
}


