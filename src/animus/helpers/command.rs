
use animusd_lib::protocol::{ Command, Action, Report, Outcome };

use crate::file::{ animi::*, remote::* };

impl crate::Brainstorm {

    // Check if an animus is currently active by pinging for its version number.
    pub(crate) fn is_active(&self, animus: &str) -> anyhow::Result<bool> {

        if ! crate::file::animi::valid_animus_name(animus) {
            return Err(anyhow::anyhow!("'{}' is an invalid name.", animus))
        }

        self.send_command(animus, Action::Query)?;

        let mut buf = [0; 1023];
        let (len, _) = self.socket.recv_from(&mut buf)?;
        let report = Report::decode(&buf[..len])?;

        // TODO: If no response (timeout), interpret as false?

        match report.outcome {
            Outcome::Success => Ok(true),
            _ => Ok(false),
        }
    }

    // Check if an animus is currently active by pinging for its version number.
    pub(crate) fn is_awake(&self, animus: &str) -> anyhow::Result<bool> {

        if ! crate::file::animi::valid_animus_name(animus) {
            return Err(anyhow::anyhow!("'{}' is an invalid name.", animus))
        }

        self.send_command(animus, Action::Status)?;

        let mut buf = [0; 1023];
        let (len, _) = self.socket.recv_from(&mut buf)?;
        let report = Report::decode(&buf[..len])?;

        match report.outcome {
            Outcome::Success => Ok(true),
            _ => Ok(false),
        }
    }

    // Send command to local or remote animus
    pub(crate) fn send_command(&self, animus: &str, action: Action) -> anyhow::Result<()> {

        if local_animus_exists(animus)? {
            self.send_local_command(animus, action)
        } else if remote_animus_exists(animus)? {
            self.send_remote_command(animus, action)
        } else {
            Err(anyhow::anyhow!("Animus '{}' does not exist", animus))
        }
    }

    // Returns an error if the network connection could not be established.
    pub(crate) fn send_local_command(&self, animus: &str, action: Action) -> anyhow::Result<()> {

        let command = Command::new(animus, action);
        self.socket.send(&command.encode()?)?;

        Ok(())
    }

    // Send command to associated IP address @ port 4048.
    pub(crate) fn send_remote_command(&self, animus: &str, action: Action) -> anyhow::Result<()> {

        let ip_addr = remote_animus_ip(animus)?;
        let remote = std::net::SocketAddr::new(ip_addr, 4048);

        let command = Command::new(animus, action);
        self.socket.send_to(&command.encode()?, remote)?;

        // animus recieves using recv_from
        // animus reports to this socket's IP

        Ok(())
    }

}
