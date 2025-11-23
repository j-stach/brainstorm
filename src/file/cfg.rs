
//! Define the Brainstorm configuration here

ezcfg::cfg!{
    BrainstormConfig ["./cajal/brainstorm/brainstorm.cfg"]
        // Define configuration field names and types here:
        // TBD: Authentication codes, etc.
}

impl Default for BrainstormConfig {
    fn default() -> Self {

        BrainstormConfig {
            // Define default configuration values here:
        }
    }
}


