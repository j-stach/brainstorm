
use clap::{ Parser, Subcommand };
use clap_repl::ClapEditor;
use clap_repl::reedline::{ DefaultPrompt, DefaultPromptSegment };

use std::path::Path;

use crate::helpers::*;
use crate::error::SetupError;


#[derive(Parser)]
#[command(
    name = "brainstorm",
    about = "REPL for managing Animus services and networks",
    long_about = "A tool for managing Animus services for Cajal-based simulated spiking neural networks.",
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}


#[derive(Subcommand, Debug)]
enum Command {

    /// Create a new Animus for the network provided, then activate it.
    Animate {
        #[arg(
            help = 
"Provide the name of the `.nn` file that holds the network to be animated.
Brainstorm will search for the file in ~/.cajal/saved.
Use `list-networks` to view saved network names." 
        )]
        network: std::path::PathBuf
    },

    /// Load and activate an Animus that is saved on this device.
    Load {
        #[arg(
            help = 
"Provide the name of the Animus as it appears in the filesystem. 
Brainstorm will search for the animus in ~/.cajal/animi.
View all available Animi using the `list-all` command."
        )]
        animus_name: String
    },


    /// Select an active Animus to manage.
    Select {
        #[arg(
            help = 
"Provide the name of the Animus as it appears in the filesystem. 
The Animus must currently be active.
View active Animi using the `list-active` command."
        )]
        animus_name: String
    },

    /// List all Animi that are currently active on this device.
    ListActive,

    /// List all Animi that have data saved in ~/.cajal/animi/
    ListAll,

    /// List all `.nn` networks found in ~/.cajal/saved/
    ListNetworks,

    /// Exit Brainstorm. (This will not affect any active Animi.)
    Quit, Exit,
}


// Launch the top-level REPL and process commands.
pub(crate) fn brainstorm_repl() {

    println!("Welcome to Brainstorm! For usage information, enter 'help'.");

    let prompt = DefaultPrompt {
        left_prompt: DefaultPromptSegment::Basic("brainstorm".to_owned()),
        ..DefaultPrompt::default()
    };

    let repl = ClapEditor::<Cli>::builder()
        .with_prompt(Box::new(prompt))
        .build();

    // Handle commands:
    repl.repl(|cli: Cli| {
        match cli.command {

            Command::Quit | Command::Exit => {
                count_active_animi();
                println!("Goodbye!");
                std::process::exit(0);
            },

            Command::ListActive => {
                // TODO Need a ways to query all animi at 4048
                list_active_animi()
            },

            Command::ListAll => {
                list_all_animi()
            },

            Command::ListNetworks => {
                list_saved_networks()
            },

            Command::Animate { network } => {
                let network_filename = network.display().to_string();
                animate_network(&network_filename)
            },

            Command::Load { animus_name } => {
                load_animus(&animus_name)
            },

            Command::Select { animus_name } => {
                if animus_exists(&animus_name) {
                    super::animus_manager_repl(&animus_name)
                } else {
                    println!("Animus '{}' not found! Use `list-all`", &animus_name)
                }
            },

        }
    });

}


/* Helper functions */

// Print a list of all active animi that can be found in the `animi` directory.
// Expects that all files within have valid animus filestructures.
fn list_active_animi() {

    // TODO Need a ways to query all animi at 4048

    let animi = read_animi();
    for animus in animi {
        let animus = animus
            .expect("Access animus metadata. If you are seeing this message, your `animi` directory contains an unrecognized filestructure or you lack permission to access it.");
        let name = animus.file_name().into_string()
            .expect("Animus name must be a valid string. If you are seeing this message, your `animi` directory contains an unrecognized filestructure or you lack permission to access it.");
        if animus_is_active(&name).unwrap() {
            println!("{}", name) 
        }
    }

    // TODO Print total animi active 
}


// Print a list of all files that appear in the `animi` directory.
// Expects that all files within have valid animus filestructures.
fn list_all_animi() {

    let animi = read_animi();
    for animus in animi {
        let animus = animus
            .expect("Access animus metadata. If you are seeing this message, your `animi` directory contains an unrecognized filestructure or you lack permission to access it.");
        let name = animus.file_name().into_string()
            .expect("Animus name must be a valid string. If you are seeing this message, your `animi` directory contains an unrecognized filestructure or you lack permission to access it.");
        println!("{}", name) 
    }
}


// Print a list of all network files that appear in the `saved` directory.
fn list_saved_networks() {

    let saved = read_saved();
    for network in saved {
        let network = network
            .expect("Access network metadata. If you are seeing this message, your `saved` directory contains an unrecognized filestructure or you lack permission to access it.");
        let name = network.file_name().into_string()
            .expect("Network name must be a valid string. If you are seeing this message, your `saved` directory contains an unrecognized filestructure or you lack permission to access it.");
        if name.ends_with(".nn") {
            println!("{}", name) 
        }
    }
}


// Load an existing but inactive animus.
fn load_animus(animus_name: &str) {

    if animus_exists(&animus_name) {
        match animus_is_active(&animus_name) {

            Ok(active) => {
                if !active {
                    match launch_animus(&animus_name) {

                        Ok(_) => {
                            println!("Animus '{}' is loaded!", &animus_name)
                        },

                        Err(e) => {
                            println!("An error occurred during launch.");
                            eprintln!("{}", e);
                        }
                    }
                } else { 
                    println!("Animus '{}' is already active!", &animus_name)
                }
            },

            Err(e) => {
                println!("Failed to connect to animus host IP address.");
                eprintln!("{}", e);
            }
        }
    } else {
        println!("Animus '{}' not found! Use `list-all`", &animus_name)
    }
}


// Create a new animus setup for the given network.
fn animate_network(network_filename: &str) {

    if network_exists(&network_filename) {
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
    let animus_path = Path::new(&animus_dir);
    if !animus_path.is_dir() {
        std::fs::create_dir(animus_path)
            .expect("Create animus directory");
    }

    // Run the REPL to generate config.
    // WARNING: REPL expects the animus_name is valid.
    super::animus_config_repl(&animus_name);
    build_animus(&animus_name)?;

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


// Execute the animusd service for an animus.
fn launch_animus(animus_name: &str) -> Result<(), SetupError> {

    let animus_dir = animus_dir(&animus_name);
    let bin_path = format!("{}/bin/animusd-{}", &animus_dir, &animus_name);
    
    // The binary should have been made executable when it was set up above.
    let mut cmd = std::process::Command::new(bin_path);

    let result = cmd.output()?;
    if !result.status.success() {
        return Err(SetupError::ExecutionFailed("animusd".to_string()))
    }

    Ok(())
}



