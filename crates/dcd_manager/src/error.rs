use simplicityhl::elements;
use simplicityhl::elements::VerificationError;

#[derive(thiserror::Error, Debug)]
pub enum DcdManagerError {
    #[error("Occurred error, msg: {0}")]
    Internal(String),
    #[error("Verification error: '{0}'")]
    VerificationError(#[from] VerificationError),
    #[error("Verification error: '{0}'")]
    PSetError(#[from] elements::pset::Error),
}

pub type Result<T> = std::result::Result<T, DcdManagerError>;
