
use crate::helpers::*;
use crate::error::SetupError;

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


