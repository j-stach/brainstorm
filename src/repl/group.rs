
//! This allows animi to be grouped and controlled together.

use clap::{ Parser, Subcommand };
use clap_repl::{ ClapEditor, ReadCommandOutput };
use clap_repl::reedline::{ DefaultPrompt, DefaultPromptSegment };

use animusd_lib::protocol::Action;


#[derive(Parser)]
#[command(
    name = "group-manager",
    about = "REPL for managing Animi when grouped in a System",
    long_about = "This sub-REPL is used to manage a group (System) of Animi",
)]
struct GroupManagerCli {
    #[command(subcommand)]
    command: GroupCommand,
}

#[derive(Subcommand, Debug)]
enum GroupCommand {

    /// List all Animi that are members of this group.
    ListMembers,

    /// Add an animus to the system group, if not already present.
    /// Must be findable with `list-all` using the top level REPL.
    Add { 
        #[arg( help = 
            "Provide the name of the Animus as it appears in the filesystem. \
            Brainstorm will search for the animus in ~/.cajal/animi. \n\
            View Animi using the `list-all` command in the top level REPL."
        )]
        animus: String 
    },

    /// Remove an animus from the system, if present.
    Remove { 
        #[arg( help = 
            "Provide the name of the Animus as it appears in the group registry."
        )]
        animus: String 
    },

    /// Begin processing inputs for all animi in the system.
    Wake,

    /// Stop processing inputs for all animi in the system.
    Sleep,

    /// Get the status of each animus in the system.
    Status,

    /// Query each listed animus to determine if they are all present.
    Query,

    /// Automatically link tracts for all networks in the system.
    /// Will fail if there are duplicate tract names, 
    /// and will report a warning if any tracts are not paired
    /// (e.g., left open for Sensor/Motor IO).
    AutoLink,

    /// Return to the Brainstorm REPL.
    Back,
}

impl crate::Brainstorm {

    // Spawns an inner REPL for sending goup commands.
    pub(super) fn group_repl(&self, group: &str) {

        match crate::file::groups::group_exists(group) {
            Ok(exists) => {
                if !exists {
                    // TODO ask if user wants to create Y/N
                    // otherwise return to top repl
                }
            },
            Err(e) => {
                eprintln!("An error occurred while reading groups dir: {}", e);
                return
            }
        }

        println!("Selected group '{}'", group);

        // Set the prompt appearance
        let prompt = DefaultPrompt {
            left_prompt: DefaultPromptSegment::Basic(group.to_owned()),
            ..DefaultPrompt::default()
        };

        let mut inner_repl = ClapEditor::<GroupManagerCli>::builder()
            .with_prompt(Box::new(prompt))
            .build();

        // Execute commands:
        loop { match inner_repl.read_command() {
            ReadCommandOutput::Command(cli) => match cli.command {
                
                GroupCommand::Back => { break },

                GroupCommand::Wake => {
                    self.group_action(group, Action::Wake)
                },

                GroupCommand::Sleep => {
                    self.group_action(group, Action::Sleep)
                },

                GroupCommand::Status => {
                    self.group_action(group, Action::Status)
                },

                GroupCommand::Query => {
                    self.group_action(group, Action::Query)
                },

                GroupCommand::AutoLink => {
                    self.group_attempt_autolink(group);
                },

                GroupCommand::ListMembers => {
                    match crate::file::groups::read_group_members(group) {
                        Err(e) => eprintln!("{}", e),
                        Ok(list) => {
                            for member in list {
                                println!("{}", member)
                            }
                        }
                    }
                },

                GroupCommand::Add { animus } => {
                    // TODO if animus doesn't exist, err
                    if let Err(e) = crate::file::groups::group_add_animus(group, &animus) {
                        eprintln!("{}", e)
                    }
                },

                GroupCommand::Remove { animus } => {
                    if let Err(e) = crate::file::groups::group_remove_animus(group, &animus) {
                        eprintln!("{}", e)
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

    // Send an animus command to each animus in the group.
    fn group_action(&self, group: &str, action: Action) {

        let read = crate::file::groups::read_group_members(group);
        if let Err(e) = &read {
            return println!("Group file for '{}' is corrupted or missing: {}", group, e)
        }

        let members = read.unwrap();
        for animus in members.iter() {
            self.handle_command(animus, action.clone())
        }
    }

    // 
    fn group_attempt_autolink(&self, group: &str) {

        use std::collections::HashMap;
        use cajal_cx::tract::receiver::ReceiverInfo;
        use animusd_lib::protocol::{ Action, Outcome };

        let read = crate::file::groups::read_group_members(group);
        if let Err(e) = &read {
            return println!("Group file for '{}' is corrupted or missing: {}", group, e)
        }
        let members = read.unwrap();

        // Check if all animi are active (accepting commands)
        // and asleep (not processing inputs)
        for animus in members.iter() {
            match self.is_active(animus) {
                Ok(active) => {
                    if !active {
                        return println!(
                            "Animus '{}' is not active. \
                            Please activate all animi in the group '{}' before linking tracts.",
                            animus,
                            group
                        )
                    }
                },
                Err(e) => {
                    return Self::animus_command_error(animus, e)
                }
            }

            match self.is_awake(animus) {
                Ok(awake) => {
                    if awake {
                        return println!(
                            "Animus '{}' is processing inputs. \
                            Please sleep all animi in the group '{}' before linking tracts.",
                            animus,
                            group
                        )
                    }
                },
                Err(e) => {
                    return Self::animus_command_error(animus, e)
                }
            }
        }
        
        // Gather Input and Output information from each animus:
        // tract_name -> (animus_name, ReceiverInfo)
        let mut receivers: HashMap<String, (String, ReceiverInfo)> = HashMap::new();
        // tract_name -> animus_name
        let mut senders: HashMap<String, String> = HashMap::new();

        for animus in members.iter() {

            let action = Action::ReportInputs;

            // Collect all Inputs
            if let Err(e) = self.send_command(animus, action.clone()) {
                return Self::animus_command_error(animus, e)
            } else {
                match self.read_report() {
                    Err(e) => return Self::animus_response_error(animus, action, e),
                    Ok(report) => {

                        match report.outcome {
                            Outcome::Return(msg) => {

                                let msg = bincode::deserialize(&msg);
                                if let Err(e) = &msg {
                                    return println!(
                                        "Failed to deserialize a Report's Outcome::Return data. \
                                        Check the version of `animusd-lib` you are using. \
                                        {}", e
                                    )
                                }
                                let list: Vec<ReceiverInfo> = msg.unwrap();

                                for info in list.iter() {
                                    let tract_name = &info.tract_name;
                                    if let Some(..) = receivers.insert(
                                        tract_name.clone(), 
                                        (animus.clone(), info.clone())
                                    ) {
                                        return println!(
                                            "ERROR: Aborting auto-link: Duplicate of Input '{}' found",
                                            tract_name
                                        )
                                    }
                                }
                            },
                            _ => {
                                return println!(
                                    "ERROR: Unexpected Outcome violates protocol. \
                                    Check the version of `animusd-lib` you are using."
                                )
                            }
                        }
                    }
                }
            }

            let action = Action::ListOutputs;

            // Collect all Outputs
            if let Err(e) = self.send_command(animus, action.clone()) {
                return Self::animus_command_error(animus, e)
            } else {
                match self.read_report() {
                    Err(e) => return Self::animus_response_error(animus, action, e),
                    Ok(report) => {

                        match report.outcome {
                            Outcome::Return(msg) => {

                                let msg = bincode::deserialize(&msg);
                                if let Err(e) = &msg {
                                    return println!(
                                        "Failed to deserialize a Report's Outcome::Return data. \
                                        Check the version of `animusd-lib` you are using. \
                                        {}", e
                                    )
                                }
                                let list: Vec<String> = msg.unwrap();

                                for output in list.iter() {
                                    if let Some(..) = senders.insert(
                                        output.clone(), animus.clone() 
                                    ) {
                                        return println!(
                                            "ERROR: Aborting auto-link: Duplicate of Output '{}' found",
                                            output
                                        )
                                    }
                                }
                            },
                            _ => {
                                return println!(
                                    "ERROR: Unexpected Outcome violates protocol. \
                                    Check the version of `animusd-lib` you are using."
                                )
                            }
                        }

                    }
                }
            }
        }

        // Attempt to link using collected Inputs and Outputs
        for (tract_name, animus) in senders.clone().iter() {
            if let Some((_, info)) = receivers.get(tract_name) {
                let action = Action::LinkOutput(info.clone());
                if let Err(e) = self.send_command(animus, action.clone()) {
                    Self::animus_command_error(animus, e)
                } else {
                    receivers.remove(tract_name);
                    senders.remove(tract_name);
                }
            }
        }

        // Report unlinked tracts:
        if !senders.is_empty() {
            println!("NOTE -- Some Ouputs were not linked (these may go to Motors):");
            for output in senders.iter() {
                // "animus_name: tract_name"
                println!("{}: {}", output.1, output.0)
            }
        }

        if !receivers.is_empty() {
            println!("NOTE -- Some Inputs were not linked (these may come from Sensors):");
            for input in receivers.iter() {
                // "animus_name: tract_name"
                println!("{}: {}", input.1.0, input.0)
            }
        }

    }
}

