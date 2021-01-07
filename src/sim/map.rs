use std::borrow::{Borrow};
use std::collections::HashSet;
use std::ptr;
use std::rc::Rc;
use std::rc::Weak;
use std::vec::Vec;

#[derive(PartialEq, Eq)]
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

fn in_region(countries_region: &Region, searching_for: &Region) -> bool {
    //TODO: throw error if searching_for is a Both?
    match countries_region {
        Region::Both(a, b) => in_region(a, &searching_for) || in_region(b, &searching_for),
        s => ptr::eq(s, searching_for),
    }
}

//TODO: the functalinality this provides I would prefer to be part of the HasBorders trait, but that was causing a lot of grief.
pub enum State {
    SuperpowerState(SuperpowerState),
    Country(Country),
}

pub trait HasBorders {
    //Can't use a hashset becuase ???
    fn get_neighbors(&self) -> &Vec<Weak<State>>;

    fn add_border(&mut self, new_neighbor: Weak<State>);

    fn neighbors_with(&self, to_check: &State) -> bool {
        self.get_neighbors().iter().any(|x| match x.upgrade() {
            None => false,
            Some(s) => match s.borrow() {
                State::SuperpowerState(s1) => match to_check {
                    State::SuperpowerState(s2) => s1.power == s2.power,
                    State::Country(_) => false,
                },
                State::Country(c1) => match to_check {
                    State::SuperpowerState(_) => false,
                    State::Country(c2) => std::ptr::eq(c1, c2),
                },
            },
        })
    }
}

//TODO: WHY AM I USING AN ENUM TO DO DOUBLE DISPATCH
impl State{
    pub fn get_name(&self)-> &str{
        match self{
            State::SuperpowerState(s) => match s.power {
                Superpower::USA => "USA",
                Superpower::USSR => "USSR",
            },
            State::Country(c) => c.get_name(),
        }
    }

}

impl HasBorders for State {
    fn get_neighbors(&self) -> &Vec<Weak<State>> {
        match self {
            State::SuperpowerState(s) => s.get_neighbors(),
            State::Country(c) => c.get_neighbors(),
        }
    }

    fn add_border(&mut self, new_neighbor: Weak<State>) {
        match self {
            State::SuperpowerState(s) => s.add_border(new_neighbor),
            State::Country(c) => c.add_border(new_neighbor),
        }
    }
}

pub struct Country {
    name: &'static str,
    stability: u8,
    region: Region,
    battleground: bool,
    us_influence: u8,
    ussr_influence: u8,
    bordering: Box<Vec<Weak<State>>>,
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
    pub fn in_region(&self, checking: &Region) -> bool {
        in_region(&self.region, checking)
    }

    pub fn get_name(&self) -> &str {
        self.name
    }
    pub fn get_stability(&self) -> u8 {
        self.stability
    }
    pub fn is_battleground(&self) -> bool {
        self.battleground
    }
}

impl HasBorders for Country {
    fn get_neighbors(&self) -> &Vec<Weak<State>> {
        return &*self.bordering;
    }

    fn add_border(&mut self, new_neighbor: Weak<State>) {
        self.bordering.push(new_neighbor);
    }
}

pub struct SuperpowerState {
    power: Superpower,
    bordering: Box<Vec<Weak<State>>>,
}

impl HasBorders for SuperpowerState {
    fn get_neighbors(&self) -> &Vec<Weak<State>> {
        return &*self.bordering;
    }

    fn add_border(&mut self, new_neighbor: Weak<State>) {
        self.bordering.push(new_neighbor);
    }
}

pub struct WorldMap {
    countries: Box<Vec<Rc<State>>>,
}

pub fn create_map() -> Box<WorldMap> {

    //TODO: wont all these states be on the stack? Should they be Boxed?
    //TODO: add the countries to `map.countries`
    let mut map = WorldMap {
        countries: Box::new(Vec::new()),
    };

    let mut usa:Rc<State> = Rc::new(State::SuperpowerState(SuperpowerState {
        power: Superpower::USA,
        bordering: Box::new(Vec::new()),
    }));
    map.countries.push(Rc::clone(&usa));

    //Central America
    let mut mexico:Rc<State> = create_country("Mexico",2,Region::CentralAmerica,true,&mut map);
    create_border(&mut usa,&mut mexico);

    let mut cuba:Rc<State> = create_country("Cuba", 3,Region::CentralAmerica,true,&mut map);
    create_border(&mut usa, &mut cuba);

    let mut haiti:Rc<State> = create_country("Haiti",1,Region::CentralAmerica,false,&mut map);
    create_border(&mut haiti, &mut cuba);

    let mut dom_rep:Rc<State> = create_country("Dominican Republic",1, Region::CentralAmerica,false,&mut map);
    create_border(&mut haiti, &mut dom_rep);

    let mut nicaragua:Rc<State> = create_country("Nicaragua",1,Region::CentralAmerica,false,&mut map);
    create_border(&mut nicaragua, &mut cuba);

    let mut guatemala:Rc<State> = create_country("Guatemala",1,Region::CentralAmerica,false,&mut map);
    create_border(&mut mexico, &mut guatemala);

    let mut el_salva:Rc<State> = create_country("El Salvador",1,Region::CentralAmerica,false,&mut map);
    create_border(&mut el_salva, &mut guatemala);

    let mut honduras:Rc<State> = create_country("Honduras",2, Region::CentralAmerica,false,&mut map);
    create_border(&mut honduras, &mut guatemala);
    create_border(&mut honduras, &mut el_salva);
    create_border(&mut honduras, &mut nicaragua);

    let mut costa_rica:Rc<State> = create_country("Costa Rica",3,Region::CentralAmerica,false,&mut map);
    create_border(&mut honduras, &mut costa_rica);
    create_border(&mut costa_rica, &mut nicaragua);

    let mut panama:Rc<State> = create_country("Panama",2,Region::CentralAmerica,true,&mut map);
    create_border(&mut panama, &mut costa_rica);


    //South America
    let mut colombia:Rc<State> = create_country("Colombia",1,Region::SouthAmerica,false,&mut map);
    create_border(&mut panama, &mut colombia);

    let mut ecuador:Rc<State> = create_country("Ecuador",2,Region::SouthAmerica,false,&mut map);
    create_border(&mut ecuador, &mut colombia);

    let mut venezuela:Rc<State> = create_country("Venezuela",2,Region::SouthAmerica,true,&mut map);
    create_border(&mut venezuela, &mut colombia);

    let mut brazil:Rc<State> = create_country("Brazil",2,Region::SouthAmerica,true,&mut map);
    create_border(&mut venezuela, &mut brazil);

    let mut uruguay:Rc<State> =  create_country("Uruguay",2,Region::SouthAmerica,false,&mut map);
    create_border(&mut uruguay, &mut brazil);

    let mut peru:Rc<State> = create_country("Peru",2,Region::SouthAmerica,false,&mut map);
    create_border(&mut ecuador, &mut peru);

    let mut bolivia:Rc<State> = create_country("Bolivia",2,Region::SouthAmerica,false,&mut map);
    create_border(&mut bolivia, &mut peru);

    let mut paraguay:Rc<State> = create_country("Paraguay",2,Region::SouthAmerica,false,&mut map);
    create_border(&mut bolivia, &mut paraguay);
    create_border(&mut uruguay, &mut paraguay);

    let mut chile:Rc<State> = create_country("Chile",3,Region::SouthAmerica,true,&mut map);
    create_border(&mut chile, &mut peru);

    let mut argentina:Rc<State> = create_country("Argentina",2,Region::SouthAmerica,true,&mut map);
    create_border(&mut chile, &mut argentina);
    create_border(&mut paraguay, &mut argentina);
    create_border(&mut uruguay, &mut argentina);


    //West Europe
    let mut canada:Rc<State> = create_country("Canada",4, Region::Both(Box::new(Region::WestEurope),Box::new(Region::Europe)),false,&mut map);
    create_border(&mut usa, &mut canada);

    let mut uk:Rc<State> = create_country("UK",5, Region::Both(Box::new(Region::WestEurope),Box::new(Region::Europe)),false,&mut map);
    create_border(&mut uk, &mut canada);

    let mut france:Rc<State> = create_country("France",3, Region::Both(Box::new(Region::WestEurope),Box::new(Region::Europe)),true,&mut map);
    create_border(&mut uk, &mut france);

    let mut spain:Rc<State> = create_country("Spain/Portugal",2, Region::Both(Box::new(Region::WestEurope),Box::new(Region::Europe)),false,&mut map);
    create_border(&mut france, &mut spain);

    let mut italy:Rc<State> = create_country("Italy",2, Region::Both(Box::new(Region::WestEurope),Box::new(Region::Europe)),true,&mut map);
    create_border(&mut france, &mut italy);
    create_border(&mut spain, &mut italy);

    let mut greece:Rc<State> = create_country("Greece",2, Region::Both(Box::new(Region::WestEurope),Box::new(Region::Europe)),false,&mut map);
    create_border(&mut italy, &mut greece);

    let mut turkey:Rc<State> = create_country("Turkey",2, Region::Both(Box::new(Region::WestEurope),Box::new(Region::Europe)),false,&mut map);
    create_border(&mut turkey, &mut greece);

    let mut benelux:Rc<State> = create_country("BeNeLux",3, Region::Both(Box::new(Region::WestEurope),Box::new(Region::Europe)),false,&mut map);
    create_border(&mut uk, &mut benelux);

    let mut norway:Rc<State> = create_country("Norway",4, Region::Both(Box::new(Region::WestEurope),Box::new(Region::Europe)),false,&mut map);
    create_border(&mut uk, &mut norway);

    let mut sweden:Rc<State> = create_country("Sweden",4, Region::Both(Box::new(Region::WestEurope),Box::new(Region::Europe)),false,&mut map);
    create_border(&mut sweden, &mut norway);

    let mut denmark:Rc<State> = create_country("Norway",3, Region::Both(Box::new(Region::WestEurope),Box::new(Region::Europe)),false,&mut map);
    create_border(&mut sweden, &mut denmark);

    let mut west_germany:Rc<State> = create_country("West Germany",4, Region::Both(Box::new(Region::WestEurope),Box::new(Region::Europe)),true,&mut map);
    create_border(&mut west_germany, &mut benelux);
    create_border(&mut west_germany, &mut france);
    create_border(&mut west_germany, &mut denmark);


    //"Central" Europe
    let mut austria = create_country("Austria",4,Region::Both(Box::new(Region::Europe),Box::new(Region::Both(Box::new(Region::WestEurope),Box::new(Region::EastEurope)))),false,&mut map);
    create_border(&mut austria,&mut west_germany);
    create_border(&mut austria, &mut italy);

    let mut finland = create_country("Finland",4,Region::Both(Box::new(Region::Europe),Box::new(Region::Both(Box::new(Region::WestEurope),Box::new(Region::EastEurope)))),false,&mut map);
    create_border(&mut finland,&mut sweden);


    //East Europe
    let mut ussr:Rc<State> = Rc::new(State::SuperpowerState(SuperpowerState {
        power: Superpower::USSR,
        bordering: Box::new(Vec::new()),
    }));
    map.countries.push(Rc::clone(&ussr));
    create_border(&mut ussr, &mut finland);

    let mut east_germany = create_country("East Germany",3,Region::Both(Box::new(Region::EastEurope),Box::new(Region::Europe)),true,&mut map);
    create_border(&mut east_germany, &mut west_germany);
    create_border(&mut east_germany, &mut austria);

    //TODO: rest of East Europe, Asia, ME, Africa
    return Box::new(map);
}
//TODO: *should* name be 'static?
fn create_country( name: &'static str, stability:u8, region:Region, battleground:bool, map :&mut WorldMap) -> Rc<State>{
    let c = Rc::new(State::Country(Country{
        name:  name,
        stability:stability,
        region:region,
        battleground:battleground,
        us_influence: 0,
        ussr_influence: 0,
        bordering: Box::new(vec![])
    }));
    map.countries.push(Rc::clone(&c));
    return c;
}
fn create_border(a:&mut Rc<State>, b: &mut Rc<State>) {
    match Rc::get_mut(a) {
        None => {panic!("Could not get mut of {} in to create a border", (*a).get_name())}
        Some(aa) => {aa.add_border(Rc::downgrade(b));}
    }
    match Rc::get_mut(b) {
        None => {panic!("Could not get mut of {} in to create a border", (*b).get_name())}
        Some(bb) => {bb.add_border(Rc::downgrade(a));}
    }
}

impl WorldMap {


    /*
    pub fn get_country(&self, name: &str) -> Rc<State> {
        match name {
            "usa" => match self.countries.iter().find(|x| match x {
                State::SuperpowerState(c) => c.power == Superpower::USA,
                State::Country(_) => false,
            }) {
                None => {
                    panic!("USA wasn't created.");
                }
                Some(usa) => Rc::clone(usa),
            },
            "ussr" => match self.countries.iter().find(|x| match x {
                State::SuperpowerState(c) => c.power == Superpower::USSR,
                State::Country(_) => false,
            }) {
                None => {
                    panic!("USSR wasn't created.");
                }
                Some(ussr) => Rc::clone(ussr),
            },
            _ => match self.countries.iter().find(|x| match x { //TODO: don't I need to deref this? If I delete the interior on the match and auto-fill the match then it matches on a `&_`
                State::SuperpowerState(_) => false,
                State::Country(c) => c.get_name() == name,
            }) {
                None => {
                    panic!("Country {} does not exist.", name)
                }
                Some(country) => Rc::clone(country),
            },
        }
    }*/
}
