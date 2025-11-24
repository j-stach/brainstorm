
//! Commands for retrieving information

// Lists all animi
pub(super) fn active_animi() -> anyhow::Result<()> {

    // TODO Send Query AnimusCommand with no name, wait 1 sec to collect responses
    // print list of all that respond
    Ok(())
}

// Print a list of all files that appear in the `animi` directory.
pub(super) fn all_animi() -> anyhow::Result<()> {

    let animi = crate::file::animi::read_animi()?.flatten();
    for animus in animi {
        let name = animus.file_name().into_string()
            .map_err(|_| anyhow::anyhow!("Couldn't read OsString"))?;
        println!("{}", name) 
    }

    Ok(())
}

// Print a list of all network files that appear in the `saved` directory.
pub(super) fn saved_networks() -> anyhow::Result<()> {

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


