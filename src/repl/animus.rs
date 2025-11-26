
//! This aligns Brainstorm with the capabilities of the `animusd` protocol.

use clap::{ Parser, Subcommand };
use clap_repl::{ ClapEditor, ReadCommandOutput };
use clap_repl::reedline::{ DefaultPrompt, DefaultPromptSegment };

use animusd_lib::protocol::{ Command, Action, Response, Outcome };


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
    pub(super) fn command_repl(&self, animus_name: &str) {

        println!("Selected animus '{}'", animus_name);

        // Set the prompt appearance
        let prompt = DefaultPrompt {
            left_prompt: DefaultPromptSegment::Basic(animus_name.to_owned()),
            ..DefaultPrompt::default()
        };

        let mut inner_repl = ClapEditor::<AnimusManagerCli>::builder()
            .with_prompt(Box::new(prompt))
            .build();

        // Execute commands:
        loop {

            match inner_repl.read_command() {
                ReadCommandOutput::Command(cli) => match cli.command {
                    
                    AnimusCommand::Back => { break },

                    AnimusCommand::Name => {
                        self.command_animus(animus_name, Action::Name)
                    },

                    AnimusCommand::Version => {
                        self.command_animus(animus_name, Action::Version)
                    },

                    // Print the list of structures in the complex
                    AnimusCommand::ListStructures => {

                        let response = self.send_command(
                            animus_name, 
                            Action::ListStructures,
                        );

                        match response {

                            Ok(r) => {
                                // TODO
                                /*
                                if let Some(csv) = response.unwrap() {
                                    // Structures will be listed as CSV string,
                                    // Needs to be split and listed vertically.
                                    let values: Vec<_> = csv.split(",")
                                        .collect();
                                    for value in values {
                                        println!("{}", value);
                                    }
                                } else {
                                    println!(
                                        "No response from animus {}", 
                                        animus_name
                                    );
                                }
                                */
                            },

                            Err(e) => Self::report_command_error(animus_name, e)
                        }

                    },

                    AnimusCommand::Save => {
                        println!("Saving network state...");
                        self.command_animus(animus_name, Action::Save)
                    },

                    AnimusCommand::Wake => {
                        self.command_animus(animus_name, Action::Wake)
                    },

                    AnimusCommand::Sleep => {
                        self.command_animus(animus_name, Action::Sleep)
                    },
                    
                    AnimusCommand::Status => {
                        self.command_animus(animus_name, Action::Status)
                    },

                    AnimusCommand::Terminate => {
                        self.command_animus(animus_name, Action::Terminate)
                    },
                },

                ReadCommandOutput::EmptyLine => {/* Continue */},
                ReadCommandOutput::ClapError(e) => { println!{"{}", e}},
                ReadCommandOutput::ReedlineError(e) => { println!{"{}", e}},
                ReadCommandOutput::ShlexError => { println!{"Bad syntax"}},

                _ => {/* Continue */}

            }

            // Don't hog CPU!
            // TODO TBD Necessary?
            //std::thread::yield_now();
        }

    }

    // Check if an animus is currently active by pinging for its version number.
    pub(crate) fn is_active(&self, animus_name: &str) -> anyhow::Result<bool> {

        if !crate::file::animi::valid_animus_name(animus_name) {
            // TODO Err
        }

        let response = self.send_command(animus_name, Action::Query)?;

        match response {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    // Send an animus action as a command, then process the response.
    fn command_animus(&self, animus_name: &str, action: Action) {
        
        let response = self.send_command(animus_name, action);
        match response {
            Ok(r) => Self::report_response(animus_name, r),
            Err(e) => Self::report_command_error(animus_name, e)
        }
    }

    // Send command to associated IP address @ port 4048.
    // Get any animus response and parse results to string.
    // Returns an error if the network connection could not be established.
    fn send_command(
        &self,
        animus_name: &str, 
        action: Action 
    ) -> anyhow::Result<Option<Response>> {

        let command = Command::new(animus_name, action);
        self.socket.send(&command.encode()?)?;

        let mut buf = [0; 256]; // TODO TBD Big enough?
        if let Ok((len, src)) = self.socket.recv_from(&mut buf) {
            let response = Response::decode(&buf)?;
            Ok(Some(response))
        } else {
            Ok(None)
        }

    }

    // Unpack the response received by send_animus_command and print it.
    fn report_response(animus_name: &str, response: Option<Response>) {

        if let Some(response) = response {
            //response.name
            // TODO println!("{}", value);
            // animus, action, outcome
        } else {
            println!("No response from animus '{}'", animus_name);
        }
    }

    // Log and display an error that occurred while sending an animus command.
    fn report_command_error(animus_name: &str, e: anyhow::Error) {

        println!("WARN: An error occurred: Command was not sent properly.");
        eprintln!("{}", e);
    }
}

