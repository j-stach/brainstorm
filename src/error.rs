
// TODO: REDO errors entirely

#[derive(Debug, thiserror::Error)]
pub(crate) enum SetupError {

    #[error("IO operation failed: {0}")]
    IoFailed(#[from] std::io::Error),

    #[error("Failed to execute command {0}")]
    ExecutionFailed(String),

    #[error("Invalid features string in config: {0}")]
    InvalidFeatures(String),

    #[error("Invalid config detected: {0}")]
    BadConfig(#[from] animusd_lib::error::ConfigError),

    #[error("Failed to execute animus command: {0}")]
    CommandFailed(#[from] animusd_lib::error::CommandError),

    #[error("Invalid file '{0}': Requires `.nn` file extension.")]
    BadFilename(String),

    #[error("Animus setup was aborted")]
    SetupAborted,
}

