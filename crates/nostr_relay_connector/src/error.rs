#[derive(Debug, thiserror::Error)]
pub enum RelayClientError {
    #[error("Failed to convert custom url to RelayURL, err: {err_msg}")]
    FailedToConvertRelayUrl { err_msg: String },
    #[error("An error occurred in Nostr Client, err: {0}")]
    NostrClientFailure(#[from] nostr_sdk::client::Error),
    #[error("Relay Client requires for operation signature, add key to the Client")]
    MissingSigner,
}

pub type Result<T> = std::result::Result<T, RelayClientError>;
