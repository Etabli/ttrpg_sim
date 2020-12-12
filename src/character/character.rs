use crate::character::{
    AbilityBoostChoice, AbilityBoostChoiceSet, AbilityScore, AbilityScoreSet, AbilityScoreType,
    Ancestry, Background, Class, Health,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

pub struct Character<'class, 'ancestry, 'background> {
    name: String,
    description: String,
    class: &'class Class,
    ancestry: &'ancestry Ancestry,
    background: &'background Background,
    level: u32,
    hp: Health,
    speed: u32,
    size: Size,

    ability_scores: AbilityScoreSet,
}

impl<'class, 'ancestry, 'background> Character<'class, 'ancestry, 'background> {
    pub fn new(
        name: &str,
        class: &'class Class,
        ancestry: &'ancestry Ancestry,
        ancestry_boost_choices: &Vec<AbilityScoreType>,
        background: &'background Background,
        background_boost_choices: &Vec<AbilityScoreType>,
        extra_boost_choices: &Vec<AbilityScoreType>,
    ) -> Result<Character<'class, 'ancestry, 'background>, String> {
        let extra_boosts: Vec<AbilityBoostChoice> = vec![
            AbilityBoostChoice::free(),
            AbilityBoostChoice::free(),
            AbilityBoostChoice::free(),
            AbilityBoostChoice::free(),
        ];

        Ok(Character {
            name: String::from(name),
            description: "".to_string(),
            class,
            ancestry,
            background,
            level: 1,
            hp: Health::new(class.hp_increment() + ancestry.base_hp()),
            speed: ancestry.speed(),
            size: ancestry.size(),

            ability_scores: AbilityScoreSet::with_boosts(&vec![
                &hashset![class.key_ability()],
                &ancestry
                    .ability_boosts()
                    .apply_choices(ancestry_boost_choices)?,
                &background
                    .ability_boosts()
                    .apply_choices(background_boost_choices)?,
                &extra_boosts.apply_choices(extra_boost_choices)?,
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

    pub fn background(&self) -> &Background {
        &self.background
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

    use crate::character::AbilityBoostChoice;

    #[test]
    fn class_stats() {
        let class = Class::new("Bob".to_string(), AbilityScoreType::Strength, 10);
        let ancestry = Ancestry::new("Bob".to_string(), 0, Size::Medium, 30, vec![]);
        let background = Background::new("Bob".to_string(), "".to_string(), vec![]);
        let character = Character::new(
            "Bob",
            &class,
            &ancestry,
            &vec![],
            &background,
            &vec![],
            &vec![
                AbilityScoreType::Strength,
                AbilityScoreType::Constitution,
                AbilityScoreType::Dexterity,
                AbilityScoreType::Wisdom,
            ],
        )
        .unwrap();

        assert_eq!(character.name(), "Bob");
        assert_eq!(character.level(), 1);
        assert_eq!(character.hp().max(), 10);
        assert_eq!(character.strength().value(), 14);
        assert_eq!(character.dexterity().value(), 12);
        assert_eq!(character.constitution().value(), 12);
        assert_eq!(character.intelligence().value(), 10);
        assert_eq!(character.wisdom().value(), 12);
        assert_eq!(character.charisma().value(), 10);
    }

    #[test]
    fn ancestry_stats() {
        let class = Class::new("Bob".to_string(), AbilityScoreType::Strength, 10);
        let ancestry = Ancestry::new(
            "Bob".to_string(),
            8,
            Size::Medium,
            30,
            vec![
                AbilityBoostChoice::predetermined(AbilityScoreType::Strength),
                AbilityBoostChoice::predetermined(AbilityScoreType::Constitution),
                AbilityBoostChoice::free(),
            ],
        );
        let background = Background::new("Bob".to_string(), "".to_string(), vec![]);
        let character = Character::new(
            "Bob",
            &class,
            &ancestry,
            &vec![AbilityScoreType::Dexterity],
            &background,
            &vec![],
            &vec![
                AbilityScoreType::Strength,
                AbilityScoreType::Constitution,
                AbilityScoreType::Dexterity,
                AbilityScoreType::Wisdom,
            ],
        )
        .unwrap();

        assert_eq!(character.speed(), ancestry.speed());
        assert_eq!(character.size(), ancestry.size());
        assert_eq!(character.strength().value(), 16);
        assert_eq!(character.dexterity().value(), 14);
        assert_eq!(character.constitution().value(), 14);
        assert_eq!(character.intelligence().value(), 10);
        assert_eq!(character.wisdom().value(), 12);
        assert_eq!(character.charisma().value(), 10);
    }

    #[test]
    fn background_stats() {
        let class = Class::new("Bob".to_string(), AbilityScoreType::Strength, 10);
        let ancestry = Ancestry::new(
            "Bob".to_string(),
            8,
            Size::Medium,
            30,
            vec![
                AbilityBoostChoice::predetermined(AbilityScoreType::Strength),
                AbilityBoostChoice::predetermined(AbilityScoreType::Constitution),
                AbilityBoostChoice::free(),
            ],
        );
        let background = Background::new(
            "Bob".to_string(),
            "".to_string(),
            vec![
                AbilityBoostChoice::restricted(hashset![
                    AbilityScoreType::Strength,
                    AbilityScoreType::Constitution,
                ]),
                AbilityBoostChoice::free(),
            ],
        );
        let character = Character::new(
            "Bob",
            &class,
            &ancestry,
            &vec![AbilityScoreType::Dexterity],
            &background,
            &vec![AbilityScoreType::Strength, AbilityScoreType::Constitution],
            &vec![
                AbilityScoreType::Strength,
                AbilityScoreType::Constitution,
                AbilityScoreType::Dexterity,
                AbilityScoreType::Wisdom,
            ],
        )
        .unwrap();

        assert_eq!(character.strength().value(), 18);
        assert_eq!(character.dexterity().value(), 14);
        assert_eq!(character.constitution().value(), 16);
        assert_eq!(character.intelligence().value(), 10);
        assert_eq!(character.wisdom().value(), 12);
        assert_eq!(character.charisma().value(), 10);
    }

    #[test]
    fn level_up() {
        let class = Class::new("Bob".to_string(), AbilityScoreType::Strength, 10);
        let ancestry = Ancestry::new(
            "Bob".to_string(),
            8,
            Size::Medium,
            30,
            vec![
                AbilityBoostChoice::predetermined(AbilityScoreType::Strength),
                AbilityBoostChoice::predetermined(AbilityScoreType::Constitution),
                AbilityBoostChoice::free(),
            ],
        );
        let background = Background::new(
            "Bob".to_string(),
            "".to_string(),
            vec![
                AbilityBoostChoice::restricted(hashset![
                    AbilityScoreType::Strength,
                    AbilityScoreType::Constitution,
                ]),
                AbilityBoostChoice::free(),
            ],
        );
        let mut character = Character::new(
            "Bob",
            &class,
            &ancestry,
            &vec![AbilityScoreType::Dexterity],
            &background,
            &vec![AbilityScoreType::Strength, AbilityScoreType::Constitution],
            &vec![
                AbilityScoreType::Strength,
                AbilityScoreType::Constitution,
                AbilityScoreType::Dexterity,
                AbilityScoreType::Wisdom,
            ],
        )
        .unwrap();

        character.level_up();

        assert_eq!(character.level(), 2);
        assert_eq!(character.hp().max(), 28);
    }
}
