
// Read the saved networks directory.
// Expects that the filesystem is correctly configured and readable.
pub(crate) fn read_saved() -> std::fs::ReadDir {
    std::fs::read_dir("~/.brainstorm/saved")
        .expect("Framework directory must be set up. Restart brainstorm.")
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

