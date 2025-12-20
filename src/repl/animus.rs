
//! This aligns Brainstorm with the capabilities of the `animusd` protocol.

mod command;
mod report;


use clap::{ Parser, Subcommand };
use clap_repl::{ ClapEditor, ReadCommandOutput };
use clap_repl::reedline::{ DefaultPrompt, DefaultPromptSegment };

use animusd_lib::protocol::Action;


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

        // TODO Start independent loop to listen for reports

        // Execute commands:
        loop { match inner_repl.read_command() {
            ReadCommandOutput::Command(cli) => match cli.command {
                
                AnimusCommand::Back => { break },

                AnimusCommand::Name => {
                    if let Err(e) = self.send_command(animus, Action::Name) {
                        Self::animus_command_error(animus, e)
                    }
                },

                AnimusCommand::Version => {
                    if let Err(e) = self.send_command(animus, Action::Version) {
                        Self::animus_command_error(animus, e)
                    }
                },

                AnimusCommand::ListStructures => {
                    
                    if let Err(e) = self.send_command(
                        animus, 
                        Action::ListStructures,
                    ) {
                        Self::animus_command_error(animus, e)
                    }
                },

                AnimusCommand::Save => {
                    println!("Saving network state...");
                    if let Err(e) = self.send_command(animus, Action::Save) {
                        Self::animus_command_error(animus, e)
                    }
                },

                AnimusCommand::Wake => {
                    if let Err(e) = self.send_command(animus, Action::Wake) {
                        Self::animus_command_error(animus, e)
                    }
                },

                AnimusCommand::Sleep => {
                    if let Err(e) = self.send_command(animus, Action::Sleep) {
                        Self::animus_command_error(animus, e)
                    }
                },

                AnimusCommand::Status => {
                    if let Err(e) = self.send_command(animus, Action::Status) {
                        Self::animus_command_error(animus, e)
                    }
                },

                AnimusCommand::Terminate => {
                    if let Err(e) = self.send_command(
                        animus, 
                        Action::Terminate
                    ) {
                        Self::animus_command_error(animus, e)
                    }
                },
            },

            ReadCommandOutput::EmptyLine => {/* Continue */},
            ReadCommandOutput::ClapError(e) => { println!{"{}", e}},
            ReadCommandOutput::ReedlineError(e) => { println!{"{}", e}},
            ReadCommandOutput::ShlexError => { println!{"Bad syntax"}},

            _ => {/* Continue */}

        }}
    }
}

