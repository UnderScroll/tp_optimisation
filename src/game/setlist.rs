use super::card::Card;
use rand::Rng;

pub(crate) type SetList = Box<[Card]>;

pub trait SetListTrait {
    fn gen_all() -> SetList;
    fn get_rand(&self) -> Card;
}

impl SetListTrait for Box<[Card]> {
    fn gen_all() -> SetList {
        let mut set_list = vec![];

        for def in 1..=16 {
            for atk in 0..=15 {
                for prov in 0..=1 {
                    for dist in 0..=1 {
                        for trmpl in 0..=1 {
                            for f_strike in 0..=1 {
                                if prov == 0 && atk == 0 {
                                    continue;
                                }
                                if f_strike == 1 && atk == 0 {
                                    continue;
                                }
                                if trmpl == 1 && atk < 2 {
                                    continue;
                                }
                                let card: Card = Card::new(
                                    atk,
                                    def,
                                    prov != 0,
                                    dist != 0,
                                    trmpl != 0,
                                    f_strike != 0,
                                );
                                if card.get_cost() > 8 {
                                    break;
                                }
                                set_list.push(card);
                            }
                        }
                    }
                }
            }
        }

        set_list.into_boxed_slice()
    }

    fn get_rand(&self) -> Card {
        self[rand::thread_rng().gen_range(0..self.len())]
    }
}
