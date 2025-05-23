
// TODO: Errors

use clap::{ Parser, Subcommand };
use clap_repl::{ ClapEditor, ReadCommandOutput };
use clap_repl::reedline::{ DefaultPrompt, DefaultPromptSegment };

use std::path::Path;


#[derive(Parser)]
#[command(
    name = "animus-config",
    about = "REPL for configuring Animus services and networks",
    long_about = "This sub-REPL is used to configure an Animus before launch",
)]
struct AnimusConfigCli {
    #[command(subcommand)]
    config: AnimusConfig,
}

#[derive(Subcommand, Debug)]
enum AnimusConfig {

    // TODO: Build out animusd and cajal-lib with features, then come back to this.
    // Logging
    // OrderOnly
    // ???

    /// Return to the parent Brainstorm REPL.
    Return,

    Mindreader {
        #[arg(
            help = ("")
        )]
        add: String
    },

}


// Spawns an inner REPL for creating an animus configuration.
// WARN: Expects that `animus_name` contains only valid characters.
pub(crate) fn animus_config_repl(animus_name: &str) {

    println!("Configuring animus '{}'...", animus_name);

    let animus_root = format!("~/.brainstorm/animi/{}", animus_name);
    let animus_path = Path::new(&animus_root);
    let config_path = animus_path.join("config.toml");

    let mut ip = String::new();
    let mut mindreader = Vec::new();
    let mut features = Vec::new();

    let prompt = format!("{}/config", animus_name);
    let prompt = DefaultPrompt {
        left_prompt: DefaultPromptSegment::Basic(prompt),
        ..DefaultPrompt::default()
    };

    let mut inner_repl = ClapEditor::<AnimusConfigCli>::builder()
        .with_prompt(Box::new(prompt))
        .build();

    let mut run_config = true;

    // Logic for configuration process:
    while run_config {
        match inner_repl.read_command() {
            ReadCommandOutput::Command(cli) => match cli.config {

                AnimusConfig::Return => {
                    run_config = false
                },

                AnimusConfig::Mindreader { add } => {
                    add_mindreader_layer(add)
                },           
            },

            ReadCommandOutput::EmptyLine => { /* Do nothing */ },
            ReadCommandOutput::ClapError(e) => { println!{"{}", e}},
            ReadCommandOutput::ReedlineError(e) => { println!{"{}", e}},
            ReadCommandOutput::ShlexError => { println!{"Bad input syntax"}},

            _ => {}

        }
    }

    animusd_lib::config::generate_config_from_values(
        config_path,
        animus_name,
        animusd_lib::VERSION,
        ip,
        mindreader,
        features
    )
}


fn add_mindreader_layer(layer_name: &str, mindreader: Vec<String>) {
    // TODO: Check mindreader layer eligibility
    // Add layer to mindreader string
}

