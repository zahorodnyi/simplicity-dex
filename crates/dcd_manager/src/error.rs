#[derive(thiserror::Error, Debug)]
pub enum DcdManagerError {
    // #[error(transparent)]
    // RelayClient(#[from] RelayClientError),
    // #[error("Signer error: {0}")]
    // Signer(#[from] SignerError),
    // #[error("Single letter error: {0}")]
    // SingleLetterTag(#[from] SingleLetterTagError),
}

pub type Result<T> = std::result::Result<T, DcdManagerError>;
