pub mod card;
pub mod deck;
pub mod setlist;
pub mod weighted_setlist;

use card::Card;
use deck::Deck;
use setlist::SetList;

use self::{card::PlayedCard, setlist::SetListTrait};
use crate::game::deck::DeckTrait;
use rand::prelude::SliceRandom;

#[derive(Clone)]
pub struct Player {
    pv: i32,
    pub base_deck: Deck,
    pub deck: Deck,
    mana: u32,
    max_mana: u32,
    card_to_draw: usize,
    hand: Vec<Card>,
    board: Vec<Card>,
}

pub trait Queue {
    fn pop(&self);
}

impl Player {
    fn new(deck: Deck) -> Player {
        Player {
            pv: 100,
            base_deck: deck,
            deck: deck,
            mana: 0,
            max_mana: 0,
            card_to_draw: 0,
            hand: Vec::with_capacity(30),
            board: Vec::with_capacity(30),
        }
    }
}

pub struct Game {
    pub p1: Player,
    pub p2: Player,
    pub setlist: SetList,
}

#[derive(Debug)]
pub struct Stats {
    pub player1_won: bool,
    pub nb_turns: u32,
    pub deck_cost: [u32; 30],
}

impl Game {
    pub fn new() -> Game {
        let setlist = SetList::gen_all();
        let deck1 = Deck::new(&setlist);
        let deck2 = Deck::new(&setlist);

        Game {
            p1: Player::new(deck1),
            p2: Player::new(deck2),
            setlist,
        }
    }

    pub fn play(&mut self) -> Stats {
        self.p1 = Player::new(self.p1.base_deck);
        self.p2 = Player::new(self.p2.base_deck);

        self.p1.deck.shuffle(&mut rand::thread_rng());
        self.p2.deck.shuffle(&mut rand::thread_rng());

        //Draw 3 cards
        for _ in 0..3 {
            let drawn_card = &self.p1.deck[self.p1.card_to_draw];
            self.p1.card_to_draw += 1;

            //Insert card in order
            let mut index = self.p1.hand.len();
            for (i, card) in self.p1.hand.iter().enumerate() {
                if card.get_cost() <= drawn_card.get_cost() {
                    index = i;
                    break;
                }
            }

            self.p1.hand.insert(index, *drawn_card);
        }

        for _ in 0..3 {
            let drawn_card = &self.p2.deck[self.p2.card_to_draw];
            self.p2.card_to_draw += 1;

            //Insert card in order
            let mut index = self.p2.hand.len();
            for (i, card) in self.p2.hand.iter().enumerate() {
                if card.get_cost() <= drawn_card.get_cost() {
                    index = i;
                    break;
                }
            }

            self.p2.hand.insert(index, *drawn_card);
        }

        let mut turn_nb = 0;
        loop {
            if Self::do_turn(&mut self.p1, &mut self.p2) {
                return Stats {
                    player1_won: true,
                    nb_turns: turn_nb,
                    deck_cost: self.p1.base_deck.deck_cost(),
                };
            }

            if Self::do_turn(&mut self.p2, &mut self.p1) {
                return Stats {
                    player1_won: false,
                    nb_turns: turn_nb,
                    deck_cost: self.p2.base_deck.deck_cost(),
                };
            }

            turn_nb += 1;
        }
    }

    fn do_turn(p1: &mut Player, p2: &mut Player) -> bool {
        //Set mana
        p1.max_mana += 1;
        p1.mana = p1.max_mana;

        //Draw card in hand
        if p1.card_to_draw < 30 {
            let drawn_card = &p1.deck[p1.card_to_draw];
            p1.card_to_draw += 1;

            //Insert card in order
            let mut index = p1.hand.len();
            for (i, card) in p1.hand.iter().enumerate() {
                if card.get_cost() <= drawn_card.get_cost() {
                    index = i;
                    break;
                }
            }

            p1.hand.insert(index, *drawn_card);
        }

        //Place cards
        let mut new_hand = Vec::with_capacity(30);
        for card in p1.hand.iter() {
            if card.get_cost() > p1.mana {
                new_hand.push(*card);
                continue;
            }

            p1.board.push(*card);

            p1.mana -= card.get_cost();
        }

        p1.hand = new_hand.iter().cloned().collect();

        //Attack opposite player
        let mut new_board = vec![];
        let mut new_opposite_board = vec![];
        for card in &p1.board {
            let mut has_attacked = false;
            let mut overflow = 0;
            let opposit_board = p2.board.iter().map(|card| PlayedCard::new(card.clone()));
            for mut opposit_card in opposit_board {
                if should_attack_card(card, &opposit_card.card) {
                    let result = resolve_card_fight(&card, &opposit_card);
                    if result.card_a_survived {
                        new_board.push(*card);
                    }
                    if result.card_b_survived {
                        new_opposite_board.push(opposit_card.card);
                        opposit_card.defense_left -= card.get_attack();
                    }
                    overflow = result.overflow;
                    has_attacked = true;
                    break;
                }
            }
            if !has_attacked {
                p2.pv -= card.get_attack() as i32;
            }
            if card.get_trample() && overflow > 0 {
                p2.pv -= overflow;
            }
        }
        p1.board = new_board;
        p2.board = new_opposite_board;

        //Returns if p1 won
        p2.pv <= 0
    }

    pub fn switch_player(&mut self) {
        let p1 = self.p1.clone();
        self.p1 = self.p2.clone();
        self.p2 = p1;
    }
}

fn should_attack_card(a: &Card, b: &Card) -> bool {
    if a.get_distortion() {
        b.get_distortion() && b.get_taunt()
    } else {
        b.get_taunt()
    }
}

struct CardFightResult {
    card_a_survived: bool,
    card_b_survived: bool,
    overflow: i32,
}

fn resolve_card_fight(card_a: &Card, card_b: &PlayedCard) -> CardFightResult {
    let can_card_a_survive = card_b.card.get_attack() < card_a.get_defense();
    let can_card_b_survive = card_a.get_attack() < card_b.defense_left;

    let card_a_survived = if card_a.get_first_strike() && !card_b.card.get_first_strike() {
        !can_card_b_survive || can_card_a_survive
    } else {
        can_card_a_survive
    };

    let card_b_survived = if card_b.card.get_first_strike() && !card_a.get_first_strike() {
        !can_card_a_survive || can_card_b_survive
    } else {
        can_card_b_survive
    };

    CardFightResult {
        card_a_survived,
        card_b_survived,
        overflow: card_b.defense_left as i32 - card_a.get_attack() as i32,
    }
}
