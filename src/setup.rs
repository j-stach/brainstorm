
use std::{ fs, path::Path };

// Ensure the framework's directory structure is in place.
pub(crate) fn directory_setup() -> anyhow::Result<()> {

    let root = Path::new("~/.brainstorm");  // Framework directory
    let animi = &root.join("animi");        // Animus records and chroot
    let saved = &root.join("saved");        // Serialized networks
    let cajal = &root.join("cajal");        // Global library configs

    let directories = vec![root, animi, cajal, saved];

    for dir in directories {
        if !dir.is_dir() {
            fs::create_dir(dir)?;
            // TODO: Protect from accidental modification/corruption
        }
    }

    Ok(())
}

