
mod file;
//mod repl;

mod meta;
mod animus;
mod group;

use clap::Parser;
use ezcfg::Config;

// Singleton handling global program resources
pub(crate) struct Brainstorm {
    socket: std::net::UdpSocket,
} 

impl Brainstorm {
    fn new(_config: file::cfg::BrainstormConfig) -> anyhow::Result<Self> {

        Ok(Brainstorm {
            socket: std::net::UdpSocket::bind("127.0.0.1:4048")?,
        })
    }
}

/// Run `$ brainstorm` to launch the control REPL.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    /// Run with this flag to set up the necessary directories.
    #[arg(long, short, action)]
    setup: bool,

    /// Add this to --setup to immediately launch brainstorm. 
    /// Unnecessary otherwise.
    #[arg(long, short, action)]
    run: bool,
}

fn main() {

    let args = Args::parse();

    if args.setup == true {

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

        match Brainstorm::new(config) {
            Ok(brainstorm) => brainstorm.meta_manager(),
            Err(e) => eprintln!("{}", e),
        }

    } else {
        println!("Missing `.cajal` directories. Run `$ brainstorm --setup`.");
    }

}

