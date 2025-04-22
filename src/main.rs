
mod setup;
mod repl;

fn main() {
    // Set up the directory structure:
    let _ = setup::directory_setup(); // TODO: Errors
    // Run the loop:
    repl::brainstorm_repl();
}

