use thiserror::Error;

#[derive(Error, Debug)]
pub enum RedisError {
    #[error("unknown command: '{0}'")]
    UnknownCommand(String),

    #[error("wrong number of arguments for '{0}'")]
    WrongArgCount(String),

    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[error("empty command")]
    EmptyCommand,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
