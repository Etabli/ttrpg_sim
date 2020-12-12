use crate::character::{AbilityScore, AbilityScoreSet, AbilityScoreType, Ancestry, Class, Health};
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

pub struct Character<'class, 'ancestry> {
    name: String,
    description: String,
    class: &'class Class,
    ancestry: &'ancestry Ancestry,
    level: u32,
    hp: Health,
    speed: u32,
    size: Size,

    ability_scores: AbilityScoreSet,
}

impl<'class, 'ancestry> Character<'class, 'ancestry> {
    pub fn new(
        name: &str,
        class: &'class Class,
        ancestry: &'ancestry Ancestry,
        ancestry_free_boosts: &HashSet<AbilityScoreType>,
    ) -> Result<Character<'class, 'ancestry>, String> {
        Ok(Character {
            name: String::from(name),
            description: "".to_string(),
            class,
            ancestry,
            level: 1,
            hp: Health::new(class.hp_increment() + ancestry.base_hp()),
            speed: ancestry.speed(),
            size: ancestry.size(),

            ability_scores: AbilityScoreSet::with_boosts(&vec![
                &hashset![class.key_ability()],
                &ancestry.get_ability_boosts(ancestry_free_boosts)?,
            ]),
        })
    }

    // ============================= Accessors =============================

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn class(&self) -> &Class {
        &self.class
    }

    pub fn ancestry(&self) -> &Ancestry {
        &self.ancestry
    }

    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn hp(&self) -> &Health {
        &self.hp
    }

    pub fn speed(&self) -> u32 {
        self.speed
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn abliity_scores(&self) -> &AbilityScoreSet {
        &self.ability_scores
    }

    pub fn strength(&self) -> AbilityScore {
        self.ability_scores.get(AbilityScoreType::Strength)
    }

    pub fn dexterity(&self) -> AbilityScore {
        self.ability_scores.get(AbilityScoreType::Dexterity)
    }

    pub fn constitution(&self) -> AbilityScore {
        self.ability_scores.get(AbilityScoreType::Constitution)
    }

    pub fn intelligence(&self) -> AbilityScore {
        self.ability_scores.get(AbilityScoreType::Intelligence)
    }

    pub fn wisdom(&self) -> AbilityScore {
        self.ability_scores.get(AbilityScoreType::Wisdom)
    }

    pub fn charisma(&self) -> AbilityScore {
        self.ability_scores.get(AbilityScoreType::Charisma)
    }

    // ============================= Logic =============================

    pub fn level_up(&mut self) {
        self.level += 1;
        self.hp.increase_max(self.class.hp_increment());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn character_from_class() {
        let class = Class::new("Bob".to_string(), AbilityScoreType::Strength, 10);
        let ancestry = Ancestry::new("Bob".to_string(), 0, Size::Medium, 30, hashset![], 0);
        let character = Character::new("bob", &class, &ancestry, &hashset![]).unwrap();

        assert_eq!(character.name(), "bob");
        assert_eq!(character.level(), 1);
        assert_eq!(character.hp().max(), 10);
        assert_eq!(character.strength().value(), 12);
        assert_eq!(character.dexterity().value(), 10);
        assert_eq!(character.constitution().value(), 10);
        assert_eq!(character.intelligence().value(), 10);
        assert_eq!(character.wisdom().value(), 10);
        assert_eq!(character.charisma().value(), 10);
    }

    #[test]
    fn character_from_ancestry() {
        let class = Class::new("Bob".to_string(), AbilityScoreType::Strength, 10);
        let ancestry = Ancestry::new(
            "Bob".to_string(),
            8,
            Size::Medium,
            30,
            hashset![AbilityScoreType::Strength, AbilityScoreType::Constitution],
            1,
        );
        let character = Character::new(
            "bob",
            &class,
            &ancestry,
            &hashset![AbilityScoreType::Dexterity],
        )
        .unwrap();

        assert_eq!(character.name(), "bob");
        assert_eq!(character.level(), 1);
        assert_eq!(character.hp().max(), 18);
        assert_eq!(character.speed(), ancestry.speed());
        assert_eq!(character.size(), ancestry.size());
        assert_eq!(character.strength().value(), 14);
        assert_eq!(character.dexterity().value(), 12);
        assert_eq!(character.constitution().value(), 12);
        assert_eq!(character.intelligence().value(), 10);
        assert_eq!(character.wisdom().value(), 10);
        assert_eq!(character.charisma().value(), 10);
    }

    #[test]
    fn character_level_up() {
        let class = Class::new("Bob".to_string(), AbilityScoreType::Strength, 10);
        let ancestry = Ancestry::new(
            "Bob".to_string(),
            8,
            Size::Medium,
            30,
            hashset![AbilityScoreType::Strength, AbilityScoreType::Constitution],
            1,
        );
        let mut character = Character::new(
            "bob",
            &class,
            &ancestry,
            &hashset![AbilityScoreType::Dexterity],
        )
        .unwrap();

        character.level_up();

        assert_eq!(character.level(), 2);
        assert_eq!(character.hp().max(), 28);
    }
}
