use crate::sim::map::Superpower;

pub mod cards;
pub mod map;

pub struct Player{
    //TODO: connection details
    superpower:Superpower,
    hand: Vec<cards::Card>,
}
//TODO we probably want to keep hand outside of the sim? After all the client shouldn't know what the other has, or their connection details




pub mod game {

    use super::cards;
    use super::map;
    use crate::sim::map::{Superpower, WorldMap, Country};
    use crate::sim::cards::Card;
    use std::rc::Weak;

    //TODO: store effects in place (.ie Quagmire, Camp David...)
    pub struct Game {
        world: Box<WorldMap>,
        deck: Vec<cards::Card>,
        discards: Vec<cards::Card>,
        phasing_player: Superpower,
        action_round: u8, //TODO: should these be enums?
        turn: u8,
        defcon: i8,
    }

    //TODO: don't we want to return a reference, not copy the data out? Box<> it?
    pub fn set_up_game() -> Game {
        //set up the game board/deck
        let mut state: Game = Game {
            world: map::create_map(),
            deck: Vec::new(),
            discards: Vec::new(),
            phasing_player: Superpower::USSR,
            action_round: 0,
            turn: 1,
            defcon: 5,
        };
        cards::populate_deck_early_war(&mut state.deck);
        //TODO: initial influence setup
        return state;
    }

    pub enum Action{
        Operations(Card,Superpower,OpsAction),
        SpaceRace(Card,Superpower),
        Event(Card,Superpower),
    }

    pub enum OpsAction{
        PlaceInfluence(Box<Vec<Weak<Country>>>),
        Coup(Weak<Country>),
        Realignment(Box<Vec<Weak<Country>>>), //TODO: but realignments are resolved one by one!
    }

    pub enum ActionError{
        NotImplemented,

        TooManyActions,
        TooLittleActions,

        OddInfluenceInControlledCountry,
        NoNeighboringInfluence,

        CannotCoupInRegion,
        CannotCoupRInCountry,

        CannotRealignInRegion,
        CannotRealignInCountry,



        InvalidForEvent, //i.e Italy targeted by Brush War after NATO //TODO: this one needs work
        EventBlocked, //i.e NATO before Marshall/Warsaw, Arab-Israeli War after Camp David


        InsufficentSpaceRaceValue,
        EnoughSpaceRaceAttempts,

    }
    impl Game {
        pub fn next_turn(&mut self) {
            // at the end of a turn we must:
            //TODO: if we finished turn 10, game ends
            if self.turn == 10 {
                todo!();
            } else {
                //increment turn counter by 1.
                self.turn += 1;

                //TODO:if we are entering turn 4, shuffle in the midwar cards
                //TODO: if we are entering turn 8, shuffle in latewar cards
                if self.turn==4{
                    todo!();
                }else if self.turn==8{
                    todo!();
                }

                //improve DEFCON by 1
                self.mod_defcon(1);
                //TODO: allow both sides to space race again

                //TODO: flip China

                //TODO: deal out cards to both players

                //reset AR to 0 (headliners)
                self.action_round = 0;
            }
        }

        ///modifies DEFCON by the amount (negative degrades, positive improves) and returns the new DEFCON level
        pub fn mod_defcon(&mut self, change: i8) -> i8 {
            self.defcon -= change;
            if self.defcon > 5 {
                self.defcon = 5;
            } else if self.defcon <= 1 {
                //TODO: phasing player loses
                todo!();
            }
            return self.defcon;
        }




        //TODO: contain next game state?
        pub fn is_action_valid(proposition :&Action )->Result<(),ActionError>{
            match proposition{
                Action::Operations(card, power, action) => {
                    //TODO: resolve event if oppo event
                    match action {
                        OpsAction::PlaceInfluence(_) => {Err(ActionError::NotImplemented)}
                        OpsAction::Coup(_) => {Err(ActionError::NotImplemented)}
                        OpsAction::Realignment(_) => {Err(ActionError::NotImplemented)}
                    }
                }
                Action::SpaceRace(_, _) => {
                    //check if this card can be spaceraced (i.e worth enough, also there havent been attepts (or 2 if at goal)
                    todo!();
                    Err(ActionError::NotImplemented)
                }
                Action::Event(_, _) => {
                    todo!();
                    Err(ActionError::NotImplemented)
                }
            }
        }
    }
}
