use std::cmp::min;

pub struct Health {
    max: u32,
    current: u32,
    temp: u32,
}

impl Health {
    pub fn new(max: u32) -> Health {
        Health {
            max,
            current: max,
            temp: 0,
        }
    }

    pub fn max(&self) -> u32 {
        self.max
    }
    pub fn increase_max(&mut self, value: u32) -> u32 {
        self.max += value;
        self.max
    }

    pub fn current(&self) -> u32 {
        self.current
    }

    pub fn temp(&self) -> u32 {
        self.temp
    }
    /// Sets a new amount of temporary HP. Does nothing if the character already has more temporary HP than the new value.
    pub fn set_temp(&mut self, value: u32) {
        if (self.temp < value) {
            self.temp = value;
        }
    }

    /// Heals by the specified amount, up to the maximum HP
    pub fn heal(&mut self, value: u32) -> u32 {
        self.current = min(self.current + value, self.max);
        self.current
    }
    /// Damages the character. Damage is first taken from temporary HP.
    pub fn damage(&mut self, value: u32) -> u32 {
        // handle temp hp
        if self.temp >= value {
            self.temp -= value;
            return self.current;
        } else {
            let value = value - self.temp;
            self.temp = 0;

            if self.current <= value {
                self.current = 0;
            } else {
                self.current -= value;
            }
            self.current
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_produces_correct_state() {
        let hp = Health::new(10);

        assert_eq!(hp.max(), 10);
        assert_eq!(hp.current(), 10);
        assert_eq!(hp.temp(), 0);
    }

    #[test]
    fn heal_caps_at_max() {
        let mut hp = Health {
            max: 10,
            current: 5,
            temp: 0,
        };

        hp.heal(10);
        assert_eq!(hp.current(), 10);
    }

    #[test]
    fn damage_caps_at_0() {
        let mut hp = Health::new(10);

        hp.damage(20);

        assert_eq!(hp.current(), 0);
    }

    #[test]
    fn damage_takes_from_temp_hp_first() {
        let mut hp = Health::new(10);
        hp.set_temp(5);

        hp.damage(10);

        assert_eq!(hp.current(), 5);
    }
}
