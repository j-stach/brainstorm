
use std::{ fs, path::Path };

// Ensure the framework's directory structure is in place.
pub(crate) fn directory_setup() -> anyhow::Result<()> {

    let root = Path::new("~/.cajal");       // Framework directory
    let animi = &root.join("animi");        // Animus records and chroot
    let saved = &root.join("saved");        // Serialized networks
    let brain = &root.join("brainstorm");   // Global library configs

    let directories = vec![root, animi, saved, brain];

    for dir in directories {
        if !dir.is_dir() {
            fs::create_dir(dir)?;
        }
    }

    // TODO: Establish a config file for brainstorm,
    // including authentication codes, etc.
    // Use `ez-cfg` to create file w params
    /*
    let config = &brain.join("brainstorm.cfg");
    // TODO Function to auto-generate configuration file
    */

    Ok(())
}

// TODO Helper function to generate configuration file
/*
pub fn default_config() { ... }
*/
