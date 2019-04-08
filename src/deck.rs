use crate::format::Format;

#[derive(PartialEq, Debug)]
pub struct Deck {
    format: Format,
    heroes: Vec<u32>,
    cards: Vec<(u8, u32)>,
}

impl Deck {
    pub fn new(bytes: &Vec<u32>) -> Self {
        let total_bytes = bytes.len();

        if total_bytes < 7 {
            panic!("Invalid deck encoding: length is too small");
        }

        if bytes[0] != 0 {
            panic!("Invalid deck encoding: No leading 0 byte found")
        }

        let version = bytes[1];
        if version != 1 {
            panic!("Invalid deck encoding: Invalid or unsupported version")
        }

        let format: Format = Format::from_u32(bytes[2]);

        // Figure out where each region lies in the bytes
        let hero_count = bytes[3] as usize;
        let last_hero_byte: usize = 3 + hero_count;
        let single_card_count = bytes[last_hero_byte + 1];
        let last_single_card_byte = next_end(last_hero_byte, single_card_count);
        if last_single_card_byte as usize > total_bytes {
            panic!("Error in deck encoding: Length of card sections do not match number of bytes");
        }

        let double_card_count = bytes[last_single_card_byte + 1];
        let last_double_card_byte = next_end(last_single_card_byte, double_card_count);
        if last_double_card_byte as usize > total_bytes {
            panic!("Error in deck encoding: Length of card sections do not match number of bytes");
        }

        let multi_card_count = bytes[last_double_card_byte + 1];
        let last_multi_card_byte = next_end(last_double_card_byte, multi_card_count * 2); // 2 * because of count and card id
        if last_multi_card_byte as usize > total_bytes {
            panic!("Error in deck encoding: Length of card sections do not match number of bytes");
        }

        // Iterate over heroes
        let first_hero_byte: usize = 4;
        let mut heroes: Vec<u32> = Vec::new();
        for i in first_hero_byte..(last_hero_byte + 1) {
            heroes.push(bytes[i]);
        }

        // Iterate over cards
        let mut cards: Vec<(u8, u32)> = Vec::new();
        let first_single_card_byte: usize = last_hero_byte + 2;
        for i in first_single_card_byte..(last_single_card_byte + 1) {
            cards.push((1, bytes[i]));
        }

        let first_double_card_byte: usize = last_single_card_byte + 2;
        for i in first_double_card_byte..(last_double_card_byte + 1) {
            cards.push((2, bytes[i]));
        }

        // Iterate over card and number pairs
        let mut index = last_double_card_byte + 2;
        while index < last_multi_card_byte {
            let card = bytes[index];
            let number_of_card = bytes[index + 1] as u8;
            cards.push((number_of_card, card));
            index = index + 2;
        }

        cards.sort_by(|a, b| a.1.partial_cmp(&b.1).expect("Error comparing values"));

        return Deck {
            format: format,
            heroes: heroes,
            cards: cards,
        };
    }
}

fn next_end(previous_end: usize, total_number: u32) -> usize {
    return previous_end + total_number as usize + 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[should_panic]
    #[test]
    fn new_panics_if_there_is_a_small_number_of_bytes_than_7() {
        let input = vec![0, 1, 2];
        Deck::new(&input);
    }

    #[should_panic]
    #[test]
    fn new_panics_if_there_is_no_leading_0_byte() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        Deck::new(&input);
    }

    #[should_panic]
    #[test]
    fn new_panics_if_there_is_an_unexpected_version() {
        let input = vec![0, 2, 0, 0, 0, 0, 0, 0, 0, 0];
        Deck::new(&input);
    }

    #[should_panic]
    #[test]
    fn new_panics_if_there_is_a_larger_suggested_number_of_bytes_than_total_bytes() {
        let input = vec![
            0, 1, 1, //
            // Hero Section
            1, 7, //
            // Single card section
            7, // A LIE: 7 Single Cards when there are none
            // Double card section
            0, // Paired (Id, Count) Section
            0,
        ];
        Deck::new(&input);
    }

    #[test]
    fn new_matches_simple_example() {
        let input = vec![
            0, // Null byte
            1, // Version 1
            1, // Wild
            // Hero Section
            1, // 1 Hero
            7, // Hero is id 7: Garrosh
            // Single card section
            0, // 0 Single Cards
            // Double card section
            0, // 0 Double cards
            // Paired (Id, Count) Section
            4, // 4 Pairs
            1, 3, //
            2, 3, //
            3, 3, //
            4, 3, //
        ];

        let result = Deck::new(&input);

        let expected = Deck {
            format: Format::Wild,
            heroes: vec![7],
            cards: vec![(3, 1), (3, 2), (3, 3), (3, 4)],
        };

        assert_eq!(result, expected);
    }
}
