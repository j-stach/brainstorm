
/// The top-level REPL logic goes here.
pub(crate) mod brainstorm;
pub(crate) use brainstorm::brainstorm_repl;

/// Sub-REPL for managing an existing Animus instance.
pub(self) mod animus;
pub(self) use animus::animus_manager_repl;

// TODO: Other REPL as needed

