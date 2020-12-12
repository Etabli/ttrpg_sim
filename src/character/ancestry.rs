use std::collections::HashSet;

use crate::character::AbilityScoreType;
use crate::character::Size;

#[derive(Debug)]
pub struct Ancestry {
    name: String,
    base_hp: u32,
    size: Size,
    speed: u32,
    ability_boosts: HashSet<AbilityScoreType>,
    free_boosts: u8,
}

impl Ancestry {
    pub fn new(
        name: String,
        base_hp: u32,
        size: Size,
        speed: u32,
        ability_boosts: HashSet<AbilityScoreType>,
        free_boosts: u8,
    ) -> Ancestry {
        Ancestry {
            name,
            base_hp,
            size,
            speed,
            ability_boosts,
            free_boosts,
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

    pub fn get_ability_boosts(
        &self,
        free_boosts: &HashSet<AbilityScoreType>,
    ) -> Result<HashSet<AbilityScoreType>, String> {
        if free_boosts.len() != self.free_boosts as usize {
            return Err(format!(
                "Invalid number of free boosts: expected {}, got {}!",
                self.free_boosts,
                free_boosts.len()
            ));
        } else if free_boosts
            .iter()
            .any(|free_boost| self.ability_boosts.contains(free_boost))
        {
            return Err("Free boosts cannot overlap with predetermined boosts!".to_string());
        }

        Ok(self
            .ability_boosts
            .iter()
            .cloned()
            .chain(free_boosts.iter().cloned())
            .collect())
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
            hashset![AbilityScoreType::Strength, AbilityScoreType::Constitution],
            1,
        );

        let boosts = ancestry
            .get_ability_boosts(&hashset![AbilityScoreType::Dexterity])
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
    fn get_ability_boosts_incorrect_number_of_free_boosts() {
        let ancestry = Ancestry::new(
            "Bob".to_string(),
            8,
            Size::Medium,
            30,
            hashset![AbilityScoreType::Strength, AbilityScoreType::Constitution],
            1,
        );

        ancestry
            .get_ability_boosts(&hashset![
                AbilityScoreType::Dexterity,
                AbilityScoreType::Strength,
            ])
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn get_ability_boosts_incorrect_duplicate_free_boost() {
        let ancestry = Ancestry::new(
            "Bob".to_string(),
            8,
            Size::Medium,
            30,
            hashset![AbilityScoreType::Strength, AbilityScoreType::Constitution],
            1,
        );

        ancestry
            .get_ability_boosts(&hashset![AbilityScoreType::Strength])
            .unwrap();
    }
}
