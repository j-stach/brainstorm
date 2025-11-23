
use std::path::Path;

use ezcfg::Config;
use super::cfg::BrainstormConfig;

// Call expression per directory 
macro_rules! per_missing_dir {
    ($fn:expr) => {{

        let root = Path::new("~/.cajal");       // Framework directory
        let animi = &root.join("animi");        // Animus records and run
        let saved = &root.join("saved");        // Serialized networks
        let brain = &root.join("brainstorm");   // Brainstorm configs

        let directories = vec![root, animi, saved, brain];

        for dir in directories {
            if !dir.is_dir() {
                $fn(dir);
            }
        }
    }}
}

// Ensure the framework's directory structure is in place.
pub(crate) fn directory_setup() -> anyhow::Result<()> {

    per_missing_dir!(|d| std::fs::create_dir(d)
        .expect("Permission to create dir"));

    // Generate a default config file if none exists
    let config = Path::new(&BrainstormConfig::PATH);
    if !config.exists() {
        BrainstormConfig::default().write()?;
    }

    Ok(())
}

// Are all directories in place?
pub(crate) fn setup_ok() -> bool {
    per_missing_dir!(|_| return false );
    true
}


