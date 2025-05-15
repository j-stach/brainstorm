
// The top-level REPL logic goes here.
pub(crate) mod brainstorm;
pub(crate) use brainstorm::brainstorm_repl;

// Sub-REPL for managing an existing Animus instance.
pub(self) mod animus;
pub(self) use animus::animus_manager_repl;

// Sub-REPL for configuring a new Animus instance.
pub(self) mod config;
pub(self) use config::animus_config_repl;

