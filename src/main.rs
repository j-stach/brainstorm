
mod file;

mod meta;
mod animus;
mod group;

use clap::Parser;

// Singleton handling global program resources
pub(crate) struct Brainstorm {
    socket: std::net::UdpSocket,
} 

impl Brainstorm {
    fn new() -> anyhow::Result<Self> {

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

        match Brainstorm::new() {
            Ok(brainstorm) => brainstorm.meta_manager(),
            Err(e) => eprintln!("{}", e),
        }

    } else {
        println!("Missing `.cajal` directories. Run `$ brainstorm --setup`.");
    }

}

