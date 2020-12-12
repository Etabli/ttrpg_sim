use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AbilityScore {
    value: u32,
}

impl AbilityScore {
    pub fn new(value: u32) -> AbilityScore {
        AbilityScore { value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn boost(&mut self) {
        if self.value < 18 {
            self.value += 2
        } else {
            self.value += 1;
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AbilityScoreType {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Debug)]
pub struct AbilityScoreSet {
    strength: AbilityScore,
    dexterity: AbilityScore,
    constitution: AbilityScore,
    intelligence: AbilityScore,
    wisdom: AbilityScore,
    charisma: AbilityScore,
}

impl AbilityScoreSet {
    pub fn new() -> AbilityScoreSet {
        AbilityScoreSet {
            strength: AbilityScore::new(10),
            dexterity: AbilityScore::new(10),
            constitution: AbilityScore::new(10),
            intelligence: AbilityScore::new(10),
            wisdom: AbilityScore::new(10),
            charisma: AbilityScore::new(10),
        }
    }

    pub fn with_boosts(boosts: &Vec<&HashSet<AbilityScoreType>>) -> AbilityScoreSet {
        let mut set = AbilityScoreSet::new();
        for &boost in boosts.iter() {
            set.boost(boost);
        }
        set
    }

    pub fn boost(&mut self, boosts: &HashSet<AbilityScoreType>) {
        for score in boosts.iter() {
            match score {
                AbilityScoreType::Strength => self.strength.boost(),
                AbilityScoreType::Dexterity => self.dexterity.boost(),
                AbilityScoreType::Constitution => self.constitution.boost(),
                AbilityScoreType::Intelligence => self.intelligence.boost(),
                AbilityScoreType::Wisdom => self.wisdom.boost(),
                AbilityScoreType::Charisma => self.charisma.boost(),
            }
        }
    }

    pub fn get(&self, ability_score: AbilityScoreType) -> AbilityScore {
        match ability_score {
            AbilityScoreType::Strength => self.strength,
            AbilityScoreType::Dexterity => self.dexterity,
            AbilityScoreType::Constitution => self.constitution,
            AbilityScoreType::Intelligence => self.intelligence,
            AbilityScoreType::Wisdom => self.wisdom,
            AbilityScoreType::Charisma => self.charisma,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ability_score_boost_below_18() {
        let mut score = AbilityScore::new(10);
        score.boost();

        assert_eq!(score.value(), 12);
    }

    #[test]
    fn ability_score_boost_above_18() {
        let mut score = AbilityScore::new(18);

        score.boost();

        assert_eq!(score.value(), 19);
    }

    #[test]
    fn ability_score_set_boost() {
        let mut set = AbilityScoreSet::new();

        set.boost(&hashset![
            AbilityScoreType::Strength,
            AbilityScoreType::Dexterity,
            AbilityScoreType::Constitution,
            AbilityScoreType::Charisma,
        ]);

        assert_eq!(set.get(AbilityScoreType::Strength).value(), 12);
        assert_eq!(set.get(AbilityScoreType::Dexterity).value(), 12);
        assert_eq!(set.get(AbilityScoreType::Constitution).value(), 12);
        assert_eq!(set.get(AbilityScoreType::Charisma).value(), 12);

        assert_eq!(set.get(AbilityScoreType::Intelligence).value(), 10);
        assert_eq!(set.get(AbilityScoreType::Wisdom).value(), 10);
    }
}
