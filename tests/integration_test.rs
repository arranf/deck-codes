extern crate deck_codes;
use deck_codes::{decode_deck_code, encode_deck_code, format::Format};

#[test]
fn pre_sideboard_standard_deck_string_matches_deck() {
    let code = "AAECAf0EBMABobcC3s0Cps4CDXHDAbsClQOrBJYF7AWjtgLXtgLpugKHvQLBwQKYxAIA";
    let deck = decode_deck_code(code).expect("Decoded correctly");
    assert_eq!(deck.format, Format::Standard);
    assert_eq!(deck.heroes, vec![637]); // Code for Jaina
    assert_eq!(deck.total_cards(), 30);

    let expected_cards = vec![
        // Singles
        (1, 192, None),
        (1, 39841, None),
        (1, 42718, None),
        (1, 42790, None),
        // Doubles
        (2, 113, None),
        (2, 195, None),
        (2, 315, None),
        (2, 405, None),
        (2, 555, None),
        (2, 662, None),
        (2, 748, None),
        (2, 39715, None),
        (2, 39767, None),
        (2, 40297, None),
        (2, 40583, None),
        (2, 41153, None),
        (2, 41496, None),
    ];
    assert_eq!(deck.cards(), expected_cards);
}

#[test]
fn post_sideboard_standard_deck_string_matches_deck_and_string() {
    let code = "AAECAf0EBMABobcC3s0Cps4CDXHDAbsClQOrBJYF7AWjtgLXtgLpugKHvQLBwQKYxAIAAA==";
    let deck = decode_deck_code(code).expect("Decoded correctly");
    assert_eq!(deck.format, Format::Standard);
    assert_eq!(deck.heroes, vec![637]); // Code for Jaina
    assert_eq!(deck.total_cards(), 30);

    let expected_cards = vec![
        // Singles
        (1, 192, None),
        (1, 39841, None),
        (1, 42718, None),
        (1, 42790, None),
        // Doubles
        (2, 113, None),
        (2, 195, None),
        (2, 315, None),
        (2, 405, None),
        (2, 555, None),
        (2, 662, None),
        (2, 748, None),
        (2, 39715, None),
        (2, 39767, None),
        (2, 40297, None),
        (2, 40583, None),
        (2, 41153, None),
        (2, 41496, None),
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
fn pre_sideboard_classic_deck_matches_deck() {
    let code =
        "AAEDAaIHCIKWBNyWBPigBIahBLWhBNyhBN+hBKWjBAv8lQT9lQTqlgT7lgT6oATUoQTdoQTkoQTnoQTooQSTogQA";
    let deck = decode_deck_code(code).expect("Decoded correctly");
    assert_eq!(deck.format, Format::Classic);
    assert_eq!(deck.heroes, vec![930]); // Code for Valeera
    assert_eq!(deck.total_cards(), 30);

    let expected_cards = vec![
        // Singles
        (1, 68354, None), // Assassin's Blade
        (1, 68444, None), // Fan of Knives
        (1, 69752, None), // Cold Blood
        (1, 69766, None), // Blade Flurry
        (1, 69813, None), // Bloodmage Thalnos
        (1, 69852, None), // Leeroy Jenkins
        (1, 69855, None), // Conceal
        (1, 70053, None), // Edwin VanCleef
        // Doubles
        (2, 68348, None), // Backstab
        (2, 68349, None), // Deadly Poison
        (2, 68458, None), // Shiv,
        (2, 68475, None), // Sap
        (2, 69754, None), // Earthen Ring Farseer
        (2, 69844, None), // Gadgetzan Auctioneer
        (2, 69853, None), // Eviscerate
        (2, 69860, None), // SI:7 Agent
        (2, 69863, None), // Shadowstep
        (2, 69864, None), // Preparation
        (2, 69907, None), // Azure Drake
    ];
    assert_eq!(deck.cards(), expected_cards);
}

#[test]
fn post_sideboard_classic_deck_matches_deck_and_code() {
    let code =
        "AAEDAaIHCIKWBNyWBPigBIahBLWhBNyhBN+hBKWjBAv8lQT9lQTqlgT7lgT6oATUoQTdoQTkoQTnoQTooQSTogQAAA==";
    let deck = decode_deck_code(code).expect("Decoded correctly");
    assert_eq!(deck.format, Format::Classic);
    assert_eq!(deck.heroes, vec![930]); // Code for Valeera
    assert_eq!(deck.total_cards(), 30);

    let expected_cards = vec![
        // Singles
        (1, 68354, None), // Assassin's Blade
        (1, 68444, None), // Fan of Knives
        (1, 69752, None), // Cold Blood
        (1, 69766, None), // Blade Flurry
        (1, 69813, None), // Bloodmage Thalnos
        (1, 69852, None), // Leeroy Jenkins
        (1, 69855, None), // Conceal
        (1, 70053, None), // Edwin VanCleef
        // Doubles
        (2, 68348, None), // Backstab
        (2, 68349, None), // Deadly Poison
        (2, 68458, None), // Shiv,
        (2, 68475, None), // Sap
        (2, 69754, None), // Earthen Ring Farseer
        (2, 69844, None), // Gadgetzan Auctioneer
        (2, 69853, None), // Eviscerate
        (2, 69860, None), // SI:7 Agent
        (2, 69863, None), // Shadowstep
        (2, 69864, None), // Preparation
        (2, 69907, None), // Azure Drake
    ];

    let reverse_code = encode_deck_code(&deck);
    assert_eq!(code, reverse_code);
    assert_eq!(deck.cards(), expected_cards);
}

#[test]
fn post_sideboard_wild_dk_reverse() {
    let code = "AAEBAfHhBAK0gAW0gAUO9eMEguQEseYEjvEErqEF88gF6oAG6akG/7oG/8kGkMsGpdwGpPQGpvQGAAA=";
    let deck = decode_deck_code(code).expect("Decoded correctly");
    assert_eq!(deck.format, Format::Wild);
    assert_eq!(deck.heroes, vec![78065]); // Code for The Lich King

    let reverse_code = encode_deck_code(&deck);
    assert_eq!(code, reverse_code);
}

#[test]
fn standard_sideboard_deck() {
    let code =
        "AAECAfHhBB6H9gS0gAX9xAWt6QWC+AX8+QWT+wXt/wXWgAaFjgaUlQb/lwbQngaSoAbHpAavqAa7sQb/uga/vgbDvgakwAamwAb/yQaWywa6zgag4gbR5QbC6Aaq6gbt6gYAAAEGrekF/cQFu7EG/cQF9bMGx6QG97MGx6QG694Gx6QG6e0G/cQFAAA=";
    let deck = decode_deck_code(code).expect("Decoded correctly");
    assert_eq!(deck.format, Format::Standard);
    assert_eq!(deck.heroes, vec![78065]); // Code for The Lich King
    assert_eq!(deck.total_cards(), 30);

    let expected_cards = vec![
        // Singles
        (1, 80647, None),  // Chillfallen Baron
        (1, 81972, None),  // Harbringer of Winter
        (1, 90749, None),  // E.T.C., Band Manager
        (1, 95405, None),  // Speaker Stomper
        (1, 97282, None),  // Down with the Ship
        (1, 97532, None),  // Climactic Necrotic Explosion
        (1, 97683, None),  // Helya
        (1, 98285, None),  // The Primus
        (1, 98390, None),  // Cold Feet
        (1, 100101, None), // Miracle Salesman
        (1, 101012, None), // Runes of Darkness
        (1, 101375, None), // Reska, the Pit Boss
        (1, 102224, None), // Cult Neophyte
        (1, 102418, None), // Mining Casulties
        (1, 102983, None), // Zilliax Deluxe 3000
        (1, 103471, None), // Reno, Lone Ranger
        (1, 104635, None), // Threads of Despair
        (1, 105855, None), // Dreadhound Handler
        (1, 106303, None), // Razzle-Dazzler
        (1, 106307, None), // Frosty Décor
        (1, 106532, None), // Malted Magma
        (1, 106534, None), // Meltemental
        (1, 107775, None), // Corpsicle
        (1, 107926, None), // Buttons
        (1, 108346, None), // Gorgonzormu
        (1, 110880, None), // Airlock Breach
        (1, 111313, None), // Troubled Mechanic
        (1, 111682, None), // Exarch Maladaar
        (1, 111914, None), // The Ceaseless Expanse
        (1, 111981, None), // Eredar Brute
        // Sideboard
        (1, 95405, Some(90749)),   // Speaker_Stomper
        (1, 104635, Some(90749)),  // Threads of Despair
        (1, 104949, Some(102983)), // Twin Module
        (1, 104951, Some(102983)), // Perfect Module
        (1, 110443, Some(102983)), // Zilliax Deluxe 3000
        (1, 112361, Some(90749)),  // Kil'jaeden
    ];

    let reverse_code = encode_deck_code(&deck);
    assert_eq!(deck.cards(), expected_cards);
    assert_eq!(code, reverse_code);
}

// ### Dungar Druid
// # Class: Druid
// # Format: Standard
// #
// # 2x (0) Innervate
// # 2x (1) Malfurion's Gift
// # 2x (2) Trail Mix
// # 2x (3) New Heights
// # 2x (3) Pendant of Earth
// # 2x (4) Arkonite Defense Crystal
// # 2x (4) Oaken Summons
// # 2x (6) Crystal Cluster
// # 1x (7) Beached Whale
// # 2x (8) Hydration Station
// # 2x (8) Splitting Spacerock
// # 2x (8) Star Grazer
// # 1x (8) Thunderbringer
// # 1x (9) Travelmaster Dungar
// # 1x (9) Zilliax Deluxe 3000
// #   1x (0) Zilliax Deluxe 3000
// #   1x (4) Virus Module
// #   1x (5) Perfect Module
// # 1x (10) Eonar, the Life-Binder
// # 2x (10) Factory Assemblybot
// # 1x (10) Yogg-Saron, Unleashed
// #
// AAECAZICBp/zBamVBvajBsekBtrBBoviBgyunwSaoAagoAaHqAbvqQbDugbQygbzygaL3Aad4wb75Qad6wYAAQP0swbHpAb3swbHpAbo3gbHpAYAAA==

#[test]
fn sideboard_standard_reverse() {
    let code = "AAECAZICBp/zBamVBvajBsekBtrBBoviBgyunwSaoAagoAaHqAbvqQbDugbQygbzygaL3Aad4wb75Qad6wYAAQP0swbHpAb3swbHpAbo3gbHpAYAAA==";
    let deck = decode_deck_code(code).expect("Decoded correctly");
    assert_eq!(deck.format, Format::Standard);
    assert_eq!(deck.heroes, vec![274]); // Code for Malfurion

    let reverse_code = encode_deck_code(&deck);
    assert_eq!(code, reverse_code);
}
