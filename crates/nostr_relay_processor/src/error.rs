use nostr_relay_connector::error::RelayClientError;

#[derive(thiserror::Error, Debug)]
pub enum RelayProcessorError {
    #[error(transparent)]
    RelayClient(#[from] RelayClientError),
}

pub type Result<T> = std::result::Result<T, RelayProcessorError>;
