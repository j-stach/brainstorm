
impl crate::Brainstorm {
    // 
    pub(crate) fn group_attempt_autolink(&self, group: &str) {

        use std::collections::HashMap;
        use cajal_cx::tract::receiver::ReceiverInfo;
        use animusd_lib::protocol::{ Action, Outcome };

        let read = crate::file::groups::read_group_members(group);
        if let Err(e) = &read {
            return println!("Group file for '{}' is corrupted or missing: {}", group, e)
        }
        let members = read.unwrap();

        // Check if all animi are active (accepting commands)
        // and asleep (not processing inputs)
        for animus in members.iter() {
            match self.is_active(animus) {
                Ok(active) => {
                    if !active {
                        return println!(
                            "Animus '{}' is not active. \
                            Please activate all animi in the group '{}' before linking tracts.",
                            animus,
                            group
                        )
                    }
                },
                Err(e) => {
                    return Self::animus_command_error(animus, e)
                }
            }

            match self.is_awake(animus) {
                Ok(awake) => {
                    if awake {
                        return println!(
                            "Animus '{}' is processing inputs. \
                            Please sleep all animi in the group '{}' before linking tracts.",
                            animus,
                            group
                        )
                    }
                },
                Err(e) => {
                    return Self::animus_command_error(animus, e)
                }
            }
        }
        
        // Gather Input and Output information from each animus:
        // tract_name -> (animus_name, ReceiverInfo)
        let mut receivers: HashMap<String, (String, ReceiverInfo)> = HashMap::new();
        // tract_name -> animus_name
        let mut senders: HashMap<String, String> = HashMap::new();

        for animus in members.iter() {

            let action = Action::ReportInputs;

            // Collect all Inputs
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
                                        (animus.clone(), info.clone())
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

            let action = Action::ListOutputs;

            // Collect all Outputs
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
                                        output.clone(), animus.clone() 
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

        // Attempt to link using collected Inputs and Outputs
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

        // Report unlinked tracts:
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

