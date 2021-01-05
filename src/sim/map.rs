pub enum Superpower {
    USA,
    USSR,
} //TODO this should probably be in a different module

#[derive(PartialEq, Eq)]
pub enum Region {
    //TODO: guarantee being in SEAsia means being in Asia, same for Europe
    Both(Box<Region>, Box<Region>),
    Europe,
    WestEurope,
    EastEurope,
    Asia,
    SEAsia,
    MiddleEast,
    Africa,
    SouthAmerica,
    CentralAmerica,
}

pub struct Country {
    name: &'static str,
    stability: u8,
    region: Region,
    battleground: bool,
    us_influence: u8,
    ussr_influence: u8,
    //TODO: bordering
}

fn in_region(countries_region: &Region, searching_for: &Region) -> bool {
    //TODO: throw error if searching_for is a Both?
    match countries_region {
        Region::Both(a, b) => in_region(a, &searching_for) || in_region(b, &searching_for),
        s => {
            if *s == *searching_for {
                true
            } else {
                false
            }
        }
    }
}

impl Country {
    /// Returns the Superpower the country is aligned to. If if is uncontrolled, it returns a None.
    pub fn alignment(self) -> Option<Superpower> {
        let diff: i8 = self.us_influence as i8 - self.ussr_influence as i8;
        if diff.abs() < self.stability as i8 {
            None
        } else {
            return if diff > 0 {
                Some(Superpower::USA)
            } else {
                Some(Superpower::USSR)
            };
        }
    }

    /// Modifies the countries influence from the target superpower by `change`. Returns the new influence level.
    pub fn mod_influence(&mut self, change: i8, power: Superpower) -> u8 {
        match power {
            Superpower::USA => {
                if change >= 0 {
                    self.us_influence += change as u8;
                } else {
                    let mag = change.abs() as u8;
                    if mag >= self.us_influence {
                        self.us_influence = 0;
                    } else {
                        self.us_influence -= mag;
                    }
                }
                return self.us_influence;
            }
            Superpower::USSR => {
                if change >= 0 {
                    self.ussr_influence += change as u8;
                } else {
                    let mag = change.abs() as u8;
                    if mag >= self.ussr_influence {
                        self.ussr_influence = 0;
                    } else {
                        self.ussr_influence -= mag;
                    }
                }
                return self.ussr_influence;
            }
        }
    }

    /// Checks that the country is in the specified Region. DO NOT PASS A BOTH()!
    pub fn in_region(self, checking: &Region) -> bool {
        in_region(&self.region, checking)
    }
}
