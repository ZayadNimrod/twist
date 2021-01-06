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
    use crate::sim::map::Superpower;

    pub struct Game {
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
            deck: Vec::new(),
            discards: Vec::new(),
            phasing_player: Superpower::USSR,
            action_round: 0,
            turn: 1,
            defcon: 5,
        };
        cards::populate_deck_early_war(&mut state.deck);

        return state;
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
    }
}
