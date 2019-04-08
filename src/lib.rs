#![allow(clippy::all, clippy::pedantic)]

extern crate base64;

mod deck_reader;
mod format;

use crate::deck_reader::read;
use base64::{decode, encode};

fn decode_deck_code(deck_code: &str) {
    let decoded = decode(deck_code).expect("An error occured decoding the deck string");
    dbg!(&decoded);
    read(&decoded);
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[should_panic]
    #[test]
    fn decode_deck_code_panics_if_there_is_no_leading_0_byte() {
        decode_deck_code("AAECAf0EBMABobcC3s0Cps4CDXHDAbsClQOrBJYF7AWjtgLXtgLpugKHvQLBwQKYxAIA");
    }
}
