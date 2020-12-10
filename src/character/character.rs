use crate::character::health::Health;
use std::cmp::{max, min};

struct AbilityScore {
    value: u32,
}

pub struct Character {
    name: String,
    description: String,
    level: u32,
    hp: Health,

    strength: AbilityScore,
    dexterity: AbilityScore,
    constituion: AbilityScore,
    intelligence: AbilityScore,
    wisdom: AbilityScore,
    charisma: AbilityScore,
}
