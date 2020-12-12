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

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct AbilityBoostFree {}

impl AbilityBoostFree {
    pub fn choose(&self, boost: AbilityScoreType) -> AbilityScoreType {
        boost
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct AbilityBoostRestricted {
    options: Vec<AbilityScoreType>,
}

impl AbilityBoostRestricted {
    pub fn choose(&self, boost: AbilityScoreType) -> Result<AbilityScoreType, String> {
        if self.options.contains(&boost) {
            Ok(boost)
        } else {
            Err(format!(
                "Boost to {:?} is not a valid choice! Options: {:?}",
                boost, self.options
            ))
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum AbilityBoostChoice {
    Free(AbilityBoostFree),
    Restricted(AbilityBoostRestricted),
    Predetermined(AbilityScoreType),
}

impl AbilityBoostChoice {
    pub fn free() -> AbilityBoostChoice {
        AbilityBoostChoice::Free(AbilityBoostFree {})
    }

    pub fn restricted(options: HashSet<AbilityScoreType>) -> AbilityBoostChoice {
        AbilityBoostChoice::Restricted(AbilityBoostRestricted {
            options: options.into_iter().collect(),
        })
    }

    pub fn predetermined(boost: AbilityScoreType) -> AbilityBoostChoice {
        AbilityBoostChoice::Predetermined(boost)
    }
}

pub trait AbilityBoostChoiceSet {
    fn apply_choices(
        &self,
        choices: &Vec<AbilityScoreType>,
    ) -> Result<HashSet<AbilityScoreType>, String>;
}
impl AbilityBoostChoiceSet for Vec<AbilityBoostChoice> {
    fn apply_choices(
        &self,
        choices: &Vec<AbilityScoreType>,
    ) -> Result<HashSet<AbilityScoreType>, String> {
        let mut choices_iter = choices.iter();
        let mut result = HashSet::with_capacity(self.len());
        for boost in self.iter() {
            let choice = match boost {
                AbilityBoostChoice::Predetermined(boost) => *boost,
                AbilityBoostChoice::Free(_) | AbilityBoostChoice::Restricted(_) => {
                    if let Some(choice) = choices_iter.next() {
                        match boost {
                            AbilityBoostChoice::Free(_) => *choice,
                            AbilityBoostChoice::Restricted(options) => options.choose(*choice)?,
                            _ => panic!("should never happen"),
                        }
                    } else {
                        return Err("Too few boost choices!".to_string());
                    }
                }
            };
            if result.contains(&choice) {
                return Err(format!(
                    "Duplicte boost choice! {:?} was already chosen!",
                    choice
                ));
            } else {
                result.insert(choice);
            }
        }

        if let Some(_) = choices_iter.next() {
            Err("Too many boost choices!".to_string())
        } else {
            Ok(result)
        }
    }
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

#[cfg(test)]
mod ability_boost_choice_tests {
    use super::*;

    #[test]
    fn free() {
        let choice = AbilityBoostChoice::free();

        let boost = match choice {
            AbilityBoostChoice::Free(boost) => boost.choose(AbilityScoreType::Strength),
            _ => panic!(""),
        };

        assert_eq!(boost, AbilityScoreType::Strength);
    }

    #[test]
    fn predetermined() {
        let choice = AbilityBoostChoice::Predetermined(AbilityScoreType::Strength);

        let boost = match choice {
            AbilityBoostChoice::Predetermined(boost) => boost,
            _ => panic!(""),
        };

        assert_eq!(boost, AbilityScoreType::Strength);
    }

    #[test]
    fn restricted() {
        let choice = AbilityBoostChoice::restricted(hashset![
            AbilityScoreType::Strength,
            AbilityScoreType::Dexterity,
            AbilityScoreType::Constitution
        ]);

        let boost = match choice {
            AbilityBoostChoice::Restricted(boost) => {
                boost.choose(AbilityScoreType::Dexterity).unwrap()
            }
            _ => panic!(""),
        };

        assert_eq!(boost, AbilityScoreType::Dexterity);
    }

    #[test]
    #[should_panic]
    fn restricted_incorrect_choice() {
        let choice = AbilityBoostChoice::restricted(hashset![
            AbilityScoreType::Strength,
            AbilityScoreType::Dexterity,
            AbilityScoreType::Constitution
        ]);

        match choice {
            AbilityBoostChoice::Restricted(boost) => {
                boost.choose(AbilityScoreType::Intelligence).unwrap()
            }
            _ => panic!(""),
        };
    }

    #[test]
    fn ability_boost_set_works() {
        let set = vec![
            AbilityBoostChoice::predetermined(AbilityScoreType::Strength),
            AbilityBoostChoice::predetermined(AbilityScoreType::Constitution),
            AbilityBoostChoice::free(),
        ];

        let boosts = set
            .apply_choices(&vec![AbilityScoreType::Dexterity])
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
    fn ability_boost_set_too_many_boost_choices() {
        let set = vec![
            AbilityBoostChoice::predetermined(AbilityScoreType::Strength),
            AbilityBoostChoice::predetermined(AbilityScoreType::Constitution),
            AbilityBoostChoice::free(),
        ];

        set.apply_choices(&vec![
            AbilityScoreType::Dexterity,
            AbilityScoreType::Strength,
        ])
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn ability_boost_set_too_few_boost_choices() {
        let set = vec![
            AbilityBoostChoice::predetermined(AbilityScoreType::Strength),
            AbilityBoostChoice::predetermined(AbilityScoreType::Constitution),
            AbilityBoostChoice::free(),
        ];

        set.apply_choices(&vec![]).unwrap();
    }

    #[test]
    #[should_panic]
    fn ability_boost_set_incorrect_duplicate_free_boost() {
        let set = vec![
            AbilityBoostChoice::predetermined(AbilityScoreType::Strength),
            AbilityBoostChoice::predetermined(AbilityScoreType::Constitution),
            AbilityBoostChoice::free(),
        ];

        set.apply_choices(&vec![AbilityScoreType::Strength])
            .unwrap();
    }
}
