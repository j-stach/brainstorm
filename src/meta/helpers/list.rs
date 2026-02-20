
// Lists all animi
pub(crate) fn active_animi() -> anyhow::Result<()> {

    // TODO Send Query AnimusCommand with no name, wait 1 sec to collect responses
    // print list of all that respond
    Ok(())
}

// Print a list of all files that appear in the `animi` directory.
pub(crate) fn all_animi() -> anyhow::Result<()> {

    println!("Local animi:");
    let local_animi = crate::file::animi::read_local_animi()?.flatten();
    for animus in local_animi {
        let name = animus.file_name().into_string()
            .map_err(|_| anyhow::anyhow!("Couldn't read OsString"))?;
        println!("{}", name) 
    }

    println!("Remote animi:");
    let remote_animi = crate::file::animi::read_remote_animi()?.flatten();
    for animus in remote_animi {
        let name = animus.file_name().into_string()
            .map_err(|_| anyhow::anyhow!("Couldn't read OsString"))?;
        println!("{}", name) 
    }

    Ok(())
}

// Print a list of all network files that appear in the `saved` directory.
pub(crate) fn saved_networks() -> anyhow::Result<()> {

    let saved = crate::file::saved::read_saved()?.flatten();
    for network in saved {
        let name = network.file_name().into_string()
            .map_err(|_| anyhow::anyhow!("Couldn't read OsString"))?;
        if name.ends_with(".nn") {
            println!("{}", name) 
        }
    }

    Ok(())
}

