
use clap::{ Parser, Subcommand };
use clap_repl::ClapEditor;
use clap_repl::reedline::{ DefaultPrompt, DefaultPromptSegment };


#[derive(Parser)]
#[command(
    name = "brainstorm",
    about = "REPL for managing Animus services and networks",
    long_about = "This is a tool for managing Animus services for Cajal-based simulated spiking neural networks.",
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}


#[derive(Subcommand, Debug)]
enum Command {

    /// Create a new Animus for the network provided, then activate it.
    Animate {
        #[arg(
            help = 
"Provide the path to the `.nn` file that holds the network to be animated.
Brainstorm will search for the file in ~/.brainstorm/saved/ before trying elsewhere.
Use `list-networks` to view saved network filenames." 
        )]
        network: std::path::PathBuf
    },

    /// Load and activate an Animus that is saved on this device.
    Load {
        #[arg(
            help = 
"Provide the name of the Animus as it appears in the filesystem. 
The Animus must have data saved on the device.
View available Animi using the `list-all` command."
        )]
        animus_name: String
    },


    /// Select an active Animus to manage.
    Select {
        #[arg(
            help = 
"Provide the name of the Animus as it appears in the filesystem. 
The Animus must currently be active.
View active Animi using the `list-active` command."
        )]
        animus_name: String
    },

    /// List all Animi that are currently active on this device.
    ListActive,

    /// List all Animi that have data saved in ~/.brainstorm/animi/, including inactive Animi.
    ListAll,

    /// List all `.nn` networks found in ~/.brainstorm/saved/.
    ListNetworks,

    /// Exit Brainstorm (This will not affect any active Animi).
    Quit,

    // TBD: AddLobe
}


// Launch the top-level REPL and process commands.
pub(crate) fn brainstorm_repl() {
    println!("Welcome to Brainstorm! For usage information, enter 'help'.");

    let prompt = DefaultPrompt {
        left_prompt: DefaultPromptSegment::Basic("brainstorm".to_owned()),
        ..DefaultPrompt::default()
    };

    let repl = ClapEditor::<Cli>::builder()
        .with_prompt(Box::new(prompt))
        .build();

    // TODO: Errors
    repl.repl(|cli: Cli| {
        match cli.command {

            Command::Quit => {
                // TODO: "There are X animi still running in the background."
                println!("Goodbye!");
                std::process::exit(0);
            },

            Command::ListActive => {
                let animi = read_animi();
                for animus in animi {
                    let animus = animus.expect("Access animus file metadata.");
                    let name = animus.file_name().into_string().unwrap();
                    if animus_is_active(&name) {
                        println!("{}", name) 
                    }
                }
            },

            Command::ListAll => {
                let animi = read_animi();
                for animus in animi {
                    let animus = animus.expect("Access animus file metadata.");
                    let name = animus.file_name().into_string().unwrap();
                    println!("{}", name) 
                }
            },

            Command::ListNetworks => {
                let saved = read_saved();
                for network in saved {
                    let network = network.expect("Access network file metadata.");
                    let name = network.file_name().into_string().unwrap();
                    println!("{}", name) 
                }
            },

            Command::Animate { network } => {
                let network_filename = network.display().to_string();
                if network_exists(&network_filename) {
                    let config = configure_animus(&network_filename);
                    launch_animus(config);
                }
            },

            Command::Load { animus_name } => {
                if animus_exists(&animus_name) {
                    if !animus_is_active(&animus_name) {

                        // TODO: Get saved config from saved file
                        
                        let config = ();
                        launch_animus(config);
                        println!("Animus '{}' loaded!", &animus_name)
                    } else { 
                        println!("Animus '{}' is already active!", &animus_name)
                    }
                } else {
                    println!("Animus '{}' not found! Use `list-all`", &animus_name)
                }
            },

            Command::Select { animus_name } => {
                if animus_exists(&animus_name) {
                    super::animus_manager_repl(&animus_name)
                } else {
                    println!("Animus '{}' not found! Use `list-all`", &animus_name)
                }
            },

            // TBD: Remote animus startup, connection via SSH

        }
    });

}


fn read_animi() -> std::fs::ReadDir {
    std::fs::read_dir("~/.brainstorm/animi")
        .expect("Framework directory must be set up. Restart brainstorm.")
}

fn read_saved() -> std::fs::ReadDir {
    std::fs::read_dir("~/.brainstorm/saved")
        .expect("Framework directory must be set up. Restart brainstorm.")
}

fn animus_exists(animus_name: &str) -> bool {
    let mut exists = false;
    for animus in read_animi() {
        let animus = animus.expect("Access animus metadata.");
        let name = animus.file_name().into_string().unwrap();
        if &name == animus_name {
            exists = true;
        }
    }
    exists
}

fn network_exists(network_name: &str) -> bool {
    let mut exists = false;
    for saved in read_saved() {
        let saved = saved.expect("Access animus metadata.");
        let name = saved.file_name().into_string().unwrap();
        if &name == network_name {
            exists = true;
        }
    }
    exists
}

fn animus_is_active(animus_name: &str) -> bool {
    // TODO: 
    // For all lobes and locally
    // Ping animus name to 4048 to see if any respond
    todo!()
}

fn configure_animus(network_filename: &str) {
    // TODO: parse animus name from complex name .nn
    let mut animus_name = network_filename.to_string();
    
    animus_name = rename_animus(animus_name);

    // TODO: 
    // Create directory for animus_name
    // Run a loop to build AnimusConfig builder struct from animusd,
    // Config other options (eg logging, neuron model, etc.)
    // Save the config to/as a file in the animus directory
    // then return the builder
}

fn rename_animus(mut animus_name: String) -> String {

    // Check for an active animus with that name.
    // Naming the animus with an empty string will cancel.
    while animus_is_active(&animus_name) && &animus_name != "" {
        println!("Animus '{}' is already active!", &animus_name);
        println!("Type a new name or sumbit an empty line to cancel.");
        print!("New name: ");
        // TODO: wait for new name,
        // TODO: Check for valid string characters (alphanumeric & underscore)

    }


    todo!{}
}

fn launch_animus(config: ()) {
    // TODO: Take AnimusConfig 
    // Compile Cajal features based on config options
    // TODO: Launch animus runtime loop with AnimusConfig
}
