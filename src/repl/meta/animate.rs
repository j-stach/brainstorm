
//! Generate animi for saved networks

impl crate::Brainstorm {

    // Create a new animus setup for the given network.
    pub(super)fn animate_network(
        &self, 
        network_name: &str,
    ) -> anyhow::Result<()> {

        if crate::file::saved::network_exists(network_name)? {
            let animus_name = self.animus_setup(network_name)?;
            println!("{} was animated as {}", network_name,  animus_name);
        } else {
            println!(
                "Network '{}' not found! \n\
                (TIP: Use `list-networks`)", 
                network_name
            );
        }
        
        Ok(())
    }

    // Set up a new animus directory, executable, and all necessary files.
    fn animus_setup(&self, network_name: &str) -> anyhow::Result<String> {

        let network_path = crate::file::saved::network_path(network_name);
        let network_path = std::path::Path::new(&network_path);

        if ! network_path.exists() {
            println!(
                "Network '{}' not found: No networks were animated. \n\
                (Tip: Use `list-networks` to view saved networks)",
                network_name
            )
        }

        // Rename the animus if necessary or desired
        let animus_name = self.rename_animus(network_name.to_string())?;

        if animus_name.is_none() {
            // TODO return Ok(None)
        }

        // Safe because we check for it above
        let animus_name = animus_name.expect("Animus name exists");

        // Create animus directory
        let animus_dir = crate::file::animi::animus_dir(&animus_name);
        let animus_path = std::path::Path::new(&animus_dir);
        if !animus_path.is_dir() {
            std::fs::create_dir(animus_path)?
        }

        Self::build_animus(&animus_name)?;

        Ok(animus_name)
    }

    // Check if the proposed animus name is valid, then rename it if necessary.
    fn rename_animus(&self, name: String) -> anyhow::Result<Option<String>> {

        // If the animus name is blank, exit without choosing a name
        if name.is_empty() {
            println!("No name chosen.");
            return Ok(None)
        }

        let mut required = false;

        if ! crate::file::animi::valid_animus_name(&name) {
            println!("Name '{}' is invalid! (Use a-Z, 0-9, and _)", &name);
            required = true
        }

        if self.is_active(&name)? {
            println!("An animus named '{}' is already active!", &name);
            required = true
        } 

        // Ask if rename desired 
        if ! required { loop {
            print!("Name '{}' is valid. Use? (Y/n): ", &name);

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let input = input.trim();

            match input {
                "Y" | "y" | "" => { return Ok(Some(name)) },
                     "N" | "n" => { break },
                             _ => {/* Continue loop */},
            }
        }}

        println!("Type a new name or sumbit an empty line to cancel.");
        print!("New name: ");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let new_name = input.trim().to_string();

        self.rename_animus(new_name)
    }

    // Install a unique `animusd` executable for an animus, based on features
    fn build_animus(animus_name: &str) -> anyhow::Result<()> {

        // Build the animusd executable with the features specified.
        let animus_dir = crate::file::animi::animus_dir(&animus_name);

        let mut cmd = std::process::Command::new("cargo");
        cmd.arg("install")
            .arg("animusd")
            // TODO Logging enabled, and other features
            .arg("--features=animus")
            .arg("--root")
            .arg(&animus_dir);

        let result = cmd.output()?;
        if ! result.status.success() {
            // TODO return Err
        }

        // Rename the service executable to distinguish it as a process
        let default_path = format!("{}/bin/animusd", &animus_dir);
        let bin_path = format!("{}-{}", &default_path, &animus_name);

        let mut cmd = std::process::Command::new("mv");
        cmd.arg(&default_path).arg(&bin_path);

        let result = cmd.output()?;
        if ! result.status.success() {
            // TODO return Err
        }

        // Make animusd executable
        let mut cmd = std::process::Command::new("chmod");
        cmd.arg("+x").arg(&bin_path);

        let result = cmd.output()?;
        if ! result.status.success() {
            // TODO return Err
        }

        Ok(())
    }
}

