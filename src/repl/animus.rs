
// This aligns with the capabilities of the `cajal-animus` 
// and executes the stored commands via the dedicated 4048 port.


use clap::{ Parser, Subcommand };
use clap_repl::{ ClapEditor, ReadCommandOutput };
use clap_repl::reedline::{ DefaultPrompt, DefaultPromptSegment };


#[derive(Parser)]
#[command(
    name = "animus-name",
    about = "REPL for managing Animus services and networks",
    long_about = "This sub-REPL is used to manage an active Animus service",
)]
struct AnimusManagerCli {
    #[command(subcommand)]
    command: AnimusCommand,
}

#[derive(Subcommand, Debug)]
enum AnimusCommand {

    /// Retrieve the name of the Complex handled by the Animus.
    Name,

    /// Retrieve the version of the Animus.
    Version,

    /// List the names of all Structures in the Complex handled by the Animus.
    ListStructures,

    /// Save the state of the Complex to the associated network file.
    Save,

    /// Begin processing inputs for the Animus.
    Vive,

    /// Shut down the Animus, saving the state of the Complex to the associated network file.
    RespiceFinem,

    /// Return to the parent Brainstorm REPL.
    Return,
}


// Spawns an inner REPL for sending animus commands.
pub(crate) fn animus_manager_repl(animus_name: &str) {
    println!("Selected animus '{}'", animus_name);

    let prompt = DefaultPrompt {
        left_prompt: DefaultPromptSegment::Basic(animus_name.to_owned()),
        ..DefaultPrompt::default()
    };

    let mut inner_repl = ClapEditor::<AnimusManagerCli>::builder()
        .with_prompt(Box::new(prompt))
        .build();

    let mut run_manager = true;

    // Logic for commands:
    while run_manager {
        match inner_repl.read_command() {
            ReadCommandOutput::Command(cli) => match cli.command {

                AnimusCommand::Name => {
                    // TODO: Get name from complex
                    println!("Animus name is: ");
                },

                AnimusCommand::Version => {
                    // TODO: Get version from complex
                    println!("Animus version is: ");
                },

                AnimusCommand::ListStructures => {
                    // TODO: Get list of structures from complex
                    println!("This animus contains the following structures: ");
                },

                AnimusCommand::Save => {
                    // Clone & save? 
                    // Save without stopping?
                    println!("Saving network state...");
                    println!("Done");
                },

                AnimusCommand::Vive => {
                    // TODO: Start processing inputs for the animus
                    println!("It's Alive!");
                },

                AnimusCommand::RespiceFinem => {
                    // TODO: Stop processing inputs, save, then deactivate

                    println!("Memento mori");
                    run_manager = false
                },
                
                AnimusCommand::Return => {
                    run_manager = false
                },
            },

            ReadCommandOutput::EmptyLine => { /* Do nothing */ },
            ReadCommandOutput::ClapError(e) => { println!{"{}", e}},
            ReadCommandOutput::ReedlineError(e) => { println!{"{}", e}},
            ReadCommandOutput::ShlexError => { println!{"Error in input syntax"}},

            _ => {}

        }
    }

}

// Helper function to send AnimusCommand via TCP and receive AnimusResponse in exchange.
fn send_animus_command() {
    // Send command to associated IP addr @ port 4048
    // Get animus response and parse results
}

