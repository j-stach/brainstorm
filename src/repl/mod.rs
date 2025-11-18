
// The top-level REPL logic goes here.
pub(crate) mod meta;
pub(crate) use meta::meta_repl as run;

// Sub-REPL for managing an existing Animus instance.
pub(self) mod animus;
pub(self) use animus::animus_manager_repl;

