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

pub trait HasBorders {
    //Can't use a hashset becuase ???
    fn get_neighbors(&self) -> &Vec<Weak<dyn HasBorders>>;

    fn add_border(&mut self, new_neighbor: Weak<dyn HasBorders>);

    fn neighbors_with(&self, to_check: &dyn HasBorders) -> bool {
        self.get_neighbors().iter().any(|x| match x.upgrade() {
            None => false,
            Some(s) => std::ptr::eq(&*s, to_check),
        })
    }

    fn get_name(&self) -> & str;
}

pub struct Country {
    name: &'static str,
    stability: u8,
    region: Region,
    battleground: bool,
    us_influence: u8,
    ussr_influence: u8,
    bordering: Vec<Weak<dyn HasBorders>>,
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

    pub fn get_stability(&self) -> u8 {
        self.stability
    }
    pub fn is_battleground(&self) -> bool {
        self.battleground
    }
}

impl HasBorders for Country {
    fn get_neighbors(&self) -> &Vec<Weak<dyn HasBorders>> {
        return &self.bordering;
    }

    fn add_border(&mut self, new_neighbor: Weak<dyn HasBorders>) {
        self.bordering.push(new_neighbor);
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

pub struct SuperpowerState {
    power: Superpower,
    bordering: Vec<Weak<dyn HasBorders>>,
}

impl HasBorders for SuperpowerState {
    fn get_neighbors(&self) -> &Vec<Weak<dyn HasBorders>> {
        return &self.bordering;
    }

    fn add_border(&mut self, new_neighbor: Weak<dyn HasBorders>) {
        self.bordering.push(new_neighbor);
    }

    fn get_name(&self) -> &str {
        match self.power {
            Superpower::USA => {"USA"}
            Superpower::USSR => {"USSR"}
        }
    }
}

pub struct WorldMap {
    countries: Vec<Box<Rc<dyn HasBorders>>>,
}

pub fn create_map() -> Box<WorldMap> {

    let mut map = WorldMap {
        countries: Vec::new(),
    };

    let mut usa:Rc<dyn HasBorders> = Rc::new(SuperpowerState {
        power: Superpower::USA,
        bordering: Vec::new(),
    });
    map.countries.push(Box::new(Rc::clone(&usa)));

    //Central America
    let mut mexico = create_country("Mexico",2,Region::CentralAmerica,true,&mut map);
    create_border(&mut usa,&mut mexico);

    let mut cuba = create_country("Cuba", 3,Region::CentralAmerica,true,&mut map);
    create_border(&mut usa, &mut cuba);

    let mut haiti = create_country("Haiti",1,Region::CentralAmerica,false,&mut map);
    create_border(&mut haiti, &mut cuba);

    let mut dom_rep = create_country("Dominican Republic",1, Region::CentralAmerica,false,&mut map);
    create_border(&mut haiti, &mut dom_rep);

    let mut nicaragua = create_country("Nicaragua",1,Region::CentralAmerica,false,&mut map);
    create_border(&mut nicaragua, &mut cuba);

    let mut guatemala = create_country("Guatemala",1,Region::CentralAmerica,false,&mut map);
    create_border(&mut mexico, &mut guatemala);

    let mut el_salva = create_country("El Salvador",1,Region::CentralAmerica,false,&mut map);
    create_border(&mut el_salva, &mut guatemala);

    let mut honduras = create_country("Honduras",2, Region::CentralAmerica,false,&mut map);
    create_border(&mut honduras, &mut guatemala);
    create_border(&mut honduras, &mut el_salva);
    create_border(&mut honduras, &mut nicaragua);

    let mut costa_rica = create_country("Costa Rica",3,Region::CentralAmerica,false,&mut map);
    create_border(&mut honduras, &mut costa_rica);
    create_border(&mut costa_rica, &mut nicaragua);

    let mut panama = create_country("Panama",2,Region::CentralAmerica,true,&mut map);
    create_border(&mut panama, &mut costa_rica);


    //South America
    let mut colombia = create_country("Colombia",1,Region::SouthAmerica,false,&mut map);
    create_border(&mut panama, &mut colombia);

    let mut ecuador = create_country("Ecuador",2,Region::SouthAmerica,false,&mut map);
    create_border(&mut ecuador, &mut colombia);

    let mut venezuela = create_country("Venezuela",2,Region::SouthAmerica,true,&mut map);
    create_border(&mut venezuela, &mut colombia);

    let mut brazil = create_country("Brazil",2,Region::SouthAmerica,true,&mut map);
    create_border(&mut venezuela, &mut brazil);

    let mut uruguay =  create_country("Uruguay",2,Region::SouthAmerica,false,&mut map);
    create_border(&mut uruguay, &mut brazil);

    let mut peru = create_country("Peru",2,Region::SouthAmerica,false,&mut map);
    create_border(&mut ecuador, &mut peru);

    let mut bolivia = create_country("Bolivia",2,Region::SouthAmerica,false,&mut map);
    create_border(&mut bolivia, &mut peru);

    let mut paraguay = create_country("Paraguay",2,Region::SouthAmerica,false,&mut map);
    create_border(&mut bolivia, &mut paraguay);
    create_border(&mut uruguay, &mut paraguay);

    let mut chile = create_country("Chile",3,Region::SouthAmerica,true,&mut map);
    create_border(&mut chile, &mut peru);

    let mut argentina = create_country("Argentina",2,Region::SouthAmerica,true,&mut map);
    create_border(&mut chile, &mut argentina);
    create_border(&mut paraguay, &mut argentina);
    create_border(&mut uruguay, &mut argentina);


    //West Europe
    let mut canada = create_country("Canada",4, Region::Both(Box::new(Region::WestEurope),Box::new(Region::Europe)),false,&mut map);
    create_border(&mut usa, &mut canada);

    let mut uk= create_country("UK",5, Region::Both(Box::new(Region::WestEurope),Box::new(Region::Europe)),false,&mut map);
    create_border(&mut uk, &mut canada);

    let mut france = create_country("France",3, Region::Both(Box::new(Region::WestEurope),Box::new(Region::Europe)),true,&mut map);
    create_border(&mut uk, &mut france);

    let mut spain = create_country("Spain/Portugal",2, Region::Both(Box::new(Region::WestEurope),Box::new(Region::Europe)),false,&mut map);
    create_border(&mut france, &mut spain);

    let mut italy= create_country("Italy",2, Region::Both(Box::new(Region::WestEurope),Box::new(Region::Europe)),true,&mut map);
    create_border(&mut france, &mut italy);
    create_border(&mut spain, &mut italy);

    let mut greece= create_country("Greece",2, Region::Both(Box::new(Region::WestEurope),Box::new(Region::Europe)),false,&mut map);
    create_border(&mut italy, &mut greece);

    let mut turkey= create_country("Turkey",2, Region::Both(Box::new(Region::WestEurope),Box::new(Region::Europe)),false,&mut map);
    create_border(&mut turkey, &mut greece);

    let mut benelux = create_country("BeNeLux",3, Region::Both(Box::new(Region::WestEurope),Box::new(Region::Europe)),false,&mut map);
    create_border(&mut uk, &mut benelux);

    let mut norway = create_country("Norway",4, Region::Both(Box::new(Region::WestEurope),Box::new(Region::Europe)),false,&mut map);
    create_border(&mut uk, &mut norway);

    let mut sweden = create_country("Sweden",4, Region::Both(Box::new(Region::WestEurope),Box::new(Region::Europe)),false,&mut map);
    create_border(&mut sweden, &mut norway);

    let mut denmark = create_country("Norway",3, Region::Both(Box::new(Region::WestEurope),Box::new(Region::Europe)),false,&mut map);
    create_border(&mut sweden, &mut denmark);

    let mut west_germany = create_country("West Germany",4, Region::Both(Box::new(Region::WestEurope),Box::new(Region::Europe)),true,&mut map);
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
    let mut ussr:Rc<dyn HasBorders> = Rc::new(SuperpowerState {
        power: Superpower::USSR,
        bordering: Vec::new(),
    });
    map.countries.push(Box::new(Rc::clone(&ussr)));
    create_border(&mut ussr, &mut finland);

    let mut east_germany = create_country("East Germany",3,Region::Both(Box::new(Region::EastEurope),Box::new(Region::Europe)),true,&mut map);
    create_border(&mut east_germany, &mut west_germany);
    create_border(&mut east_germany, &mut austria);

    let mut poland = create_country("Poland",3,Region::Both(Box::new(Region::EastEurope),Box::new(Region::Europe)),true,&mut map);
    create_border(&mut east_germany, &mut poland);
    create_border(&mut poland, &mut ussr);

    let mut czechoslovakia = create_country("Czechoslovakia",3,Region::Both(Box::new(Region::EastEurope),Box::new(Region::Europe)), false, &mut map);
    create_border(&mut czechoslovakia,&mut poland);
    create_border(&mut czechoslovakia, &mut east_germany);

    let mut hungary = create_country("Hungary",3,Region::Both(Box::new(Region::EastEurope),Box::new(Region::Europe)), false, &mut map);
    create_border(&mut czechoslovakia,&mut hungary);
    create_border(&mut austria, &mut hungary);

    let mut yugoslavia = create_country("Yugoslavia", 3, Region::Both(Box::new(Region::EastEurope), Box::new(Region::Europe)), false, &mut map);
    create_border(&mut yugoslavia, &mut hungary);
    create_border(&mut yugoslavia, &mut italy);

    let mut romania = create_country("Romania",3,Region::Both(Box::new(Region::EastEurope),Box::new(Region::Europe)), false, &mut map);
    create_border(&mut romania,&mut hungary);
    create_border(&mut romania, &mut greece);
    create_border(&mut romania, &mut turkey);
    create_border(&mut romania, &mut ussr);

    let mut bulgaria = create_country("Bulgaria",3,Region::Both(Box::new(Region::EastEurope),Box::new(Region::Europe)), false, &mut map);
    create_border(&mut bulgaria, &mut greece);
    create_border(&mut bulgaria, &mut turkey);

    //Africa

    let mut morocco = create_country("Morocco", 3,Region::Africa, false,&mut map );
    create_border(&mut morocco, &mut spain);

    let mut w_africa = create_country("West African States", 2,Region::Africa, false, &mut map);
    create_border(&mut w_africa,&mut morocco);

    let mut ivory_coast = create_country("Ivory Coast", 2,Region::Africa, false, &mut map);
    create_border(&mut w_africa,&mut ivory_coast);

    let mut algeria = create_country("Algeria", 2,Region::Africa,true, &mut map );
    create_border(&mut morocco, &mut algeria);
    create_border(&mut france, &mut algeria);

    let mut saharan_states = create_country("Saharan States",1,Region::Africa,false,&mut map);
    create_border(&mut algeria, &mut saharan_states);

    let mut nigeria = create_country("Nigeria",1, Region::Africa, true, &mut map);
    create_border(&mut saharan_states, &mut nigeria);
    create_border(&mut ivory_coast, &mut nigeria);

    let mut cameroon = create_country("Cameroon",1, Region::Africa, false, &mut map);
    create_border(&mut nigeria, &mut cameroon);

    let mut tunisia = create_country("Tunisia", 2, Region::Africa, false, &mut map);
    create_border(&mut tunisia, &mut algeria);

    let mut zaire = create_country("Zaire",1,Region::Africa, true, &mut map);
    create_border(&mut zaire, &mut cameroon);

    let mut angola = create_country("Angola",1, Region::Africa, true, &mut map);
    create_border(&mut zaire, &mut angola);

    let mut south_africa = create_country("South Africa",3, Region::Africa, true, &mut map);
    create_border(&mut south_africa, &mut angola);

    let mut botswana = create_country("Botswana",2, Region::Africa, false, &mut map);
    create_border(&mut botswana, &mut angola);
    create_border(&mut botswana, &mut south_africa);

    let mut zimbabwe = create_country("Zimbabwe",1,Region::Africa,false, &mut map);
    create_border(&mut zaire, &mut zimbabwe);
    create_border(&mut botswana, &mut zimbabwe);

    let mut s_e_africa = create_country("South East African States",1,Region::Africa,false, &mut map);
    create_border(&mut s_e_africa, &mut zimbabwe);

    let mut kenya = create_country("Kenya",2,Region::Africa,false, &mut map);
    create_border(&mut s_e_africa, &mut kenya);

    let mut somalia = create_country("Somalia",2,Region::Africa,false, &mut map);
    create_border(&mut somalia, &mut kenya);

    let mut ethiopia = create_country("Ethiopia",1,Region::Africa,false, &mut map);
    create_border(&mut somalia, &mut ethiopia);

    let mut sudan = create_country("Sudan",1,Region::Africa,false, &mut map);
    create_border(&mut sudan, &mut ethiopia);

    //Middle East
    let mut libya = create_country("Libya",2,Region::MiddleEast,true,&mut map);
    create_border(&mut libya, &mut tunisia);

    let mut egypt = create_country("Egypt",2,Region::MiddleEast,true,&mut map);
    create_border(&mut libya, &mut egypt);
    create_border(&mut sudan, &mut egypt);

    let mut israel = create_country("Israel",4,Region::MiddleEast,true,&mut map);
    create_border(&mut egypt, &mut israel);

    let mut syria = create_country("Syria",2,Region::MiddleEast,false,&mut map);
    create_border(&mut israel, &mut syria);
    create_border(&mut turkey, &mut syria);

    let mut lebanon = create_country("Lebanon",1,Region::MiddleEast,false,&mut map);
    create_border(&mut syria, &mut lebanon);
    create_border(&mut israel, &mut lebanon);

    let mut jordan = create_country("Jordan",2,Region::MiddleEast,false,&mut map);
    create_border(&mut israel, &mut jordan);
    create_border(&mut lebanon, &mut jordan);

    let mut saudi = create_country("Saudi Arabia",3,Region::MiddleEast,true,&mut map);
    create_border(&mut jordan, &mut saudi);

    let mut gulf_states = create_country("Gulf States",3,Region::MiddleEast,false,&mut map);
    create_border(&mut saudi, &mut gulf_states);

    let mut iraq = create_country("Iraq",3,Region::MiddleEast,true,&mut map);
    create_border(&mut gulf_states, &mut iraq);
    create_border(&mut saudi, &mut iraq);

    let mut iran = create_country("Iran",2,Region::MiddleEast,true,&mut map);
    create_border(&mut iraq, &mut iran);

    //Asia
    let mut afghanistan = create_country("Afghanistan",2, Region::Asia,false,&mut map);
    create_border(&mut iran, &mut afghanistan);

    let mut pakistan = create_country("Pakistan",2, Region::Asia,true,&mut map);
    create_border(&mut iran, &mut pakistan);
    create_border(&mut afghanistan, &mut pakistan);

    let mut india = create_country("India", 3, Region::Asia,true,&mut map);
    create_border(&mut pakistan, &mut india);

    let mut n_korea = create_country("North Korea",3,Region::Asia, true,&mut map);
    create_border(&mut ussr, &mut n_korea);

    let mut s_korea = create_country("South Korea",3,Region::Asia, true,&mut map);
    create_border(&mut s_korea, &mut n_korea);

    let mut japan = create_country("Japan",4,Region::Asia, true,&mut map);
    create_border(&mut s_korea, &mut japan);
    create_border(&mut usa, &mut japan);

    let mut taiwan = create_country("Taiwan",3,Region::Asia, false,&mut map);
    create_border(&mut s_korea, &mut taiwan);
    create_border(&mut taiwan, &mut japan);

    let mut australia = create_country("Australia",4,Region::Asia, false,&mut map);

    //SE Asia
    let mut burma  = create_country("Burma", 2, Region::Both(Box::new(Region::SEAsia),Box::new(Region::Asia)),false ,&mut map);
    create_border(&mut india, &mut burma);

    let mut laos  = create_country("Laos/Cambodia", 1, Region::Both(Box::new(Region::SEAsia),Box::new(Region::Asia)),false ,&mut map);
    create_border(&mut laos, &mut burma);

    let mut vietnam  = create_country("Vietnam", 1, Region::Both(Box::new(Region::SEAsia),Box::new(Region::Asia)),false ,&mut map);
    create_border(&mut laos, &mut vietnam);

    let mut thailand  = create_country("Thailand", 2, Region::Both(Box::new(Region::SEAsia),Box::new(Region::Asia)),true ,&mut map);
    create_border(&mut laos, &mut thailand);
    create_border(&mut vietnam, &mut thailand);

    let mut malaysia  = create_country("Malaysia", 2, Region::Both(Box::new(Region::SEAsia),Box::new(Region::Asia)),false ,&mut map);
    create_border(&mut malaysia, &mut thailand);
    create_border(&mut malaysia, &mut australia);

    let mut indonesia  = create_country("Indonesia", 1, Region::Both(Box::new(Region::SEAsia),Box::new(Region::Asia)),false ,&mut map);
    create_border(&mut malaysia, &mut indonesia);

    let mut philippines  = create_country("Philippines", 2, Region::Both(Box::new(Region::SEAsia),Box::new(Region::Asia)),false ,&mut map);
    create_border(&mut indonesia, &mut philippines);
    create_border(&mut japan, &mut philippines);


    return Box::new(map);
}
//TODO: *should* name be 'static?
//TODO: somehow make it so we dont have to pass map
fn create_country( name: &'static str, stability:u8, region:Region, battleground:bool, map :&mut WorldMap) -> Rc<dyn HasBorders>{
    let c:Rc<dyn HasBorders> = Rc::new(Country{
        name:  name,
        stability:stability,
        region:region,
        battleground:battleground,
        us_influence: 0,
        ussr_influence: 0,
        bordering: vec![],
    });
    map.countries.push(Box::new(Rc::clone(&c)));
    return c;
}
fn create_border(a:&mut Rc<dyn HasBorders>, b: &mut Rc<dyn HasBorders>) {
    match Rc::get_mut(a) {
        None => {panic!("Could not get mut of {} in to create a border", (*a).get_name())}
        Some(aa) => {aa.add_border(Rc::downgrade(b));}
    }
    match Rc::get_mut(b) {
        None => {panic!("Could not get mut of {} in to create a border", (*b).get_name())}
        Some(bb) => {bb.add_border(Rc::downgrade(a));}
    }
}
