
//! Handles loading the animi and launching their executables

// Load and activate an existing but inactive animus.
pub(super) fn load_animus(animus_name: &str) -> anyhow::Result<()> {

    if crate::file::animi::animus_exists(animus_name)? {

        if !crate::repl::animus::is_active(animus_name)? {
            launch_animus(animus_name)?;
            println!("Animus '{}' has loaded", animus_name)
        } else {
            println!("An animus named '{}' is already running", animus_name)
        }

    } else {
        println!("'{}' not found! Use `animate` to generate it", animus_name)
    }

    Ok(())
}

// Execute the animusd service for an animus.
fn launch_animus(animus_name: &str) -> anyhow::Result<()> {

    let animus_dir = crate::file::animi::animus_dir(animus_name);
    let bin_path = format!("{}/bin/animusd-{}", animus_dir, animus_name);
    
    // The binary should have been made executable when it was set up.
    let mut cmd = std::process::Command::new(bin_path);

    let result = cmd.output()?;
    if !result.status.success() {
        // TODO Print error?
        // TBD Returns on execeution or completion?
    }

    Ok(())
}


