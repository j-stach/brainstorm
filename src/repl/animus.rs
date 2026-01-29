
//! This aligns Brainstorm with the capabilities of the `animusd` protocol.

mod command;
mod report;


use clap::{ Parser, Subcommand };
use clap_repl::{ ClapEditor, ReadCommandOutput };
use clap_repl::reedline::{ DefaultPrompt, DefaultPromptSegment };

use animusd_lib::protocol::Action;


#[derive(Parser)]
#[command(
    name = "animus-manager",
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
    /// Animus must be asleep (not processing inputs).
    Name,

    /// Retrieve the version of the Animus.
    Version,

    /// List the names of all Structures in the Complex handled by the Animus.
    /// Animus must be asleep (not processing inputs).
    ListStructures,

    /// Save the state of the Complex to the associated network file.
    /// Animus must be asleep (not processing inputs).
    Save,

    /// Begin processing inputs for the Animus.
    /// Animus must be asleep (not processing inputs).
    Wake,

    /// Stops the Animus from processing new input signals.
    Sleep,

    /// Reports whether the Animus is awake or asleep.
    Status,

    /// Shut down the Animus.
    Terminate,

    /// Return to the Brainstorm REPL.
    Back,
}

impl crate::Brainstorm {

    // Spawns an inner REPL for sending animus commands.
    pub(super) fn animus_repl(&self, animus: &str) {

        println!("Selected animus '{}'", animus);

        // Set the prompt appearance
        let prompt = DefaultPrompt {
            left_prompt: DefaultPromptSegment::Basic(animus.to_owned()),
            ..DefaultPrompt::default()
        };

        let mut inner_repl = ClapEditor::<AnimusManagerCli>::builder()
            .with_prompt(Box::new(prompt))
            .build();

        // Execute commands:
        loop { match inner_repl.read_command() {
            ReadCommandOutput::Command(cli) => match cli.command {
                
                AnimusCommand::Back => { break },

                AnimusCommand::Name => {
                    self.handle_command(animus, Action::Name)
                },

                AnimusCommand::Version => {
                    self.handle_command(animus, Action::Version)
                },

                AnimusCommand::ListStructures => {
                    self.handle_command(animus, Action::ListStructures)
                },

                AnimusCommand::Save => {
                    println!("Saving network state, please wait...");
                    self.handle_command(animus, Action::Save)
                },

                AnimusCommand::Wake => {
                    self.handle_command(animus, Action::Wake)
                },

                AnimusCommand::Sleep => {
                    self.handle_command(animus, Action::Sleep)
                },

                AnimusCommand::Status => {
                    self.handle_command(animus, Action::Status)
                },

                AnimusCommand::Terminate => {
                    self.handle_command(animus, Action::Terminate)
                },
            },

            ReadCommandOutput::EmptyLine => {/* Continue */},
            ReadCommandOutput::ClapError(e) => { println!{"{}", e}},
            ReadCommandOutput::ReedlineError(e) => { println!{"{}", e}},
            ReadCommandOutput::ShlexError => { println!{"Bad syntax"}},

            _ => {/* Continue */}

        }}
    }

    pub(crate) fn handle_command(&self, animus: &str, action: Action) {

        if let Err(e) = self.send_command(animus, action.clone()) {
            Self::animus_command_error(animus, e);
        } else {
            if let Err(e) = self.share_response() {
                Self::animus_response_error(animus, action, e)
            }
        }
    }

    // Log and display an error that occurred while sending an animus command.
    pub(crate) fn animus_command_error(animus: &str, e: anyhow::Error) {

        println!("ERROR: Command to '{}' was not sent properly.", animus);
        eprintln!("{}", e);
    }

    // Log and display an error that occurred while awaiting an animus response.
    pub(crate) fn animus_response_error(
        animus: &str, 
        action: Action,
        e: anyhow::Error
    ) {

        println!(
            "ERROR: Socket timed out while waiting for response from '{}'...\n\
            Command '{}' may not have executed properly.", 
            animus, action
        );
        eprintln!("{}", e);
    }
}

