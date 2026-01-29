
use animusd_lib::protocol::Report;

impl crate::Brainstorm {

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
}

