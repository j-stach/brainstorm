
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
        filename: std::path::PathBuf
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

    // TODO: CheckHealth
    // TODO: Setup
    // TODO: AddLobe

    /// Exit Brainstorm (This will not affect any active Animi).
    Quit,
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

    repl.repl(|cli: Cli| {
        match cli.command {

            Command::Quit => {
                println!("Byee~");
                std::process::exit(0);
            },

            Command::ListActive => {
                // Query communication port for names and versions of any active animi, then collect and report list
            },

            Command::ListAll => {
                // Read the ~/.brainstorm/animi directory for names
            },

            Command::ListNetworks => {
                // Read the ~/.brainstorm/saved directory for names
            },

            Command::Animate {..} => {
                // Search for network filename
                // Build animus
            },

            Command::Load {..} => {
                // Search for animus directory
                // Load & run
            },

            Command::Select { animus_name } => {
                // Check for animus in active list
                super::animus::animus_manager_repl(&animus_name)
            },

        }
    });

}


