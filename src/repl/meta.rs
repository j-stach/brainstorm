
mod list;
//mod animate;
mod load;

use clap::{ Parser, Subcommand };
use clap_repl::{
    ClapEditor, 
    reedline::{ 
        DefaultPrompt, 
        DefaultPromptSegment 
    },
};


#[derive(Parser)]
#[command(
    name = "brainstorm",
    about = "REPL for managing Animus services and networks",
    // TODO long-about = "",
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}


#[derive(Subcommand, Debug)]
enum Command {

    /// Create a new Animus for the network provided, then activate it.
    Animate {
        #[arg( help = 
            "Provide the name of the `.nn` network to be animated. \
            Brainstorm will search for the file in ~/.cajal/saved. \n\
            Use `list-networks` to view saved network names."
        )]
        network: std::path::PathBuf
    },

    /// Load and activate an Animus that is saved on this device.
    Load {
        #[arg( help = 
            "Provide the name of the Animus as it appears in the filesystem. \
            Brainstorm will search for the animus in ~/.cajal/animi. \n\
            View all available Animi using the `list-all` command."
        )]
        animus: String
    },


    /// Select an active Animus to manage.
    Select {
        #[arg( help = 
            "Provide the name of the Animus as it appears in the filesystem. \
            The Animus must currently be active. \n\
            View active Animi using the `list-active` command."
        )]
        animus: String
    },

    /// List all Animi that are currently active on this device.
    ListActive,

    /// List all Animi that have data saved in ~/.cajal/animi/
    ListAll,

    /// List all `.nn` networks found in ~/.cajal/saved/
    ListNetworks,

    /// Exit Brainstorm. (This will not affect any active Animi.)
    Quit, Exit,
}


// Launch the top-level REPL and process commands.
pub(crate) fn meta_repl(_config: crate::file::cfg::BrainstormConfig) {

    println!("Welcome to Brainstorm! For usage information, enter 'help'");

    // Set the prompt appearance
    let prompt = DefaultPrompt {
        left_prompt: DefaultPromptSegment::Basic("brainstorm".to_owned()),
        ..DefaultPrompt::default()
    };

    let repl = ClapEditor::<Cli>::builder()
        .with_prompt(Box::new(prompt))
        .build();

    // Execute commands:
    repl.repl(|cli: Cli| {
        match cli.command {

            // Exit Brainstorm
            Command::Quit | Command::Exit => {
                println!("Goodbye!");
                std::process::exit(0);
            },

            // List all animi that are listening for commands
            Command::ListActive => {
                if let Err(e) = list::active_animi() {
                    handle_command_error("list-active", e)
                }
            },

            // List all animi saved in ~/.cajal/animi 
            Command::ListAll => {
                if let Err(e) = list::all_animi() {
                    handle_command_error("list-all", e)
                }
            },

            // List all networks saved in ~/.cajal/saved
            Command::ListNetworks => {
                if let Err(e) = list::saved_networks() {
                    handle_command_error("list-networks", e)
                }
            },

            // Configure and build
            Command::Animate { network } => {
                //let network_filename = network.display().to_string();
                //if let Err(e) = animate::animate_network(&network_filename) {
                //    handle_command_error("animate", e)
                //}
            },

            // Launch an animus so it can begin receiving commands
            Command::Load { animus } => {
                if let Err(e) = load::load_animus(&animus) {
                    handle_command_error("load", e)
                }
            },

            // Select an active (loaded) animus to issue commands
            Command::Select { animus } => {

                // TODO Needs to be able to send commands to animi that 
                // "don't exist", in case records were accidentally deleted.
                // The current helper only checks if the directory is present.
                /*
                if !animus_exists(&animus) {
                    println!("Animus '{}' not found! Use `list-all`", &animus)
                }
                */

                // TODO Query animus

                //super::animus::command_repl(&animus)
            },

        }
    });

}

fn handle_command_error(cmd: &str, e: anyhow::Error) {
    
    println!("WARN: An error occurred while executing '{}' command", cmd);
    eprintln!("{}", e);
}

