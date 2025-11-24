
mod file;
mod repl;

use clap::Parser;
use ezcfg::Config;

/// Run `$ brainstorm` to launch the control REPL.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    /// Run with this flag to set up the necessary directories.
    #[arg(long, short, action)]
    setup: bool,

    /// Add this to --setup to immediately launch brainstorm. Unnecessary otherwise.
    #[arg(long, short, action)]
    run: bool,
}

fn main() {

    let args = Args::parse();

    if args.setup == true {

        // Set up the directory structure:
        if let Err(e) = file::setup::directory_setup() {
            println!("Error creating framework directory: {}", e);
            return
        }

        println!("Cajal setup complete");

        if args.run == false { return }
    }

    if file::setup::setup_ok() {

        let config = match file::cfg::BrainstormConfig::read() {
            Ok(config) => config,
            Err(_) => file::cfg::BrainstormConfig::default(),
        };

        repl::run(config)

    } else {
        println!("Missing `.cajal` directories. Run `$ brainstorm --setup`.");
    }

}

