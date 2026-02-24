
// Call expression per directory 
macro_rules! per_missing_dir {
    ($fn:expr) => {{

        let home = std::env::home_dir()
            .expect("Find user home directory");

        let root = &home.join(".cajal");        // Framework directory

        let animi = &root.join("animi");        // Animus records
        let remote = &animi.join("remote");     // Remote animi files
        let local = &animi.join("local");       // Local animi files & run dir
        let groups = &animi.join("groups");     // Groups of animi

        let saved = &root.join("saved");        // Serialized networks
        let brain = &root.join("brainstorm");   // Brainstorm configs

        let directories = vec![root, animi, remote, local, groups, saved, brain];

        for dir in directories {
            if !dir.exists() {
                $fn(dir);
            }
        }
    }}
}

// Ensure the framework's directory structure is in place.
pub(crate) fn directory_setup() -> anyhow::Result<()> {

    per_missing_dir!(|d| std::fs::create_dir(d)
        .expect("Permission to create dir"));

    Ok(())
}

// Are all directories in place?
pub(crate) fn setup_ok() -> bool {
    let mut ok = true;
    per_missing_dir!(|_| ok = false); // LOL does this work?
    ok
}


