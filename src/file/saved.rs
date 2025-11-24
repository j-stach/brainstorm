
//! Helper functions for reading saved networks

// Read the saved networks directory.
pub(crate) fn read_saved() -> anyhow::Result<std::fs::ReadDir> {
    Ok(std::fs::read_dir("~/.cajal/saved")?)
}

// Check if a network binary with the given name exists in the `saved`` folder.
// Expects that the filesystem is correctly configured and readable.
pub(crate) fn network_exists(network_name: &str) -> anyhow::Result<bool> {

    let exists = read_saved()?
        .flatten()
        .any(|f| f.file_name() == network_name);

    Ok(exists)
}

// Create a string representing the path to an animus's dedicated directory.
pub(crate) fn network_path(network_name: &str) -> String {
    format!("~/.cajal/saved/{}.nn", network_name)
}

