use custom_error::custom_error;
use base64::DecodeError;

custom_error! {pub DeckCodeError // Enum name
    // Specific types
    InvalidDeckEncoding{encoding_type: String} = "Invalid deck encoding: {encoding_type}.",
    InvalidBase64{source: DecodeError}            = "Invalid input code."
}
