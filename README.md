# deck_codes
[![CircleCI](https://circleci.com/gh/arranf/deck-codes/tree/master.svg?style=svg)](https://circleci.com/gh/arranf/deck_codes/tree/master)

[![Crate](https://img.shields.io/crates/v/deck_codes.svg)](https://crates.io/crates/deck_codes)

A Rust library for encoding and decoding Hearthstone deck codes or deckstrings.

Examples of deck codes can be found [here](https://hearthsim.info/docs/deckstrings/).

Any deckstring or deck definition returned by this library will be canonical.
This means that the cards and heroes are sorted in ascending order by dbf id.

A mapping between dbf ids and cards can be found at [HearthstoneJSON](https://hearthstonejson.com/).

## Usage

```rust
extern deck_codes;
use deck_codes::{decode_deck_code, encode_deck_code, format::Format};

fn main() {
    let code = "AAECAf0EBMABobcC3s0Cps4CDXHDAbsClQOrBJYF7AWjtgLXtgLpugKHvQLBwQKYxAIA";
    let deck = decode_deck_code(code).expect("Decoded safely");
    assert_eq!(deck.format, Format::Standard);
    assert_eq!(deck.heroes, vec![637]); // dbfid for Jaina
    assert_eq!(deck.total_cards(), 30);

    let expected_cards = vec![
        // Singles
        (1, 192), (1, 39841), (1, 42718), (1, 42790),
        // Doubles
        (2, 113), (2, 195), (2, 315), (2, 405), (2, 555), (2, 662), (2, 748),
        (2, 39715), (2, 39767), (2, 40297), (2, 40583), (2, 41153), (2, 41496)
    ];
    assert_eq!(deck.cards(), expected_cards);

    let reverse_code = encode_deck_code(deck);
    assert_eq!(code, reverse_code);
}
```

