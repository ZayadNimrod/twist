pub mod cards;
pub mod map;

pub mod game {
    use super::cards;
    use super::map;
    pub fn set_up() {
        //set up the game board/deck
        let mut deck: Vec<cards::Card> = Vec::new();
        cards::populate_deck_early_war(&mut deck);
    }
}