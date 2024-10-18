use crate::error::DeckCodeError;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Format {
    Unknown,
    Wild,
    Standard,
    Classic,
    Twist,
}

impl Format {
    /// Returns a matching `Format` from a `format_id`.
    ///
    /// # Errors
    ///
    /// This function will return an error if the `format_id` does not match a known `Format`.
    pub fn from_u32(format_id: u32) -> Result<Self, DeckCodeError> {
        match format_id {
            0 => Ok(Self::Unknown),
            1 => Ok(Self::Wild),
            2 => Ok(Self::Standard),
            3 => Ok(Self::Classic),
            4 => Ok(Self::Twist),
            _ => Err(DeckCodeError::UnknownDeckFormat {
                deck_format: format_id,
            }),
        }
    }

    #[must_use]
    pub fn to_u8(&self) -> u8 {
        match &self {
            Self::Unknown => 0,
            Self::Wild => 1,
            Self::Standard => 2,
            Self::Classic => 3,
            Self::Twist => 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_wild() {
        let input = 1;
        assert_eq!(Ok(Format::Wild), Format::from_u32(input));
    }

    #[test]
    fn decode_standard() {
        let input = 2;
        assert_eq!(Ok(Format::Standard), Format::from_u32(input));
    }

    #[test]
    fn decode_unknown() {
        let input = 0;
        assert_eq!(Ok(Format::Unknown), Format::from_u32(input));
    }

    #[test]
    fn decode_returns_err_when_outside_known_range() {
        let input = 13;
        assert_eq!(
            Err(DeckCodeError::UnknownDeckFormat { deck_format: input }),
            Format::from_u32(input)
        );
    }
}
