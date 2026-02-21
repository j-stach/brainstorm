
use std::collections::HashMap;
use cajal_cx::tract::receiver::ReceiverInfo;
use animusd_lib::protocol::{ Action, Outcome };


impl crate::Brainstorm {

    // Attempt to link matching tracts within a group
    pub(crate) fn group_attempt_autolink(&self, group: &str) {

        if let Some(members) = Self::gather_group_members(group) {

            if ! self.group_members_are_ready(group, &members) { return }
            
            // tract_name -> (animus_name, ReceiverInfo)
            let mut receivers: HashMap<String, (String, ReceiverInfo)> = HashMap::new();
            // tract_name -> animus_name
            let mut senders: HashMap<String, String> = HashMap::new();

            for animus in members.iter() {
                self.gather_animus_inputs(animus, &mut receivers);
                self.gather_animus_outputs(animus, &mut senders);
            }

            self.attempt_link_tracts(&mut senders, &mut receivers)
                .report_unlinked_tracts(senders, receivers);
        }

    }

    fn gather_group_members(group: &str) -> Option<Vec<String>> {
        let read = crate::file::groups::read_group_members(group);
        if let Err(e) = &read {
            println!("Group file for '{}' is corrupted or missing: {}", group, e);
            return None
        }
        Some(read.unwrap())
    }

    // Check if all animi are active (accepting commands)
    // and asleep (not processing inputs)
    fn group_members_are_ready(
        &self, 
        group: &str,
        members: &Vec<String>
    ) -> bool {
        for animus in members.iter() {
            match self.is_active(animus) {
                Ok(active) => {
                    if !active {
                        println!(
                            "Animus '{}' is not active. \
                            Please activate all animi in the group '{}' before linking tracts.",
                            animus,
                            group
                        );

                        return false
                    }
                },
                Err(e) => {
                    Self::animus_command_error(animus, e);
                    return false
                }
            }

            match self.is_awake(animus) {
                Ok(awake) => {
                    if awake {
                        println!(
                            "Animus '{}' is processing inputs. \
                            Please sleep all animi in the group '{}' before linking tracts.",
                            animus,
                            group
                        );

                        return false
                    }
                },
                Err(e) => {
                    Self::animus_command_error(animus, e);
                    return false
                }
            }
        }

        true
    }

    fn gather_animus_inputs(
        &self, 
        animus: &str,
        receivers: &mut HashMap<String, (String, ReceiverInfo)>, 
    ) {
        let action = Action::ReportInputs;

        if let Err(e) = self.send_command(animus, action.clone()) {
            return Self::animus_command_error(animus, e)
        } else {
            match self.read_report() {
                Err(e) => return Self::animus_response_error(animus, action, e),
                Ok(report) => {

                    match report.outcome {
                        Outcome::Return(msg) => {

                            let msg = bincode::deserialize(&msg);
                            if let Err(e) = &msg {
                                return println!(
                                    "Failed to deserialize a Report's Outcome::Return data. \
                                    Check the version of `animusd-lib` you are using. \
                                    {}", e
                                )
                            }
                            let list: Vec<ReceiverInfo> = msg.unwrap();

                            for info in list.iter() {
                                let tract_name = &info.tract_name;
                                if let Some(..) = receivers.insert(
                                    tract_name.clone(), 
                                    (animus.to_string(), info.clone())
                                ) {
                                    return println!(
                                        "ERROR: Aborting auto-link: Duplicate of Input '{}' found",
                                        tract_name
                                    )
                                }
                            }
                        },
                        _ => {
                            return println!(
                                "ERROR: Unexpected Outcome violates protocol. \
                                Check the version of `animusd-lib` you are using."
                            )
                        }
                    }

                }
            }
        }

    }

    fn gather_animus_outputs(
        &self, 
        animus: &str,
        senders: &mut HashMap<String, String>, 
    ) {

        let action = Action::ListOutputs;

        if let Err(e) = self.send_command(animus, action.clone()) {
            return Self::animus_command_error(animus, e)
        } else {
            match self.read_report() {
                Err(e) => return Self::animus_response_error(animus, action, e),
                Ok(report) => {

                    match report.outcome {
                        Outcome::Return(msg) => {

                            let msg = bincode::deserialize(&msg);
                            if let Err(e) = &msg {
                                return println!(
                                    "Failed to deserialize a Report's Outcome::Return data. \
                                    Check the version of `animusd-lib` you are using. \
                                    {}", e
                                )
                            }
                            let list: Vec<String> = msg.unwrap();

                            for output in list.iter() {
                                if let Some(..) = senders.insert(
                                    output.clone(), animus.to_string() 
                                ) {
                                    return println!(
                                        "ERROR: Aborting auto-link: Duplicate of Output '{}' found",
                                        output
                                    )
                                }
                            }
                        },
                        _ => {
                            return println!(
                                "ERROR: Unexpected Outcome violates protocol. \
                                Check the version of `animusd-lib` you are using."
                            )
                        }
                    }

                }
            }
        }
    }

    fn attempt_link_tracts(
        &self, 
        senders: &mut HashMap<String, String>, 
        receivers: &mut HashMap<String, (String, ReceiverInfo)>
    ) -> &Self {

        for (tract_name, animus) in senders.clone().iter() {
            if let Some((_, info)) = receivers.get(tract_name) {
                let action = Action::LinkOutput(info.clone());
                if let Err(e) = self.send_command(animus, action.clone()) {
                    Self::animus_command_error(animus, e)
                } else {
                    receivers.remove(tract_name);
                    senders.remove(tract_name);
                }
            }
        }

        self
    }

    fn report_unlinked_tracts(
        &self, 
        senders: HashMap<String, String>, 
        receivers: HashMap<String, (String, ReceiverInfo)>
    ) {

        if !senders.is_empty() {
            println!("NOTE -- Some Ouputs were not linked (these may go to Motors):");
            for output in senders.iter() {
                // "animus_name: tract_name"
                println!("{}: {}", output.1, output.0)
            }
        }

        if !receivers.is_empty() {
            println!("NOTE -- Some Inputs were not linked (these may come from Sensors):");
            for input in receivers.iter() {
                // "animus_name: tract_name"
                println!("{}: {}", input.1.0, input.0)
            }
        }
    }
}


