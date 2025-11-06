use nostr::SignerError;
use nostr::filter::SingleLetterTagError;
use nostr_relay_connector::error::RelayClientError;

#[derive(thiserror::Error, Debug)]
pub enum RelayProcessorError {
    #[error(transparent)]
    RelayClient(#[from] RelayClientError),
    #[error("Signer error: {0}")]
    Signer(#[from] SignerError),
    #[error("Single letter error: {0}")]
    SingleLetterTag(#[from] SingleLetterTagError),
}

pub type Result<T> = std::result::Result<T, RelayProcessorError>;
