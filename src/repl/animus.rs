
// This aligns with the capabilities of the `cajal-animus` 
// and executes the stored commands via the dedicated 4048 port.


use clap::{ Parser, Subcommand };
use clap_repl::{ ClapEditor, ReadCommandOutput };
use clap_repl::reedline::{ DefaultPrompt, DefaultPromptSegment };

use crate::helpers::send_animus_command;


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

                // Get name from complex
                AnimusCommand::Name => {
                    send_animus_command(animus_name, "name");
                    println!("Animus name is: ");
                },

                // Get version from complex
                AnimusCommand::Version => {
                    send_animus_command(animus_name, "version");
                    println!("Animus version is: ");
                },

                // Get list of structures from complex
                AnimusCommand::ListStructures => {
                    send_animus_command(animus_name, "list_structures");
                    println!("This animus contains the following structures: ");
                },

                AnimusCommand::Save => {
                    println!("Saving network state...");
                    send_animus_command(animus_name, "save");
                    println!("Done");
                },

                // Start processing inputs for the animus
                AnimusCommand::Vive => {
                    send_animus_command(animus_name, "vive");
                    println!("It's Alive!");
                },

                // Stop processing inputs, save
                AnimusCommand::RespiceFinem => {
                    send_animus_command(animus_name, "respice_finem");
                    println!("Memento mori");
                    run_manager = false
                },

                // TODO: 
                // Terminate the animus service
                // Get status (ready/running/off)
                
                AnimusCommand::Return => {
                    run_manager = false
                },
            },

            ReadCommandOutput::EmptyLine => { /* Do nothing */ },
            ReadCommandOutput::ClapError(e) => { println!{"{}", e}},
            ReadCommandOutput::ReedlineError(e) => { println!{"{}", e}},
            ReadCommandOutput::ShlexError => { println!{"Bad input syntax"}},

            _ => {}

        }
    }

}

