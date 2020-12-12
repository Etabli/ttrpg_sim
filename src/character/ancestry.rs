use crate::character::Size;
use crate::character::{AbilityBoostChoice, AbilityBoostChoiceSet, AbilityScoreType};

#[derive(Debug)]
pub struct Ancestry {
    name: String,
    base_hp: u32,
    size: Size,
    speed: u32,
    ability_boosts: Vec<AbilityBoostChoice>,
}

impl Ancestry {
    pub fn new(
        name: String,
        base_hp: u32,
        size: Size,
        speed: u32,
        ability_boosts: Vec<AbilityBoostChoice>,
    ) -> Ancestry {
        Ancestry {
            name,
            base_hp,
            size,
            speed,
            ability_boosts,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn base_hp(&self) -> u32 {
        self.base_hp
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn speed(&self) -> u32 {
        self.speed
    }

    pub fn ability_boosts(&self) -> &Vec<AbilityBoostChoice> {
        &self.ability_boosts
    }
}
