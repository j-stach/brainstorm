
use crate::helpers::animus_is_active;
use crate::error::SetupError;

// Load and activate an existing but inactive animus.
pub(super) fn load_animus(animus_name: &str) {

    if animus_exists(&animus_name) {
        match animus_is_active(&animus_name) {

            Ok(active) => {
                if !active {
                    match launch_animus(&animus_name) {

                        Ok(_) => {
                            println!("Animus '{}' is loaded!", &animus_name)
                        },

                        Err(e) => {
                            println!("An error occurred during launch.");
                            eprintln!("{}", e);
                        }
                    }
                } else { 
                    println!("Animus '{}' is already active!", &animus_name)
                }
            },

            Err(e) => {
                println!("Failed to connect to animus host IP address.");
                eprintln!("{}", e);
            }
        }
    } else {
        println!("Animus '{}' not found! Use `list-all`", &animus_name)
    }
}

// Execute the animusd service for an animus.
fn launch_animus(animus_name: &str) -> Result<(), SetupError> {

    let animus_dir = animus_dir(&animus_name);
    let bin_path = format!("{}/bin/animusd-{}", &animus_dir, &animus_name);
    
    // The binary should have been made executable when it was set up above.
    let mut cmd = std::process::Command::new(bin_path);

    let result = cmd.output()?;
    if !result.status.success() {
        return Err(SetupError::ExecutionFailed("animusd".to_string()))
    }

    Ok(())
}


