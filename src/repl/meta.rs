
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

use crate::file;


#[derive(Parser)]
#[command(
    name = "brainstorm",
    about = "REPL for managing Animus services and networks",
)]
struct Cli {
    #[command(subcommand)]
    command: MetaCommand,
}


#[derive(Subcommand, Debug)]
enum MetaCommand {

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

impl crate::Brainstorm {

    // Launch the top-level REPL and process commands.
    pub(crate) fn meta_repl(&self) {

        println!("Welcome to Brainstorm! For usage information, enter 'help'");

        // Set the prompt appearance
        let prompt = DefaultPrompt {
            left_prompt: DefaultPromptSegment::Basic("brainstorm".to_owned()),
            ..DefaultPrompt::default()
        };

        let repl = ClapEditor::<Cli>::builder()
            .with_prompt(Box::new(prompt))
            .build();

        // TODO: Silent logging for animus Reports, which in the future 
        // may be sent independently from commands.

        // Execute commands:
        repl.repl(|cli: Cli| {
            match cli.command {

                // Exit Brainstorm
                MetaCommand::Quit | MetaCommand::Exit => {
                    println!("Goodbye!");
                    std::process::exit(0);
                },

                // List all animi that are listening for commands
                MetaCommand::ListActive => {
                    if let Err(e) = list::active_animi() {
                        Self::meta_command_error("list-active", e)
                    }
                },

                // List all animi saved in ~/.cajal/animi 
                MetaCommand::ListAll => {
                    if let Err(e) = list::all_animi() {
                        Self::meta_command_error("list-all", e)
                    }
                },

                // List all networks saved in ~/.cajal/saved
                MetaCommand::ListNetworks => {
                    if let Err(e) = list::saved_networks() {
                        Self::meta_command_error("list-networks", e)
                    }
                },

                // Configure and build
                MetaCommand::Animate { network } => {
                    let network_filename = network.display().to_string();
                    if let Err(e) = self.animate_network(&network_filename) {
                        Self::meta_command_error("animate", e)
                    }
                },

                // Launch an animus so it can begin receiving commands
                MetaCommand::Load { animus } => {
                    if let Err(e) = self.load_animus(&animus) {
                        Self::meta_command_error("load", e)
                    }
                },

                // Select an active (loaded) animus to issue commands
                MetaCommand::Select { animus } => {

                    let is_active = self.is_active(&animus);

                    if let Err(e) = is_active {
                        return Self::meta_command_error("select", e)
                    } else if !is_active.expect("Checked above") {

                        return Self::meta_command_error(
                            "select", 
                            anyhow::anyhow!("'{}' is not active", &animus)
                        )
                    }
                    
                    let exists = file::animi::animus_exists(&animus);

                    if let Err(e) = exists {
                        return Self::meta_command_error("select", e)
                    } else if !exists.expect("Checked above") {
                        println!("WARN: '{}' is unregistered", &animus)
                    }

                    self.animus_repl(&animus)
                },

            }
        });

    }

    // Handle errors
    fn meta_command_error(cmd: &str, e: anyhow::Error) {
        
        println!("WARN: An error occurred while executing '{}' command", cmd);
        eprintln!("{}", e);
    }
}

