
mod setup;
mod repl;

fn main() {

    // TODO Using clap, check if there is a `--setup` flag
    // Also provide for `--version` and `--help` flags
    // And a `--run` flag that opens Brainstorm to print result & use immediate

    // TODO Instead of setting up automatically, check that it exists--
    // if not, prompt the user to re-run `brainstorm --setup` first
    /*
    // Set up the directory structure:
    if let Err(e) = setup::directory_setup() {
        println!("Error creating framework directory: {}", e);
        return
    }
    */

    // Run the loop:
    repl::run();
}

