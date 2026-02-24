
//! Helper functions for reading animus files


// Check if a proposed animus name fits the formatting requirements. 
// (a-Z, 0-9, and underscores)
pub(crate) fn valid_animus_name(name: &str) -> bool {
    if name.is_empty() { return false }
    name.chars().all(|c| { c.is_ascii_alphanumeric() || c == '_' })
}

pub(crate) fn local_animus_path(name: &str) -> String {
    let home = std::env::home_dir()
        .expect("Find user home directory");
    let local = &home.join(".cajal").join("animi").join("local");
    format!("{}/{}", local.display(), name)
}

// Read the animus directory.
pub(crate) fn read_local_animi() -> anyhow::Result<std::fs::ReadDir> {
    let home = std::env::home_dir()
        .expect("Find user home directory");
    let local = &home.join(".cajal").join("animi").join("local");
    Ok(std::fs::read_dir(local)?)
}

// Read the animus directory.
pub(crate) fn read_remote_animi() -> anyhow::Result<std::fs::ReadDir> {
    let home = std::env::home_dir()
        .expect("Find user home directory");
    let remote = &home.join(".cajal").join("animi").join("remote");
    Ok(std::fs::read_dir(remote)?)
}

// Check if local data exists for an animus with the given name.
pub(crate) fn local_animus_exists(name: &str) -> anyhow::Result<bool> {

    let exists = read_local_animi()?
        .flatten()
        .any(|f| f.file_name() == name);

    Ok(exists)
}

// Check if a record exists for a remote animus with the given name.
pub(crate) fn remote_animus_exists(name: &str) -> anyhow::Result<bool> {

    let exists = read_remote_animi()?
        .flatten()
        .any(|f| f.file_name() == name);

    Ok(exists)
}

pub(crate) fn animus_exists(name: &str) -> anyhow::Result<bool> {
    let exists = local_animus_exists(name)? || remote_animus_exists(name)?;
    Ok(exists)
}
