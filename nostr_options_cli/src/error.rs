use crate::utils::FileError;
use nostr_relay_connector::error::RelayClientError;
use nostr_relay_processor::error::RelayProcessorError;

pub type Result<T> = core::result::Result<T, CliError>;

#[derive(thiserror::Error, Debug)]
pub enum CliError {
    #[error("Occcurred error with io, err: {0}")]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    File(#[from] FileError),
    #[error(transparent)]
    RelayClient(#[from] RelayClientError),
    #[error(transparent)]
    RelayProcessor(#[from] RelayProcessorError),
}
