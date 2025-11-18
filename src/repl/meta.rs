
mod list;
mod animate;
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
    long_about = "A tool for managing Animus services for Cajal-based simulated spiking neural networks.",
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
"Provide the name of the `.nn` file that holds the network to be animated.
Brainstorm will search for the file in ~/.cajal/saved.
Use `list-networks` to view saved network names." 
        )]
        network: std::path::PathBuf
    },

    /// Load and activate an Animus that is saved on this device.
    Load {
        #[arg(
            help = 
"Provide the name of the Animus as it appears in the filesystem. 
Brainstorm will search for the animus in ~/.cajal/animi.
View all available Animi using the `list-all` command."
        )]
        animus: String
    },


    /// Select an active Animus to manage.
    Select {
        #[arg(
            help = 
"Provide the name of the Animus as it appears in the filesystem. 
The Animus must currently be active.
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
pub(crate) fn meta_repl() {

    println!("Welcome to Brainstorm! For usage information, enter 'help'.");

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

            Command::Quit | Command::Exit => {
                println!("Goodbye!");
                std::process::exit(0);
            },

            Command::ListActive => {
                // TODO Need a ways to query all animi at 4048
                list::active_animi()
            },

            Command::ListAll => {
                list::all_animi()
            },

            Command::ListNetworks => {
                list::saved_networks()
            },

            Command::Animate { network } => {
                let network_filename = network.display().to_owned();
                animate::animate_network(&network_filename)
            },

            Command::Load { animus } => {
                load::load_animus(&animus)
            },

            Command::Select { animus } => {

                // TODO Needs to be able to send commands to animi that 
                // "don't exist", in case records were accidentally deleted.
                // The current helper only checks if the directory is present.
                /*
                if !animus_exists(&animus) {
                    println!("Animus '{}' not found! Use `list-all`", &animus)
                }
                */

                super::animus::command_repl(&animus)
            },

        }
    });

}


