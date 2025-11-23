
//! Helper functions for reading animus files


// Check if a proposed animus name fits the formatting requirements. 
// (a-Z, 0-9, and underscores)
pub(crate) fn valid_animus_name(name: &str) -> bool {
    if name.is_empty() { return false }
    name.chars().all(|c| { c.is_ascii_alphanumeric() || c == '_' })
}

// Create a string representing the path to an animus's dedicated directory.
pub(crate) fn animus_dir(animus_name: &str) -> String {
    format!("~/.cajal/animi/{}", animus_name)
}

// Read the animus directory.
pub(crate) fn read_animi() -> anyhow::Result<std::fs::ReadDir> {
    Ok(std::fs::read_dir("~/.cajal/animi")?)
}

// Check if data exists for an animus with the given name.
pub(crate) fn animus_exists(animus_name: &str) -> anyhow::Result<bool> {

    let exists = read_animi()?
        .flatten()
        .any(|f| f.file_name() == animus_name);

    Ok(exists)
}

