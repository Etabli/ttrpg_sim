use crate::character::ability_score::AbilityScoreSet;
use crate::character::health::Health;
use std::cmp::{max, min};

pub struct Character {
    name: String,
    description: String,
    level: u32,
    hp: Health,

    ability_score: AbilityScoreSet,
}

impl Character {
    fn new(name: &str) -> Character {
        Character {
            name: String::from(name),
            description: "".to_string(),
            level: 1,
            hp: Health::new(10),

            ability_score: AbilityScoreSet::new(),
        }
    }
}
