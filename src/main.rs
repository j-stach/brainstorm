
mod setup;
mod repl;

fn main() {

    // Set up the directory structure:
    if let Err(e) = setup::directory_setup() {
        println!("Error creating framework directory: {}", e);
        return
    }

    // Run the loop:
    repl::brainstorm_repl();
}

