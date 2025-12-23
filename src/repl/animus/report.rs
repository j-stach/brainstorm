
use animusd_lib::protocol::{ Action, Report };

impl crate::Brainstorm {

    #[allow(dead_code)]
    pub(crate) fn share_response(&self) -> anyhow::Result<()> {
        Ok(Self::print_report(self.read_report()?))
    }
    
    pub(crate) fn read_report(&self) -> anyhow::Result<Report> {

        let mut buf = [0; 1023];
        let (len, _) = self.socket.recv_from(&mut buf)?;
        let report = Report::decode(&buf[..len])?;

        Ok(report)
    }

    pub(crate) fn print_report(report: Report) {

        println!{
            "{} ({}): {}",
            report.name,
            report.action,
            report.outcome,
        }
    }

    // Log and display an error that occurred while awaiting an animus response.
    pub(super) fn animus_response_error(
        animus: &str, 
        action: Action,
        e: anyhow::Error
    ) {

        println!(
            "ERROR: Socket timed out while waiting for response from '{}'...\n\
            Command '{}' may not have executed properly.", 
            animus, action
        );
        eprintln!("{}", e);
    }
}

