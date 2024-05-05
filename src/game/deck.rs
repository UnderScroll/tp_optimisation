use super::card::Card;
use super::setlist::SetList;
use rand::seq::SliceRandom;

pub type Deck = [Card; 30];

pub trait DeckTrait {
    fn new(set_list: &SetList) -> Deck;
    fn deck_cost(&self) -> [u32; 30];
}

impl DeckTrait for Deck {
    fn new(set_list: &SetList) -> Deck {
        let mut cards: [Card; 30] = [Card::default(); 30];
        let mut card_indexes = Vec::from_iter(0..set_list.len());
        card_indexes.shuffle(&mut rand::thread_rng());

        for i in 0..30 {
            cards[i] = set_list[card_indexes[i]];
        }

        cards
    }

    fn deck_cost(&self) -> [u32; 30] {
        let mut costs: [u32; 30] = [0; 30];
        for (i, card) in self.iter().enumerate() {
            costs[i] = card.get_cost();
        }
        costs
    }
}
