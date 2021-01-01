pub mod map{
    pub enum Superpower{
        USA,
        USSR,
    }//TODO this should probably be in a different module



    pub enum Region{
        //TODO: guarantee being in SEAsia means being in Asia, same for Europe
        Both (Region, Region),
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

    pub struct Country{
        name: &'static str,
        stability: u8,
        region: Region,
        battleground: bool
        //TODO: bordering relationships
    }

    pub fn alignment(country:&Country) -> Option<Superpower> {

    }






}