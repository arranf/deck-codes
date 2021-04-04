use base64::DecodeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeckCodeError {
    #[error("Invalid deck encoding: {encoding_type:?}.")]
    InvalidDeckEncoding { encoding_type: String },
    #[error("Invalid input code.")]
    InvalidBase64(#[from] DecodeError),
    #[error("Unknown error processing deck code")]
    Unknown,
}
