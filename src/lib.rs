#![allow(clippy::all, clippy::pedantic)]

extern crate base64;
extern crate integer_encoding;

mod deck_reader;
mod format;

use crate::deck_reader::read;
use base64::{decode, encode};
use integer_encoding::VarInt;

pub fn decode_deck_code(deck_code: &str) {
    dbg!(decode_code_to_u8_vec(deck_code));
}

/// Turns a Base64 deck code into a vector of u32 values that can then be mapped to the format of the deck
fn decode_code_to_u8_vec(deck_code: &str) -> Vec<u32> {
    let mut decoded = decode(deck_code).expect("An error occured decoding the deck string");

    let mut deck_code_decoded: Vec<u32> = vec![];
    // Read u8 values as u32 varints
    while decoded.len() > 0 {
        let (read, size) = u32::decode_var(&decoded);
        deck_code_decoded.push(read);
        decoded = decoded[size..].to_vec();
    }
    return deck_code_decoded;
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[should_panic]
    #[test]
    fn decode_code_to_u8_vec_correctly_decodes_a_simple_code() {
        let output = decode_code_to_u8_vec("AAEBAQcAAAQBAwIDAwMEAw==");
        // Expected:
        // Null Byte
        // Versio
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
        assert_eq!(output, expected);
    }

    #[test]
    fn decode_code_to_u8_vec_correctly_decodes_a_complex_code() {
        let output = decode_code_to_u8_vec(
            "AAECAf0EBMABobcC3s0Cps4CDXHDAbsClQOrBJYF7AWjtgLXtgLpugKHvQLBwQKYxAIA",
        );
        let expected = vec![
            0, 1, 2, // Standard header
            1, 637,   // Jaina,
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
        assert_eq!(output, expected);
    }
}
