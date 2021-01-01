use super::map;
pub enum Card {
    //Early War
    AsiaScoring,
    EuropeScoring,
    MiddleEastScoring,
    DuckAndCover,
    FiveYearPlan,
    //ChinaCard //TODO how to represent CC
    SocialistGovernment,
    Fidel,
    VietnamRevolt,
    Blockade,
    KoreanWar,
    RomanianAbdication,
    ArabIsraeliWar,
    Comecon,
    Nasser,
    WarsawPact,
    DeGaulle,
    CapturedNaziScientist,
    TrumanDoctrine,
    Olympics,
    Nato,
    IndependentReds,
    MarshallPlan,
    IndoPakistanWar,
    Containment,
    USJapanDefencePact,
    SuezCrisis,
    EastEuropeUnrest,
    Decolonization,
    RedScarePurge,
    UNIntervention,
    DeStalinization,
    NukeBan,
    FormosanResolution,
    //optional
    Cambridge5,
    SpecialRelationship,
    Norad,

    //Mid War
    BrushWar,
    CentralAmericaScoring,
    SEAsiaScoring,
    ArmsRace,
    CubanMissileCrisis,
    NuclearSubs,
    Quagmire,
    SaltNegotiations,
    BearTrap,
    Summit,
    HowILearnedToStopWorrying,
    Junta,
    KitchenDebate,
    MissileEnvy,
    WeWillBuryYou,
    BrezhevDoctrine,
    PortugueseEmpireCrumbles,
    SouthAfricanUnrest,
    Allende,
    WillyBrandt,
    MuslimRevolution,
    ABMTreaty,
    CulturalRevolution,
    FlowerPower,
    U2Incident,
    Opec,
    LoneGunman,
    ColonialRearguards,
    PanamaCanalReturned,
    CampDavid,
    PuppetGovernment,
    GrainSalesToSoviets,
    JohnPaul2,
    LatinAmericanDeathSquads,
    Oas,
    NixonPlaysChinaCard,
    SadatExpelsSoviets,
    ShuttleDiplomacy,
    VoiceOfAmerica,
    LiberationTheology,
    UssuriRiverSkirmish,
    AskNotWhatYourCountry,
    AllianceForProgress,
    AfricaScoring,
    OneSmallStep,
    SouthAmericaScoring,

    //optional
    Che,
    ManInTehran,

    //Late War
    IranianHostageCrisis,
    IronLady,
    ReaganBombsLibya,
    StarWars,
    NorthSeaOil,
    Reformer,
    MarineBarracksBombed,
    SovietsShootKAL007,
    Glasnost,
    OrtegaElectedInNicaragua,
    Terrorism,
    IranContraScandal,
    Chernobyl,
    LatinAmericanDebtCrisis,
    TearDownThisWall,
    EvilEmpire,
    AldrichAmes,
    Pershing2,
    Wargames,
    Solidarity,
    IranIraqWar,

    //optional
    YuriAndSamantha,
    AwacsToSaudis,
}

pub fn alignment(card: Card) -> Option<map::Superpower> {
    match card {
        Card::AsiaScoring => None,
        Card::EuropeScoring => None,
        Card::MiddleEastScoring => None,
        Card::DuckAndCover => Some(map::Superpower::USA),
        Card::FiveYearPlan => Some(map::Superpower::USA),
        Card::SocialistGovernment => Some(map::Superpower::USSR),
        Card::Fidel => Some(map::Superpower::USSR),
        Card::VietnamRevolt => Some(map::Superpower::USSR),
        Card::Blockade => Some(map::Superpower::USSR),
        Card::KoreanWar => None,
        Card::RomanianAbdication => Some(map::Superpower::USSR),
        Card::ArabIsraeliWar => Some(map::Superpower::USSR),
        Card::Comecon => Some(map::Superpower::USSR),
        Card::Nasser => Some(map::Superpower::USSR),
        Card::WarsawPact => Some(map::Superpower::USSR),
        Card::DeGaulle => Some(map::Superpower::USSR),
        Card::CapturedNaziScientist => None,
        Card::TrumanDoctrine => Some(map::Superpower::USA),
        Card::Olympics => None,
        Card::Nato => Some(map::Superpower::USA),
        Card::IndependentReds => Some(map::Superpower::USA),
        Card::MarshallPlan => Some(map::Superpower::USA),
        Card::IndoPakistanWar => None,
        Card::Containment => Some(map::Superpower::USA),
        Card::USJapanDefencePact => Some(map::Superpower::USA),
        Card::SuezCrisis => Some(map::Superpower::USSR),
        Card::EastEuropeUnrest => Some(map::Superpower::USA),
        Card::Decolonization => Some(map::Superpower::USSR),
        Card::RedScarePurge => None,
        Card::UNIntervention => None,
        Card::DeStalinization => Some(map::Superpower::USSR),
        Card::NukeBan => None,
        Card::FormosanResolution => Some(map::Superpower::USA),
        Card::Cambridge5 => Some(map::Superpower::USSR),
        Card::SpecialRelationship => Some(map::Superpower::USA),
        Card::Norad => Some(map::Superpower::USA),
        Card::BrushWar => None,
        Card::CentralAmericaScoring => None,
        Card::SEAsiaScoring => None,
        Card::ArmsRace => None,
        Card::CubanMissileCrisis => None,
        Card::NuclearSubs => Some(map::Superpower::USA),
        Card::Quagmire => Some(map::Superpower::USSR),
        Card::SaltNegotiations => None,
        Card::BearTrap => Some(map::Superpower::USA),
        Card::Summit => None,
        Card::HowILearnedToStopWorrying => None,
        Card::Junta => None,
        Card::KitchenDebate => Some(map::Superpower::USA),
        Card::MissileEnvy => None,
        Card::WeWillBuryYou => Some(map::Superpower::USSR),
        Card::BrezhevDoctrine => Some(map::Superpower::USSR),
        Card::PortugueseEmpireCrumbles => Some(map::Superpower::USSR),
        Card::SouthAfricanUnrest => Some(map::Superpower::USSR),
        Card::Allende => Some(map::Superpower::USSR),
        Card::WillyBrandt => Some(map::Superpower::USSR),
        Card::MuslimRevolution => Some(map::Superpower::USSR),
        Card::ABMTreaty => None,
        Card::CulturalRevolution => Some(map::Superpower::USSR),
        Card::FlowerPower => Some(map::Superpower::USSR),
        Card::U2Incident => Some(map::Superpower::USSR),
        Card::Opec => Some(map::Superpower::USSR),
        Card::LoneGunman => Some(map::Superpower::USSR),
        Card::ColonialRearguards => Some(map::Superpower::USA),
        Card::PanamaCanalReturned => Some(map::Superpower::USA),
        Card::CampDavid => Some(map::Superpower::USA),
        Card::PuppetGovernment => Some(map::Superpower::USA),
        Card::GrainSalesToSoviets => Some(map::Superpower::USA),
        Card::JohnPaul2 => Some(map::Superpower::USA),
        Card::LatinAmericanDeathSquads => None,
        Card::Oas => Some(map::Superpower::USA),
        Card::NixonPlaysChinaCard => Some(map::Superpower::USA),
        Card::SadatExpelsSoviets => Some(map::Superpower::USA),
        Card::ShuttleDiplomacy => Some(map::Superpower::USA),
        Card::VoiceOfAmerica => Some(map::Superpower::USA),
        Card::LiberationTheology => Some(map::Superpower::USSR),
        Card::UssuriRiverSkirmish => Some(map::Superpower::USA),
        Card::AskNotWhatYourCountry => Some(map::Superpower::USA),
        Card::AllianceForProgress => Some(map::Superpower::USA),
        Card::AfricaScoring => None,
        Card::OneSmallStep => None,
        Card::SouthAmericaScoring => None,
        Card::Che => Some(map::Superpower::USSR),
        Card::ManInTehran => Some(map::Superpower::USA),
        Card::IranianHostageCrisis => Some(map::Superpower::USSR),
        Card::IronLady => Some(map::Superpower::USA),
        Card::ReaganBombsLibya => Some(map::Superpower::USA),
        Card::StarWars => Some(map::Superpower::USA),
        Card::NorthSeaOil => Some(map::Superpower::USA),
        Card::Reformer => Some(map::Superpower::USSR),
        Card::MarineBarracksBombed => Some(map::Superpower::USSR),
        Card::SovietsShootKAL007 => Some(map::Superpower::USA),
        Card::Glasnost => Some(map::Superpower::USSR),
        Card::OrtegaElectedInNicaragua => Some(map::Superpower::USSR),
        Card::Terrorism => None,
        Card::IranContraScandal => Some(map::Superpower::USSR),
        Card::Chernobyl => Some(map::Superpower::USA),
        Card::LatinAmericanDebtCrisis => Some(map::Superpower::USSR),
        Card::TearDownThisWall => Some(map::Superpower::USA),
        Card::EvilEmpire => Some(map::Superpower::USA),
        Card::AldrichAmes => Some(map::Superpower::USSR),
        Card::Pershing2 => Some(map::Superpower::USSR),
        Card::Wargames => None,
        Card::Solidarity => Some(map::Superpower::USA),
        Card::IranIraqWar => None,
        Card::YuriAndSamantha => Some(map::Superpower::USSR),
        Card::AwacsToSaudis => Some(map::Superpower::USA),
    }
}

pub fn value(card: Card) -> u8 {
    todo!()
}

//this card is discarded after effect is triggered
pub fn is_discarded(card: Card) -> bool {
    todo!()
}

//this is a scoring card
pub fn is_scoring(card: Card) -> bool {
    match card {
        Card::AsiaScoring => true,
        Card::EuropeScoring => true,
        Card::MiddleEastScoring => true,
        Card::SEAsiaScoring => true,
        Card::AfricaScoring => true,
        Card::CentralAmericaScoring => true,
        Card::SouthAmericaScoring => true,
        _ => false,
    }
}

//this is a "War" card (used for Flower Power)
pub fn is_war(card: Card) -> bool {
    match card {
        Card::KoreanWar => true,
        Card::ArabIsraeliWar => true,
        Card::IndoPakistanWar => true,
        Card::BrushWar => true,
        Card::IranIraqWar => true,
        _ => false,
    }
}

pub fn populate_deck_early_war(deck: &mut Vec<Card>) {
    deck.push(Card::AsiaScoring);
    deck.push(Card::EuropeScoring);
    deck.push(Card::MiddleEastScoring);
    deck.push(Card::DuckAndCover);
    deck.push(Card::FiveYearPlan);
    deck.push(Card::SocialistGovernment);
    deck.push(Card::Fidel);
    deck.push(Card::VietnamRevolt);
    deck.push(Card::Blockade);
    deck.push(Card::KoreanWar);
    deck.push(Card::RomanianAbdication);
    deck.push(Card::ArabIsraeliWar);
    deck.push(Card::Comecon);
    deck.push(Card::Nasser);
    deck.push(Card::WarsawPact);
    deck.push(Card::DeGaulle);
    deck.push(Card::CapturedNaziScientist);
    deck.push(Card::TrumanDoctrine);
    deck.push(Card::Olympics);
    deck.push(Card::Nato);
    deck.push(Card::IndependentReds);
    deck.push(Card::MarshallPlan);
    deck.push(Card::IndoPakistanWar);
    deck.push(Card::Containment);
    deck.push(Card::USJapanDefencePact);
    deck.push(Card::SuezCrisis);
    deck.push(Card::EastEuropeUnrest);
    deck.push(Card::Decolonization);
    deck.push(Card::RedScarePurge);
    deck.push(Card::UNIntervention);
    deck.push(Card::DeStalinization);
    deck.push(Card::NukeBan);
    deck.push(Card::FormosanResolution);
    //optional
    deck.push(Card::Cambridge5);
    deck.push(Card::SpecialRelationship);
    deck.push(Card::Norad);
}
