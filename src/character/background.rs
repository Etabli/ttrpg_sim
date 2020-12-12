use crate::character::AbilityBoostChoice;

pub struct Background {
    name: String,
    description: String,
    ability_boosts: Vec<AbilityBoostChoice>,
}

impl Background {
    pub fn new(name: String, description: String, ability_boosts: Vec<AbilityBoostChoice>) -> Self {
        Self {
            name,
            description,
            ability_boosts,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn ability_boosts(&self) -> &Vec<AbilityBoostChoice> {
        &self.ability_boosts
    }
}
