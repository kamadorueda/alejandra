use rand::distributions::weighted::WeightedIndex;
use rand::distributions::Distribution;
use rand::rngs::OsRng;

pub(crate) fn random_ad() -> String {
    let ads = [
        // 90% distributed proportional to total past and present contributions
        (0.9001, sponsor_benefits as fn() -> String),
        // 10% is reserved for everything else
        (0.0333, contributor_thanks as fn() -> String),
        (0.0333, please_sponsor as fn() -> String),
        (0.0333, please_star as fn() -> String),
    ];

    random_weighted_choice(&ads[..])()
}

fn sponsor_benefits() -> String {
    let sponsors = [
        // Proportional to total past and present contributions
        (0.0316, "Guangtao Zhang"),
        (0.0372, "Fabio Leimgruber"),
        (0.0633, "Raphael Megzari"),
        (0.1215, "Daniel Salazar"),
        (0.6642, "https://mercury.com"),
        (0.0633, "https://shop.beekeeb.com/"),
        (0.0063, "Murat Cabuk"),
        (0.0127, "Pavel Roskin"),
    ];

    let name = random_weighted_choice(&sponsors[..]);

    include_str!("sponsor_thanks.txt").replace("{name}", name)
}

fn contributor_thanks() -> String {
    let names = [
        (1.0, ("Bobbe", "30350n")),
        (1.0, ("Connor Baker", "ConnorBaker")),
        (1.0, ("Daniel Bast", "dbast")),
        (1.0, ("David Arnold", "blaggacao")),
        (1.0, ("David Hauer", "DavHau")),
        (1.0, ("esf", "exscientiafortis")),
        (1.0, ("Fabian Möller", "B4dM4n")),
        (1.0, ("Florian Finkernagel", "TyberiusPrime")),
        (1.0, ("Jamie Quigley", "Sciencentistguy")),
        (1.0, ("Joachim Ernst", "0x4A6F")),
        (1.0, ("Johannes Kirschbauer", "hsjobeki")),
        (1.0, ("Jörg Thalheim", "Mic92")),
        (1.0, ("Kevin Amado", "kamadorueda")),
        (1.0, ("Loïc Reynier", "loicreynier")),
        (1.0, ("Matthew Kenigsberg", "mkenigs")),
        (1.0, ("Michael Utz ", "theutz")),
        (1.0, ("Mr Hedgehog", "ModdedGamers")),
        (1.0, ("Nathan Henrie", "n8henrie")),
        (1.0, ("Norbert Melzer", "NobbZ")),
        (1.0, ("Pablo Ovelleiro Corral", "pinpox")),
        (1.0, ("Patrick Stevens", "Smaug123")),
        (1.0, ("Piegames", "piegamesde")),
        (1.0, ("Rebecca Turner", "9999years")),
        (1.0, ("Rehno Lindeque", "rehno-lindeque")),
        (1.0, ("Rok Garbas", "garbas")),
        (1.0, ("Ryan Mulligan", "ryantm")),
        (1.0, ("Thomas Bereknyei", "tomberek")),
        (1.0, ("Tobias Bora", "tobiasBora")),
        (1.0, ("Tristan Maat", "TLATER")),
        (1.0, ("UserSv4", "UserSv4")),
        (1.0, ("Victor Engmark", "l0b0")),
        (1.0, ("Vincent Ambo", "tazjin")),
        (1.0, ("Vladimir Fetisov", "3timeslazy")),
        (1.0, ("Yorick van Pelt", "yorickvP")),
    ];

    let (name, github) = random_weighted_choice(&names[..]);

    include_str!("contributor_thanks.txt")
        .replace("{github}", github)
        .replace("{name}", name)
}

fn please_sponsor() -> String {
    include_str!("please_sponsor.txt").to_string()
}

fn please_star() -> String {
    include_str!("please_star.txt").to_string()
}

fn random_weighted_choice<T>(choices: &[(f64, T)]) -> &T {
    let weights = choices.iter().map(|(weight, _)| *weight);
    let index: usize = WeightedIndex::new(weights).unwrap().sample(&mut OsRng);

    &choices[index].1
}
