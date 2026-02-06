
//! Helper functions for editing groups

// Read the saved networks directory.
pub(crate) fn read_groups() -> anyhow::Result<std::fs::ReadDir> {
    Ok(std::fs::read_dir("~/.cajal/animi/groups")?)
}

// Check if a network binary with the given name exists in the `saved`` folder.
// Expects that the filesystem is correctly configured and readable.
pub(crate) fn group_exists(group_name: &str) -> anyhow::Result<bool> {

    let exists = read_groups()?
        .flatten()
        .any(|f| f.file_name() == group_name);

    Ok(exists)
}

// Create a string representing the path to a group file.
pub(crate) fn group_path(group_name: &str) -> String {
    format!("~/.cajal/animi/groups/{}", group_name)
}

// Get list of animi in a group.
// Only skips newline, doesn't check validity
pub(crate) fn read_group_members(group: &str) -> anyhow::Result<Vec<String>> {

    let file = std::fs::read_to_string(group_path(group))?;

    let mut members: Vec<String> = file.split('\n')
        .map(|l| l.to_string())
        .collect();

    members.retain(|line| line != "\n");
    
    Ok(members)
}

// Write list of animi into file.
// Assumes group already exists, it's just gonna write
pub(crate) fn write_group_members(group: &str, members: Vec<String>) -> anyhow::Result<()> { 

    let file: String = members.join("\n");
    std::fs::write(group_path(group), file)?;

    Ok(())
}

// Write animus name to group file
// Assumes valid name, exists, etc.
pub(crate) fn group_add_animus(group: &str, animus: &str) -> anyhow::Result<()> { 

    let mut members = read_group_members(group)?;
    if members.contains(&animus.to_string()) {
        return Err(anyhow::anyhow!("Group already contains '{}'", animus))
    }

    members.push(animus.to_string());
    write_group_members(group, members)?;

    Ok(())
}

// Remove line containing animus name to group file
// Assumes valid name, exists, etc.
pub(crate) fn group_remove_animus(group: &str, animus: &str) -> anyhow::Result<()> { 

    let mut members = read_group_members(group)?;
    if !members.contains(&animus.to_string()) {
        return Err(anyhow::anyhow!("Animus '{}' not found", animus))
    }

    members.retain(|a| a != animus);
    write_group_members(group, members)?;

    Ok(())
}




