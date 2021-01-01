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

fn in_region(countries_region:&Region, searching_for:Region)->bool{
    if *countries_region == searching_for {
        return true
    }

    match searching_for {
        Region::Both(a, b) => in_region(countries_region,*a) || in_region(countries_region, *b),
        _ => false,
    }
}


impl Country {
    pub fn alignment(self) -> Option<Superpower> {
        let diff: i8 = self.us_influence as i8 - self.ussr_influence as i8;
        if diff.abs() < self.stability as i8{
            None
        } else {
            return if diff > 0 {
                Some(Superpower::USA)
            } else {
                Some(Superpower::USSR)
            };
        }
    }

    pub fn mod_influence(&mut self, change:i8, power: Superpower){
        match power {
            Superpower::USA => {
                if change>=0 { self.us_influence += change as u8;}
                else{
                    let mag = change.abs() as u8;
                    if mag >= self.us_influence {
                        self.us_influence =0;
                    }else{
                        self.us_influence -= mag;
                    }
                }
            }
            Superpower::USSR => {
                if change>=0 { self.ussr_influence += change as u8;}
                else{
                    let mag = change.abs() as u8;
                    if mag >= self.ussr_influence {
                        self.ussr_influence =0;
                    }else{
                        self.ussr_influence -= mag;
                    }
                }
            }
        }
    }

}
