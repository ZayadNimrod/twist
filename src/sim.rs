pub mod cards;
pub mod map;

pub mod game {

    use super::cards;
    use super::map;

    pub struct Game {
        deck: Vec<cards::Card>,
        discards: Vec<cards::Card>,
    }

    //TODO: don't we want to return a reference, not copy the data out? Box<> it?
    pub fn set_up_game() -> Game {
        //set up the game board/deck
        let mut state: Game = Game {
            deck: Vec::new(),
            discards: Vec::new(),
        };
        cards::populate_deck_early_war(&mut state.deck);

        return state;
    }

    impl Game {}
}
