use std::collections::HashSet;

use crate::character::Size;
use crate::character::{AbilityBoostChoice, AbilityBoostChoiceSet, AbilityScoreType};

#[derive(Debug)]
pub struct Ancestry {
    name: String,
    base_hp: u32,
    size: Size,
    speed: u32,
    ability_boosts: HashSet<AbilityBoostChoice>,
}

impl Ancestry {
    pub fn new(
        name: String,
        base_hp: u32,
        size: Size,
        speed: u32,
        ability_boosts: HashSet<AbilityBoostChoice>,
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

    pub fn ability_boosts(&self) -> &HashSet<AbilityBoostChoice> {
        &self.ability_boosts
    }

    pub fn get_ability_boosts(
        &self,
        boost_choices: &Vec<AbilityScoreType>,
    ) -> Result<HashSet<AbilityScoreType>, String> {
        self.ability_boosts.apply_choices(boost_choices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_ability_boosts_works() {
        let ancestry = Ancestry::new(
            "Bob".to_string(),
            8,
            Size::Medium,
            30,
            hashset![
                AbilityBoostChoice::predetermined(AbilityScoreType::Strength),
                AbilityBoostChoice::predetermined(AbilityScoreType::Constitution),
                AbilityBoostChoice::free(),
            ],
        );

        let boosts = ancestry
            .get_ability_boosts(&vec![AbilityScoreType::Dexterity])
            .unwrap();
        assert_eq!(
            boosts,
            hashset![
                AbilityScoreType::Strength,
                AbilityScoreType::Constitution,
                AbilityScoreType::Dexterity
            ]
        );
    }

    #[test]
    #[should_panic]
    fn get_ability_boosts_too_many_boost_choices() {
        let ancestry = Ancestry::new(
            "Bob".to_string(),
            8,
            Size::Medium,
            30,
            hashset![
                AbilityBoostChoice::predetermined(AbilityScoreType::Strength),
                AbilityBoostChoice::predetermined(AbilityScoreType::Constitution),
                AbilityBoostChoice::free(),
            ],
        );

        ancestry
            .get_ability_boosts(&vec![
                AbilityScoreType::Dexterity,
                AbilityScoreType::Strength,
            ])
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn get_ability_boosts_too_few_boost_choices() {
        let ancestry = Ancestry::new(
            "Bob".to_string(),
            8,
            Size::Medium,
            30,
            hashset![
                AbilityBoostChoice::predetermined(AbilityScoreType::Strength),
                AbilityBoostChoice::predetermined(AbilityScoreType::Constitution),
                AbilityBoostChoice::free(),
            ],
        );

        ancestry.get_ability_boosts(&vec![]).unwrap();
    }

    #[test]
    #[should_panic]
    fn get_ability_boosts_incorrect_duplicate_free_boost() {
        let ancestry = Ancestry::new(
            "Bob".to_string(),
            8,
            Size::Medium,
            30,
            hashset![
                AbilityBoostChoice::predetermined(AbilityScoreType::Strength),
                AbilityBoostChoice::predetermined(AbilityScoreType::Constitution),
                AbilityBoostChoice::free(),
            ],
        );

        ancestry
            .get_ability_boosts(&vec![AbilityScoreType::Strength])
            .unwrap();
    }
}
