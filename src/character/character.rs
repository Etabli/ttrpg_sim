use crate::character::AbilityScoreSet;
use crate::character::Class;
use crate::character::Health;

pub struct Character<'class> {
    name: String,
    description: String,
    class: &'class Class,
    level: u32,
    hp: Health,

    ability_scores: AbilityScoreSet,
}

impl<'class> Character<'class> {
    pub fn new(name: &str, class: &'class Class) -> Character<'class> {
        Character {
            name: String::from(name),
            description: "".to_string(),
            class,
            level: 1,
            hp: Health::new(class.hp_increment()),

            ability_scores: AbilityScoreSet::with_boosts(vec![class.key_ability()]),
        }
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        self.hp.increase_max(self.class.hp_increment());
    }

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

    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn hp(&self) -> &Health {
        &self.hp
    }

    pub fn abliity_scores(&self) -> &AbilityScoreSet {
        &self.ability_scores
    }
}

#[cfg(test)]
mod tests {
    use crate::character::AbilityScoreType;

    use super::*;

    #[test]
    fn character_from_class() {
        let class = Class::new("Bob".to_string(), AbilityScoreType::Strength, 10);
        let character = Character::new("bob", &class);

        assert_eq!(character.name(), "bob");
        assert_eq!(character.level(), 1);
        assert_eq!(character.hp().max(), 10);
        assert_eq!(
            character
                .abliity_scores()
                .get(AbilityScoreType::Strength)
                .value(),
            12
        );
        assert_eq!(
            character
                .abliity_scores()
                .get(AbilityScoreType::Dexterity)
                .value(),
            10
        );
        assert_eq!(
            character
                .abliity_scores()
                .get(AbilityScoreType::Constitution)
                .value(),
            10
        );
        assert_eq!(
            character
                .abliity_scores()
                .get(AbilityScoreType::Intelligence)
                .value(),
            10
        );
        assert_eq!(
            character
                .abliity_scores()
                .get(AbilityScoreType::Wisdom)
                .value(),
            10
        );
        assert_eq!(
            character
                .abliity_scores()
                .get(AbilityScoreType::Charisma)
                .value(),
            10
        );
    }
}
