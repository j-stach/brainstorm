
pub(crate) fn remote_animus_path(name: &str) -> String {
    format!("~/.cajal/animi/remote/{}", name)
}

pub(crate) fn remote_animus_ip(name: &str) -> anyhow::Result<std::net::IpAddr> {
    let path = remote_animus_path(name);
    match std::fs::read_to_string(path) {
        Ok(addr) => {
            match addr.parse::<std::net::IpAddr>() {
                Ok(addr) => {
                    Ok(addr)
                },
                Err(e) => {
                    Err(anyhow::anyhow!("{}", e))
                }
            }
        },
        Err(e) => {
            Err(anyhow::anyhow!("{}", e))
        }
    } 
}

pub(crate) fn write_remote_animus(name: &str, addr: std::net::IpAddr) -> anyhow::Result<()> {
    let path = remote_animus_path(name);
    std::fs::write(path, addr.to_string())?;
    Ok(())
}


