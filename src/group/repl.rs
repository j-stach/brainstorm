
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
struct GroupCli {
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

    pub(crate) fn group_manager(&self, group: &str) {

        // TODO move to create_group function
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

        let repl = Self::group_repl_setup(group);
        self.execute_group_commands(repl, group)
    }

    fn group_repl_setup(group: &str) -> ClapEditor<GroupCli> {

        // Set the prompt appearance
        let prompt = DefaultPrompt {
            left_prompt: DefaultPromptSegment::Basic(group.to_owned()),
            ..DefaultPrompt::default()
        };

        // Create the REPL environment
        let group_repl = ClapEditor::<GroupCli>::builder()
            .with_prompt(Box::new(prompt))
            .build();

        group_repl
    }

    fn execute_group_commands(&self, mut repl: ClapEditor<GroupCli>, group: &str) {

        // TODO Update to use same pattern as meta
        loop { match repl.read_command() {
            ReadCommandOutput::Command(cli) => match cli.command {
                
                GroupCommand::Back => { break },

                GroupCommand::ListMembers => Self::list_group_members(group),

                GroupCommand::Wake => self.group_action(group, Action::Wake),
                GroupCommand::Sleep => self.group_action(group, Action::Sleep),
                GroupCommand::Status => self.group_action(group, Action::Status),
                GroupCommand::Query => self.group_action(group, Action::Query),

                GroupCommand::AutoLink => self.group_attempt_autolink(group),

                GroupCommand::Add { animus } => Self::group_add_animus(group, &animus),
                GroupCommand::Remove { animus } => Self::group_remove_animus(group, &animus),
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
    fn list_group_members(group: &str) {
        match crate::file::groups::read_group_members(group) {
            Err(e) => eprintln!("{}", e),
            Ok(list) => {
                for member in list {
                    println!("{}", member)
                }
            }
        }
    }

    //
    fn group_add_animus(group: &str, animus: &str) {
        // TODO if animus doesn't exist, err
        if let Err(e) = crate::file::groups::group_add_animus(group, &animus) {
            eprintln!("{}", e)
        }
    }

    fn group_remove_animus(group: &str, animus: &str) {
        if let Err(e) = crate::file::groups::group_remove_animus(group, &animus) {
            eprintln!("{}", e)
        }
    }

}

