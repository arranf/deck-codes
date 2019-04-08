#[derive(PartialEq, PartialOrd, Debug)]
pub enum Format {
    Unknown,
    Wild,
    Standard,
}

impl Format {
    pub fn from_u8(i: u8) -> Self {
        match i {
            0 => Format::Unknown,
            1 => Format::Wild,
            2 => Format::Standard,
            _ => panic!("Error getting format from u8: Does not decode"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_wild() {
        let input: u8 = 1;
        assert_eq!(Format::Wild, Format::from_u8(input));
    }

    #[test]
    fn decode_standard() {
        let input: u8 = 2;
        assert_eq!(Format::Standard, Format::from_u8(input));
    }

    #[test]
    fn decode_unknown() {
        let input: u8 = 0;
        assert_eq!(Format::Unknown, Format::from_u8(input));
    }

    #[should_panic]
    #[test]
    fn decode_panics_when_outside_known_range() {
        let input: u8 = 13;
        assert_eq!(Format::Unknown, Format::from_u8(input));
    }
}
