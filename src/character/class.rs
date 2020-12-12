use crate::character::AbilityScoreType;

#[derive(Debug)]
pub struct Class {
    name: String,
    key_ability: AbilityScoreType,
    hp_increment: u32,
}

impl Class {
    pub fn new(name: String, key_ability: AbilityScoreType, hp_increment: u32) -> Class {
        Class {
            name,
            key_ability,
            hp_increment,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn key_ability(&self) -> AbilityScoreType {
        self.key_ability
    }

    pub fn hp_increment(&self) -> u32 {
        self.hp_increment
    }
}
