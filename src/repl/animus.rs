
// This aligns with the capabilities of the `animusd` protocol 
// and executes the stored commands via the dedicated 4048 port.


use clap::{ Parser, Subcommand };
use clap_repl::{ ClapEditor, ReadCommandOutput };
use clap_repl::reedline::{ DefaultPrompt, DefaultPromptSegment };

use animusd_lib::protocol::AnimusAction::*;

use crate::helpers::*;


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
    Wake,

    /// Stops the Animus from processing new input signals.
    Sleep,

    /// Reports whether the Animus is awake or asleep.
    Status,

    /// Shut down the Animus.
    Terminate,

    /// Return to the parent Brainstorm REPL.
    Back,
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

                // Print the name of the animus
                AnimusCommand::Name => {
                    let response = send_animus_command(animus_name, Name);
                    handle_animus_response(animus_name, response);
                },

                // Print the version of the animus
                AnimusCommand::Version => {
                    let response = send_animus_command(animus_name, Version);
                    handle_animus_response(animus_name, response);
                },

                // Print the list of structures in the complex
                AnimusCommand::ListStructures => {

                    let response = send_animus_command(
                        animus_name, 
                        ListStructures
                    );

                    if let Err(e) = response {
                        report_command_error(e);
                    } else {
                        // Unwrap is safe because we check for errors above.
                        if let Some(csv) = response.unwrap() {
                            // Structures will be listed as a CSV string,
                            // Needs to be split and listed vertically.
                            let values: Vec<_> = csv.split(",").collect();
                            for value in values {
                                println!("{}", value);
                            }
                        } else {
                            println!("No response from animus {}", animus_name);
                        }
                    }
                },

                // Save the network back to the save file
                AnimusCommand::Save => {
                    println!("Saving network state...");
                    let response = send_animus_command(animus_name, Save);
                    handle_animus_response(animus_name, response);
                },

                // Start processing inputs for the animus
                AnimusCommand::Wake => {
                    let response = send_animus_command(animus_name, Wake);
                    handle_animus_response(animus_name, response);
                },

                // Stop processing inputs
                AnimusCommand::Sleep => {
                    let response = send_animus_command(animus_name, Sleep);
                    handle_animus_response(animus_name, response);
                },
                
                // Report the status (awake/asleep)
                AnimusCommand::Status => {
                    let response = send_animus_command(animus_name, Status);
                    handle_animus_response(animus_name, response);
                },

                // Shut down the service
                AnimusCommand::Terminate => {
                    let response = send_animus_command(animus_name, Terminate);
                    handle_animus_response(animus_name, response);
                },
                
                // Exit the loop
                AnimusCommand::Back => {
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

