#[derive(PartialEq, PartialOrd, Debug)]
pub enum Format {
    Unknown,
    Wild,
    Standard,
    Classic,
}

impl Format {
    pub fn from_u32(i: u32) -> Self {
        match i {
            0 => Self::Unknown,
            1 => Self::Wild,
            2 => Self::Standard,
            3 => Self::Classic,
            _ => panic!("Error getting format from u8: Does not decode"),
        }
    }

    pub fn to_u8(&self) -> u8 {
        match &self {
            Self::Unknown => 0,
            Self::Wild => 1,
            Self::Standard => 2,
            Self::Classic => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_wild() {
        let input = 1;
        assert_eq!(Format::Wild, Format::from_u32(input));
    }

    #[test]
    fn decode_standard() {
        let input = 2;
        assert_eq!(Format::Standard, Format::from_u32(input));
    }

    #[test]
    fn decode_unknown() {
        let input = 0;
        assert_eq!(Format::Unknown, Format::from_u32(input));
    }

    #[should_panic]
    #[test]
    fn decode_panics_when_outside_known_range() {
        let input = 13;
        assert_eq!(Format::Unknown, Format::from_u32(input));
    }
}
