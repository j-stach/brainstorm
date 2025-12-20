
use animusd_lib::protocol::{ Command, Action, Report, Outcome };

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

        // TODO Ensure that "Listen" loop does not interfere, by checking name?
        match report.outcome {
            Outcome::Success => Ok(true),
            _ => Ok(false),
        }
    }

    // Send command to associated IP address @ port 4048.
    // TODO: Ensure animusd listens to the correct IP address for commands,
    // TODO: or else, ensure that different IP can be linked thru brainstorm.
    // Returns an error if the network connection could not be established.
    pub(super) fn send_command(&self, animus: &str, action: Action) -> anyhow::Result<()> {

        let command = Command::new(animus, action);
        self.socket.send(&command.encode()?)?;

        Ok(())
    }

    // Log and display an error that occurred while sending an animus command.
    pub(super) fn animus_command_error(animus: &str, e: anyhow::Error) {

        println!("ERROR: Command to '{}' was not sent properly.", animus);
        eprintln!("{}", e);
    }
}
