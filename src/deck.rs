use crate::format::Format;

#[derive(PartialEq, Debug)]
pub struct Deck {
    version: u8,
    format: Format,
    heroes: Vec<u32>,
    single_cards: Vec<u32>,
    double_cards: Vec<u32>,
    multi_cards: Vec<(u8, u32)>,
}

impl Deck {
    pub fn total_cards(&self) -> usize {
        let mut card_count = self.single_cards.len() + self.double_cards.len() * 2;
        card_count = card_count + self.multi_cards.iter().fold(0, |acc, t| acc + t.0) as usize;
        card_count
    }

    fn total_card_slots(&self) -> usize {
        self.single_cards.len() + self.double_cards.len() + self.multi_cards.len()
    }

    pub fn cards(&self) -> Vec<(u8, u32)> {
        let mut cards: Vec<(u8, u32)> = Vec::new();
        for card in &self.single_cards {
            cards.push((1, *card));
        }

        for card in &self.double_cards {
            cards.push((2, *card));
        }

        for (amount, card) in &self.multi_cards {
            cards.push((*amount, *card));
        }

        cards.sort_by(|a, b| a.1.partial_cmp(&b.1).expect("Error comparing values"));

        cards
    }

    pub fn new(bytes: &Vec<u32>) -> Self {
        let total_bytes = bytes.len();

        if total_bytes < 7 {
            panic!("Invalid deck encoding: length is too small");
        }

        if bytes[0] != 0 {
            panic!("Invalid deck encoding: No leading 0 byte found")
        }

        let version = bytes[1] as u8;
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
        let last_multi_card_byte = next_end(last_double_card_byte, multi_card_count * 2); // 2 * because this section stores count and card id
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
        let mut single_cards: Vec<u32> = Vec::with_capacity(single_card_count as usize);
        let first_single_card_byte: usize = last_hero_byte + 2;
        for i in first_single_card_byte..(last_single_card_byte + 1) {
            single_cards.push(bytes[i]);
        }

        let mut double_cards: Vec<u32> = Vec::with_capacity(double_card_count as usize);
        let first_double_card_byte: usize = last_single_card_byte + 2;
        for i in first_double_card_byte..(last_double_card_byte + 1) {
            double_cards.push(bytes[i]);
        }

        // Iterate over card and number pairs
        let mut multi_cards: Vec<(u8, u32)> = Vec::with_capacity(multi_card_count as usize);
        let mut index = last_double_card_byte + 2;
        while index < last_multi_card_byte {
            let card = bytes[index];
            let number_of_card = bytes[index + 1] as u8;
            multi_cards.push((number_of_card, card));
            index = index + 2;
        }

        Deck {
            version: version,
            format: format,
            heroes: heroes,
            single_cards: single_cards,
            double_cards: double_cards,
            multi_cards: multi_cards,
        }
    }

    pub fn to_byte_array(&self) -> Vec<u32> {
        // Minimum amount: 0x0, version, format, hero count, single count, double count, multi-count (7 bytes) + count s
        let mut byte_array: Vec<u32> =
            Vec::with_capacity(7 + self.heroes.len() + self.total_card_slots());
        byte_array.append(&mut vec![
            0,
            (self.version as u32),
            (self.format.to_u8() as u32),
            (self.heroes.len() as u32),
        ]);
        byte_array.extend(&self.heroes.clone());
        byte_array.push(self.single_cards.len() as u32);
        byte_array.extend(&self.single_cards.clone());
        byte_array.push(self.double_cards.len() as u32);
        byte_array.extend(&self.double_cards.clone());
        byte_array.push((self.multi_cards.len()) as u32);
        byte_array.extend(&flatten(&self.multi_cards));

        dbg!(&byte_array);
        byte_array
    }
}

fn flatten(intervals: &[(u8, u32)]) -> Vec<u32> {
    use std::iter::once;

    intervals
        .iter()
        .flat_map(|tup| once(tup.1).chain(once(tup.0 as u32)))
        .collect()
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
            version: 1,
            heroes: vec![7],
            single_cards: Vec::new(),
            double_cards: Vec::new(),
            multi_cards: vec![(3, 1), (3, 2), (3, 3), (3, 4)],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn to_byte_array_matches_simple_example() {
        let expected = vec![
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

        let input = Deck {
            format: Format::Wild,
            version: 1,
            heroes: vec![7],
            single_cards: Vec::new(),
            double_cards: Vec::new(),
            multi_cards: vec![(3, 1), (3, 2), (3, 3), (3, 4)],
        };
        let result = input.to_byte_array();
        assert_eq!(result, expected);
    }

    #[test]
    fn total_cards() {
        let input = Deck {
            format: Format::Wild,
            version: 1,
            heroes: vec![7],
            single_cards: vec![1, 2, 3, 4],                    // 4
            double_cards: vec![1, 2, 3, 4],                    // 8
            multi_cards: vec![(3, 1), (3, 2), (3, 3), (3, 4)], //12
        };
        assert_eq!(24, input.total_cards())
    }

    #[test]
    fn total_card_slots() {
        let input = Deck {
            format: Format::Wild,
            version: 1,
            heroes: vec![7],
            single_cards: vec![1, 2, 3, 4],                    // 4
            double_cards: vec![1, 2, 3, 4],                    // 4
            multi_cards: vec![(3, 1), (3, 2), (3, 3), (3, 4)], // 4
        };
        assert_eq!(12, input.total_card_slots())
    }
}
