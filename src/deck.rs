use crate::error::DeckCodeError;
use crate::format::Format;

#[derive(PartialEq, Debug)]
/// A representation of a Hearthstone deck
pub struct Deck {
    version: u8,
    pub format: Format,
    /// The dbfid of the heroes this deck should use
    pub heroes: Vec<u32>,
    /// The dbfid of the cards in the deck that have a single copy. Sorted by dbfid.
    single_cards: Vec<u32>,
    /// The dbfid of the cards in the deck that have a two copies. Sorted by dbfid.
    double_cards: Vec<u32>,
    #[allow(clippy::doc_markdown)]
    /// The dbfid of the cards in the deck that have a more than two copies. Stored as tupes of (number_of_copies, dbfid). Sorted by dbfid.
    multi_cards: Vec<(u8, u32)>,
    #[allow(clippy::doc_markdown)]
    /// The dbfid of the cards in the sideboard Stored as tupes of (dbfid, number_of_copies, owner dbfid). Sorted by dbfid.
    sideboard_cards: Vec<(u32, u8, u32)>,
}

impl Deck {
    /// The total number of cards in the deck
    #[must_use]
    pub fn total_cards(&self) -> usize {
        let mut card_count = self.single_cards.len() + self.double_cards.len() * 2;
        card_count += self.multi_cards.iter().fold(0, |acc, t| acc + t.0) as usize;
        card_count
    }

    /// The number of cards in the deck. Equivalent to the height of the deck when represented in hearthstone.
    fn total_card_slots(&self) -> usize {
        self.single_cards.len() + self.double_cards.len() + self.multi_cards.len()
    }

    /// A representation of the all the cards in the Deck, including any sideboards.
    /// # Panics
    /// Panics if two cards dbfids cannot be compared. Should not occur.
    #[must_use]
    pub fn cards(&self) -> Vec<(u8, u32, Option<u32>)> {
        let mut cards: Vec<(u8, u32, Option<u32>)> = Vec::new();

        // Cards are sorted in the struct so no need to re-sort.

        for card in &self.single_cards {
            cards.push((1, *card, None));
        }

        for card in &self.double_cards {
            cards.push((2, *card, None));
        }

        for (amount, card) in &self.multi_cards {
            cards.push((*amount, *card, None));
        }

        for (card, amount, owner_dbfid) in &self.sideboard_cards {
            cards.push((*amount, *card, Some(*owner_dbfid)));
        }

        cards
    }

    /// Create a new deck from vector of u32 bytes.
    /// This representation is [described by Hearthsim](https://hearthsim.info/docs/deckstrings/).
    /// # Errors
    /// Returns an error when the bytes passed cannot produce a functional deck code.
    #[allow(clippy::too_many_lines)]
    pub fn new(bytes: &[u32]) -> Result<Self, DeckCodeError> {
        let total_bytes = bytes.len();

        if total_bytes < 7 {
            return Err(DeckCodeError::InvalidDeckEncoding {
                encoding_type: String::from("Length is too small"),
            });
        }

        if bytes[0] != 0 {
            return Err(DeckCodeError::InvalidDeckEncoding {
                encoding_type: String::from("No leading 0 byte found"),
            });
        }

        let version = std::convert::TryInto::<u8>::try_into(bytes[1]).map_err(|_| {
            DeckCodeError::InvalidDeckEncoding {
                encoding_type: "Could not read deck code version.".to_owned(),
            }
        })?;

        if version != 1 {
            return Err(DeckCodeError::UnknownVersion {
                version: u32::from(version),
            });
        }

        let format: Format = Format::from_u32(bytes[2])?;

        // Figure out where each region lies in the bytes
        let hero_count = bytes[3] as usize;
        let last_hero_byte: usize = 3 + hero_count;
        let single_card_count = bytes[last_hero_byte + 1];
        let last_single_card_byte = next_end(last_hero_byte, single_card_count);
        if last_single_card_byte > total_bytes {
            return Err(DeckCodeError::InvalidDeckEncoding {
                encoding_type: String::from(
                    "Length of card sections does not match number of bytes",
                ),
            });
        }

        let double_card_count = bytes[last_single_card_byte + 1];
        let last_double_card_byte = next_end(last_single_card_byte, double_card_count);
        if last_double_card_byte > total_bytes {
            return Err(DeckCodeError::InvalidDeckEncoding {
                encoding_type: String::from(
                    "Length of card sections does not match number of bytes",
                ),
            });
        }

        let multi_card_count = bytes[last_double_card_byte + 1];
        let last_multi_card_byte = next_end(last_double_card_byte, multi_card_count * 2); // 2 * because this section stores count and card id
        if last_multi_card_byte > total_bytes {
            return Err(DeckCodeError::InvalidDeckEncoding {
                encoding_type: String::from(
                    "Length of card sections does not match number of bytes",
                ),
            });
        }

        // Older deck codes may not have a byte indicating if a sideboard exists. Assume no sideboard if this byte is not present.
        // +2 because: 1 byte for byte indicating presence of sideboard and then 1 byte for number of cards in sideboard
        let sideboard_card_count = if last_multi_card_byte + 2 >= total_bytes {
            0
        } else {
            bytes[last_multi_card_byte + 2]
        };

        let last_sideboard_card_byte = next_end(last_multi_card_byte, sideboard_card_count * 2 + 1); // 2 * because this section stores card id, and sideboard_for id. +1 because the first byte indicates presence of a sideboard

        if sideboard_card_count > 0 && last_sideboard_card_byte > total_bytes {
            return Err(DeckCodeError::InvalidDeckEncoding {
                encoding_type: String::from(
                    "Length of card sections does not match number of bytes",
                ),
            });
        }

        // Iterate over heroes
        let first_hero_byte: usize = 4;
        let mut heroes: Vec<u32> = Vec::new();
        for i in first_hero_byte..=last_hero_byte {
            heroes.push(bytes[i]);
        }

        // Iterate over cards
        let mut single_cards: Vec<u32> = Vec::with_capacity(single_card_count as usize);
        let first_single_card_byte: usize = last_hero_byte + 2;
        for i in first_single_card_byte..=last_single_card_byte {
            single_cards.push(bytes[i]);
        }

        let mut double_cards: Vec<u32> = Vec::with_capacity(double_card_count as usize);
        let first_double_card_byte: usize = last_single_card_byte + 2;
        for i in first_double_card_byte..=last_double_card_byte {
            double_cards.push(bytes[i]);
        }

        // Iterate over card and number pairs
        let mut multi_cards: Vec<(u8, u32)> = Vec::with_capacity(multi_card_count as usize);
        let mut index = last_double_card_byte + 2;
        while index < last_multi_card_byte {
            let card = bytes[index];
            let number_of_card =
                u8::try_from(bytes[index + 1]).map_err(|_| DeckCodeError::InvalidDeckEncoding {
                    encoding_type: String::from("Amount of single card exceeded 255"),
                })?;
            multi_cards.push((number_of_card, card));
            index += 2;
        }

        // Iterate over sideboard
        let mut sideboard_cards: Vec<(u32, u8, u32)> =
            Vec::with_capacity(sideboard_card_count as usize);
        if sideboard_card_count > 0 {
            let mut index = last_multi_card_byte + 3;
            while index < last_sideboard_card_byte {
                let card_id = bytes[index];
                let sideboard_for_card_id = bytes[index + 1];
                sideboard_cards.push((card_id, 1, sideboard_for_card_id));
                index += 2;
            }
        }

        single_cards.sort_unstable();
        double_cards.sort_unstable();
        multi_cards.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        sideboard_cards.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

        Ok(Self {
            version,
            format,
            heroes,
            single_cards,
            double_cards,
            multi_cards,
            sideboard_cards,
        })
    }

    /// Encode the deck as a u32 vector
    ///
    /// # Panics
    /// Panics if the Deck provided has greater than u32 unique cards
    #[must_use]
    pub fn to_byte_array(&self) -> Vec<u32> {
        // Minimum amount: 0x0, version, format, hero count, single count, double count, multi-count. Total of (7 bytes) + counts
        let mut byte_array: Vec<u32> =
            Vec::with_capacity(7 + self.heroes.len() + self.total_card_slots());
        let mut vec = vec![
            0,
            u32::from(self.version),
            u32::from(self.format.to_u8()),
            u32::try_from(self.heroes.len()).expect("More heroes provided than expected"),
        ];
        byte_array.append(&mut vec);
        byte_array.extend(&self.heroes.clone());
        byte_array.push(
            u32::try_from(self.single_cards.len())
                .expect("More single cards provided than expected"),
        );
        byte_array.extend(&self.single_cards.clone());

        byte_array.push(
            u32::try_from(self.double_cards.len())
                .expect("More double cards provided than expected"),
        );
        byte_array.extend(&self.double_cards.clone());

        byte_array.push(
            u32::try_from(self.multi_cards.len()).expect("More multi-cards provided than expected"),
        );
        byte_array.extend(&flatten_multi_cards(&self.multi_cards));

        if self.sideboard_cards.is_empty() {
            byte_array.push(0);
        } else {
            byte_array.push(1);
            byte_array.push(
                u32::try_from(self.sideboard_cards.len())
                    .expect("More sideboard cards provided than expected"),
            );
            byte_array.extend(&flatten_sideboard(&self.sideboard_cards));
        }

        byte_array
    }
}

fn flatten_multi_cards(intervals: &[(u8, u32)]) -> Vec<u32> {
    use std::iter::once;

    intervals
        .iter()
        .flat_map(|tup| once(tup.1).chain(once(u32::from(tup.0))))
        .collect()
}

fn flatten_sideboard(intervals: &[(u32, u8, u32)]) -> Vec<u32> {
    use std::iter::once;

    intervals
        .iter()
        .flat_map(|tup| once(tup.0).chain(once(tup.2)))
        // Terminating null bytes are only for sideboards
        .chain(once(0))
        .chain(once(0))
        .collect()
}

fn next_end(previous_end: usize, total_number: u32) -> usize {
    previous_end + total_number as usize + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_returns_err_if_there_is_a_small_number_of_bytes_than_7() {
        let input = vec![0, 1, 2];
        let result = Deck::new(&input);
        assert!(result.is_err());
    }

    #[test]
    fn new_returns_err_if_there_is_no_leading_0_byte() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = Deck::new(&input);
        assert!(result.is_err());
    }

    #[test]
    fn new_returns_err_if_there_is_an_unexpected_version() {
        let input = vec![0, 2, 0, 0, 0, 0, 0, 0, 0, 0];
        let result = Deck::new(&input);
        assert!(result.is_err());
    }

    #[test]
    fn new_returns_err_if_there_is_a_larger_suggested_number_of_bytes_than_total_bytes() {
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
        let result = Deck::new(&input);
        assert!(result.is_err());
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
            4, // 4 sets of multi-card
            1, 3, // id 1, 3 copies
            2, 3, // id 2, 3 copies
            3, 3, // id 3, 3 copies
            4, 3, // id 4, 3 copies
        ];

        let result = Deck::new(&input);

        let expected = Deck {
            format: Format::Wild,
            version: 1,
            heroes: vec![7],
            single_cards: Vec::new(),
            double_cards: Vec::new(),
            multi_cards: vec![(3, 1), (3, 2), (3, 3), (3, 4)],
            sideboard_cards: Vec::new(),
        };
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn to_byte_array_matches_simple_example() {
        let expected = vec![
            0, // Null byte
            1, // Version 1
            1, // Wild format
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
            // Sideboard section
            0, // No sideboard
        ];

        let input = Deck {
            format: Format::Wild,
            version: 1,
            heroes: vec![7],
            single_cards: Vec::new(),
            double_cards: Vec::new(),
            multi_cards: vec![(3, 1), (3, 2), (3, 3), (3, 4)],
            sideboard_cards: Vec::new(),
        };
        let result = input.to_byte_array();
        assert_eq!(result, expected);
    }

    #[test]
    fn to_byte_array_matches_complex_example() {
        let expected = vec![
            0,   // Null byte
            1,   // Version 1
            2,   // Standard format
            1,   // 1 Hero
            637, // Jaina,
            // Section Single Cards
            4,     // 4 Single Cards
            192,   // Ice Block
            39841, // Medivh, the Guardian
            42718, // Ghastly Conjurer
            42790, // Bonemare
            // Section Double Cards
            13,    // 13 Double Cards
            113,   // Counter Spell
            195,   // Mirror Entity
            315,   // Fireball
            405,   // Mana Wyrm
            555,   // Arcane Intellect
            662,   // Frostbolt
            748,   // Kirin Tor Mage,
            39715, // Firelands Portal
            39767, // Medivh’s Valet
            40297, // Volcanic Potion
            40583, // Kabal Crystal Runner
            41153, // Arcanologist
            41496, // Primordial Glyph,
            // Section Multi Cards
            0, // No 3+-copy cards
            // Section Sideboard
            0, // No Sideboard
        ];

        let input = Deck {
            format: Format::Standard,
            version: 1,
            heroes: vec![637],
            single_cards: vec![192, 39841, 42718, 42790],
            double_cards: vec![
                113, 195, 315, 405, 555, 662, 748, 39715, 39767, 40297, 40583, 41153, 41496,
            ],
            multi_cards: Vec::new(),
            sideboard_cards: Vec::new(),
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
            sideboard_cards: Vec::new(),
        };
        assert_eq!(24, input.total_cards());
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
            sideboard_cards: Vec::new(),
        };
        assert_eq!(12, input.total_card_slots());
    }
}
