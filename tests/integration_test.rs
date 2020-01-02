extern crate deck_codes;
use deck_codes::{decode_deck_code, encode_deck_code, format::Format};

#[test]
fn deck_matches() {
    let code = "AAECAf0EBMABobcC3s0Cps4CDXHDAbsClQOrBJYF7AWjtgLXtgLpugKHvQLBwQKYxAIA";
    let deck = decode_deck_code(code).expect("Decoded correctly");
    assert_eq!(deck.format, Format::Standard);
    assert_eq!(deck.heroes, vec![637]); // Code for Jaina
    assert_eq!(deck.total_cards(), 30);

    let expected_cards = vec![
        // Singles
        (1, 192),
        (1, 39841),
        (1, 42718),
        (1, 42790),
        // Doubles
        (2, 113),
        (2, 195),
        (2, 315),
        (2, 405),
        (2, 555),
        (2, 662),
        (2, 748),
        (2, 39715),
        (2, 39767),
        (2, 40297),
        (2, 40583),
        (2, 41153),
        (2, 41496),
    ];
    assert_eq!(deck.cards(), expected_cards);

    let reverse_code = encode_deck_code(&deck);
    assert_eq!(code, reverse_code);
}
