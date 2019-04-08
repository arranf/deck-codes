use crate::format::*;

pub fn read(bytes: &Vec<u8>) {
    if bytes[0] != 0 {
        panic!("Invalid deck encoding: No leading 0 byte found")
    }

    let version = bytes[1];
    if version != 1 {
        panic!("Invalid deck encoding: Invalid or unsupported version")
    }

    let format: Format = Format::from_u8(bytes[2]);

    // TODO: Check whether unknown is common
    if format == Format::Unknown {
        panic!("Invalid deck encoding: Unknown format")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[should_panic]
    #[test]
    fn read_panics_if_there_is_no_leading_0_byte() {
        let numbers = vec![1, 2, 3];
        read(&numbers);
    }
}
