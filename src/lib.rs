#![allow(clippy::all, clippy::pedantic)]

extern crate base64;
extern crate custom_error;
extern crate integer_encoding;

mod deck;
mod error;
mod format;

use crate::deck::Deck;
use crate::error::*;
use base64::{decode, encode};
use integer_encoding::VarInt;

/// Convert a Hearthstone deck code into a Deck struct
pub fn decode_deck_code(deck_code: &str) -> Result<Deck, DeckCodeError> {
    let decoded: Vec<u32> = decode_code_to_u32_vec(deck_code)?;
    Deck::new(&decoded)
}

/// Convert a deck struct into an importable Hearthstone deck code
pub fn encode_deck_code(deck: Deck) -> String {
    encode_u32_vec_to_deck_code(deck.to_byte_array())
}

/// Convert a Base64 deck code into a vector of u32 values that can then be mapped to the format of the deck
fn decode_code_to_u32_vec(deck_code: &str) -> Result<Vec<u32>, DeckCodeError> {
    let mut decoded = decode(deck_code)?;

    let mut deck_code_decoded: Vec<u32> = vec![];
    // Read u8 values as u32 varints
    while decoded.len() > 0 {
        let (read, size) = u32::decode_var(&decoded);
        deck_code_decoded.push(read);
        decoded = decoded[size..].to_vec();
    }
    Ok(deck_code_decoded)
}

/// Convert a vector of u32 values into a Base64 deck code
fn encode_u32_vec_to_deck_code(byte_array: Vec<u32>) -> String {
    let mut fixed_size_integers: Vec<u8> = Vec::new();
    let mut encoded: [u8; 4] = [0, 0, 0, 0]; // This is calculated by taking the largest dbfid and calculating ceil(log(dbfid, 128)) as 128 is the largest value a u8 can store.
    for i in byte_array {
        let encoded_bytes = i.encode_var(&mut encoded[..]);
        for encoded_index in 0..encoded_bytes {
            fixed_size_integers.push(encoded[encoded_index]);
        }
    }
    encode(&fixed_size_integers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_code_to_u32_vec_correctly_decodes_a_simple_code() {
        let result = decode_code_to_u32_vec("AAEBAQcAAAQBAwIDAwMEAw==");
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
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn encode_simple_byte_array_as_deck_code() {
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
            1, 3, // 3x dbfid 1
            2, 3, //
            3, 3, //
            4, 3, //
        ];
        let expected = "AAEBAQcAAAQBAwIDAwMEAw==";
        let result = encode_u32_vec_to_deck_code(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn decode_code_to_u8_vec_correctly_decodes_a_complex_code() {
        let result = decode_code_to_u32_vec(
            "AAECAf0EBMABobcC3s0Cps4CDXHDAbsClQOrBJYF7AWjtgLXtgLpugKHvQLBwQKYxAIA",
        );
        let expected = vec![
            0, 1, 2, // Standard header
            1, 637,   // 1 Hero: Jaina,
            4,     // 4 Single Cards
            192,   // Ice Block
            39841, // Medivh, the Guardian
            42718, // Ghastly Conjurer
            42790, // Bonemare
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
            0,     // No 3+-copy cards
        ];
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn encode_complex_byte_array_as_deck_code() {
        let input = vec![
            0, 1, 2, // Standard header
            1, 637,   // 1 Hero: Jaina,
            4,     // 4 Single Cards
            192,   // Ice Block
            39841, // Medivh, the Guardian
            42718, // Ghastly Conjurer
            42790, // Bonemare
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
            0,     // No 3+-copy cards
        ];
        let expected = "AAECAf0EBMABobcC3s0Cps4CDXHDAbsClQOrBJYF7AWjtgLXtgLpugKHvQLBwQKYxAIA";
        assert_eq!(encode_u32_vec_to_deck_code(input), expected);
    }
}
