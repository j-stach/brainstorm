
//! Helper functions for reading animus files


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
        // TODO Error
        .expect("Framework directory must be set up. Restart brainstorm.")
}

// Create a string representing the path to an animus's dedicated directory.
pub(crate) fn animus_dir(animus_name: &str) -> String {

    format!("~/.cajal/animi/{}", animus_name)
}

// Check if data exists for an animus with the given name.
// Expects that the filesystem is correctly configured and readable.
pub(crate) fn animus_exists(animus_name: &str) -> bool {

    let mut exists = false;
    for animus in read_animi() {

        let animus = animus
            // TODO Error
            .expect("Access animus metadata. If you are seeing this message, your `animi` directory contains an unrecognized filestructure or you lack permission to access it.");

        let name = animus.file_name().into_string()
            // TODO Error
            .expect("Animus name must be a valid string. If you are seeing this message, your `animi` directory contains an unrecognized filestructure or you lack permission to access it.");

        if &name == animus_name {
            exists = true;
        }

    }
    exists
}

