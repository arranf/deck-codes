use base64::DecodeError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DeckCodeError {
    #[error("Invalid deck encoding: {encoding_type:?}.")]
    InvalidDeckEncoding { encoding_type: String },
    #[error("Invalid input code.")]
    InvalidBase64(#[from] DecodeError),
    #[error("Unknown deck format: {deck_format}.")]
    UnknownDeckFormat { deck_format: u32 },
    #[error("Unknown deck code version: {version}.")]
    UnknownVersion { version: u32 },
    #[error("Unknown error processing deck code")]
    Unknown,
}
