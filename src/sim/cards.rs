pub mod cards {
    pub enum Card{
        //Early War
        //AsiaScoring,
        //EuropeScoring,
        //MiddleEastScoring,
        //SEAsiaScoring,
        //AfricaScoring,
        //SouthAmericaScoring,
        //CentralAmericaScoring
        //TODO: which of these is preferable? above might be better for networking, below is probably better for cards that specify "scoring"
        Scoring(map::Region),
        DuckAndCover,
        FiveYearPlan,
        //ChinaCard //TODO how to represent CC
        SocialistGovernment,
        Fidel,
        VietnamRevolt,
        Blockade,
        //KoreanWar
        //ArabIsraelWar
        //IndoPakistanWar
        //BrushWar
        //IranIraqWar
        //TODO: which of these is preferable? above might be better for networking, below works better with Flower power
        War(War),
        RomanianAbdication,
        Comecon,
        Nasser,
        WarsawPact,
        DeGaulle,
        CapturedNaziScientist,
        TrumanDoctrine,
        Olympics,
        Nato,
        IndependentReds,
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
        //AfricaScoring,
        OneSmallStep,
        //SouthAmericaScoring,

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
        //IranIraqWar

        //optional
        YuriAndSamantha,
        AwacsToSaudis


    }


    pub enum War{
        Korea,
        ArabIsrael,
        IndoPakistan,
        Brush,
        IranIraq,
    }


    pub fn isDiscarded(card:Card)-> bool{

    }

    pub fn alignment(card:Card)-> Maybe<map::Superpower>{

    }



}