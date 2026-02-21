
use clap::{ Parser, Subcommand };
use clap_repl::{
    ClapEditor, 
    reedline::{ 
        DefaultPrompt, 
        DefaultPromptSegment 
    },
};

use crate::file;
use super::helpers::list;


#[derive(Parser)]
#[command(
    name = "brainstorm",
    about = "REPL for managing Animus services and networks",
)]
struct MetaCli {
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

    /// Select a group of animi to manage.
    Group {
        #[arg( help = 
            "Provide the name of the group as it appears in the filesystem. \
            To create a new group, enter the desired name,
            then submit Y for the next prompt."
        )]
        name: String
    },

    /// Register a animus running on another device, 
    /// to be controlled by Brainstorm on this computer.
    AddRemote {
        #[arg( help = 
            "Provide the name of the animus as it appears in the other filesystem.\
            The Animus must currently be active. \n\
            View active Animi using the `list-active` command in Brainstorm \
            on the other device."
        )]
        animus: String,
        #[arg( help = 
            "Provide the IP address of the other device -- e.g., 1.2.3.4"
        )]
        ip: std::net::IpAddr,
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

    pub(crate) fn meta_manager(&self) {

        println!("Welcome to Brainstorm! For usage information, enter 'help'");

        let repl = Self::meta_repl_setup();
        self.execute_meta_commands(repl)
    }

    fn meta_repl_setup() -> ClapEditor<MetaCli> {

        // Set the prompt appearance
        let prompt = DefaultPrompt {
            left_prompt: DefaultPromptSegment::Basic("brainstorm".to_owned()),
            ..DefaultPrompt::default()
        };

        // Create the REPL environment
        let repl = ClapEditor::<MetaCli>::builder()
            .with_prompt(Box::new(prompt))
            .build();

        repl
    }

    fn execute_meta_commands(&self, repl: ClapEditor<MetaCli>) {
        repl.repl(|cli: MetaCli| {
            match cli.command {

                MetaCommand::Quit | MetaCommand::Exit => {
                    println!("Goodbye!");
                    std::process::exit(0);
                },

                MetaCommand::ListActive => Self::list_active(),
                MetaCommand::ListAll => Self::list_all(),
                MetaCommand::ListNetworks => Self::list_networks(),

                MetaCommand::Animate { network } => self.animate(network),
                MetaCommand::Load { animus } => self.load(&animus),
                MetaCommand::Select { animus } => self.select(&animus),
                MetaCommand::Group { name } => self.group_manager(&name),

                MetaCommand::AddRemote { animus, ip } => Self::add_remote(&animus, ip),
            }
        });
    }

    // List all animi that are listening for commands
    fn list_active() {
        if let Err(e) = list::active_animi() {
            Self::meta_command_error("list-active", e)
        }
    }

    // List all animi recorded in the animus directory
    fn list_all() {
        if let Err(e) = list::all_animi() {
            Self::meta_command_error("list-all", e)
        }
    }

    // List all networks saved in ~/.cajal/saved
    fn list_networks() {
        if let Err(e) = list::saved_networks() {
            Self::meta_command_error("list-networks", e)
        }
    }

    // Configure and build a new animus for a network file
    fn animate(&self, network: std::path::PathBuf) {
        let network_filename = network.display().to_string();
        if let Err(e) = self.animate_network(&network_filename) {
            Self::meta_command_error("animate", e)
        }
    }

    // Launch an animus so it can begin receiving commands
    fn load(&self, animus: &str) {
        if let Err(e) = self.load_animus(animus) {
            Self::meta_command_error("load", e)
        }
    }

    // Select an active (loaded) animus to issue commands
    fn select(&self, animus: &str) {
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

        self.animus_manager(&animus)
    }

    // Track an animus that is running on another device
    fn add_remote(animus: &str, ip: std::net::IpAddr) {

        match crate::file::animi::animus_exists(&animus) {
            Ok(exists) => {
                if exists {
                    // TODO Err: Already exists
                } else {
                    // TODO
                    // Query remote device at ip:4048
                    // if active,
                    if let Err(e) = crate::file::remote::write_remote_animus(&animus, ip) {
                        // TODO Error
                    }
                }
            },
            Err(e) => {
                return
                // TODO Error
            }
        }
    }

    // Handle errors
    fn meta_command_error(cmd: &str, e: anyhow::Error) {
        
        println!("WARN: An error occurred while executing '{}' command", cmd);
        eprintln!("{}", e);
    }
}

