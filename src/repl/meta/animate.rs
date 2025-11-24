
use crate::helpers::*;
use crate::error::SetupError;

// Create a new animus setup for the given network.
pub(super) fn animate_network(network_filename: &str) -> anyhow::Result<()> {

    if network_exists(&network_filename)? {
        match animus_setup(&network_filename) {

            Ok(animus_name) => {
                match launch_animus(&animus_name) {

                    Ok(_) => {
                        println!("Animus '{}' is loaded!", &animus_name)
                    },

                    Err(e) => {
                        println!("An error occurred during launch.");
                        eprintln!("{}", e);
                    }
                }
            },

            Err(e) => {
                println!("An error occurred during setup.");
                eprintln!("{}", e);
            }

        }
    } else {
        println!("Network '{}' not found! Use `list-networks`", network_filename)
    }
}


// Set up a new animus directory, animusd executable, and all necessary files.
fn animus_setup(network_filename: &str) -> Result<String, SetupError> {

    if !network_filename.ends_with(".nn") {
        return Err(SetupError::BadFilename(network_filename.to_string()))
    }

    let animus_name = network_filename.strip_suffix(".nn")
        // Safe because we check for it above
        .expect("Filename has a `.nn` suffix")
        .to_string();

    // Rename the animus if necessary (e.g. name already exists or is invalid)
    let animus_name = rename_animus(animus_name)?;

    if animus_name.is_none() {
        return Err(SetupError::SetupAborted)
    }

    // Safe because we check for it above.
    let animus_name = animus_name.expect("Animus name exists");

    // Create animus directory
    let animus_dir = animus_dir(&animus_name);
    let animus_path = std::path::Path::new(&animus_dir);
    if !animus_path.is_dir() {
        std::fs::create_dir(animus_path)
            .expect("Create animus directory");
    }

    // TODO Use flags for animate command instead of config loop
    /*
    // Run the REPL to generate config.
    // WARNING: REPL expects the animus_name is valid.
    super::animus_config_repl(&animus_name);
    build_animus(&animus_name)?;
    */

    Ok(animus_name)
}


// Check if the proposed animus name is valid, then rename it if necessary.
fn rename_animus(mut animus_name: String) -> Result<Option<String>, SetupError> {

    let mut valid_name = valid_animus_name(&animus_name);

    while !valid_name && &animus_name != "" {
        if valid_animus_name(&animus_name) {
            if animus_is_active(&animus_name)? {
                println!("Animus '{}' is already active!", &animus_name);
            } else {
                valid_name = true;
            }
        } else {
            println!("Invalid character(s) in string. Use a-Z, 0-9, or underscores.")
        }

        if !valid_name {
            println!("Type a new name or sumbit an empty line to cancel.");
            print!("New name: ");

            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)?;
            animus_name = input.trim().to_string();
        }
    }

    if valid_name {
        println!("Name: {}", animus_name);
        Ok(Some(animus_name))
    } else {
        println!("No name chosen.");
        Ok(None)
    }
}


// Download a unique `animusd` executable for an animus, based on lib features.
fn build_animus(animus_name: &str) -> Result<(), SetupError> {

    // Build the animusd executable with the features specified.
    let animus_dir = animus_dir(&animus_name);
    let features = read_animus_features(animus_name)?;

    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("install").arg("animusd");
    if !features.is_empty() {
        cmd.arg("--features").arg(&features);
    }
    cmd.arg("--root").arg(&animus_dir);

    let result = cmd.output()?;
    if !result.status.success() {
        return Err(SetupError::InvalidFeatures(features.to_string()))
    }

    // Rename the service executable to distinguish it as a process.
    let default_path = format!("{}/bin/animusd", &animus_dir);
    let bin_path = format!("{}-{}", &default_path, &animus_name);

    let mut cmd = std::process::Command::new("mv");
    cmd.arg(&default_path).arg(&bin_path);

    let result = cmd.output()?;
    if !result.status.success() {
        return Err(SetupError::ExecutionFailed("mv".to_string()))
    }

    // Make animusd executable.
    let mut cmd = std::process::Command::new("chmod");
    cmd.arg("+x").arg(&bin_path);

    let result = cmd.output()?;
    if !result.status.success() {
        return Err(SetupError::ExecutionFailed("chmod".to_string()))
    }

    Ok(())
}

