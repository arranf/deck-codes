extern crate deck_codes;
use deck_codes::{decode_deck_code, encode_deck_code, format::Format};

#[test]
fn standard_deck_matches() {
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

// ### Rogue
// # Class: Rogue
// # Format: Classic
// #
// # 2x (0) Backstab
// # 2x (0) Preparation
// # 2x (0) Shadowstep
// # 1x (1) Cold Blood
// # 1x (1) Conceal
// # 2x (1) Deadly Poison
// # 1x (2) Blade Flurry
// # 1x (2) Bloodmage Thalnos
// # 2x (2) Eviscerate
// # 2x (2) Sap
// # 2x (2) Shiv
// # 2x (3) Earthen Ring Farseer
// # 1x (3) Edwin VanCleef
// # 1x (3) Fan of Knives
// # 2x (3) SI:7 Agent
// # 1x (4) Leeroy Jenkins
// # 1x (5) Assassin's Blade
// # 2x (5) Azure Drake
// # 2x (5) Gadgetzan Auctioneer
// #
// AAEDAaIHCIKWBNyWBPigBIahBLWhBNyhBN+hBKWjBAv8lQT9lQTqlgT7lgT6oATUoQTdoQTkoQTnoQTooQSTogQA
// "
#[test]
fn classic_deck_matches() {
    let code =
        "AAEDAaIHCIKWBNyWBPigBIahBLWhBNyhBN+hBKWjBAv8lQT9lQTqlgT7lgT6oATUoQTdoQTkoQTnoQTooQSTogQA";
    let deck = decode_deck_code(code).expect("Decoded correctly");
    assert_eq!(deck.format, Format::Classic);
    assert_eq!(deck.heroes, vec![930]); // Code for Valeera
    assert_eq!(deck.total_cards(), 30);

    let expected_cards = vec![
        // Singles
        (1, 68354), // Assassin's Blade
        (1, 68444), // Fan of Knives
        (1, 69752), // Cold Blood
        (1, 69766), // Blade Flurry
        (1, 69813), // Bloodmage Thalnos
        (1, 69852), // Leeroy Jenkins
        (1, 69855), // Conceal
        (1, 70053), // Edwin VanCleef
        // Doubles
        (2, 68348), // Backstab
        (2, 68349), // Deadly Poison
        (2, 68458), // Shiv,
        (2, 68475), // Sap
        (2, 69754), // Earthen Ring Farseer
        (2, 69844), // Gadgetzan Auctioneer
        (2, 69853), // Eviscerate
        (2, 69860), // SI:7 Agent
        (2, 69863), // Shadowstep
        (2, 69864), // Preparation
        (2, 69907), // Azure Drake
    ];
    assert_eq!(deck.cards(), expected_cards);

    let reverse_code = encode_deck_code(&deck);
    assert_eq!(code, reverse_code);
}
