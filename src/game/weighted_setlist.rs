use rand::Rng;

use super::{card::Card, setlist::SetList};

pub struct WeightedSetlist {
    setlist: SetList,
    weights: Vec<u32>,
    sum_weights: u64,
}

impl WeightedSetlist {
    pub fn new(setlist: SetList, default_weight: u32) -> WeightedSetlist {
        let size = setlist.to_owned().len();
        WeightedSetlist {
            setlist: setlist,
            weights: vec![default_weight; size],
            sum_weights: size as u64 * default_weight as u64,
        }
    }

    pub fn get_rand(&self) -> (Card, usize) {
        let mut index = 0;
        let mut cumul = 0;
        let rand = rand::thread_rng().gen_range(0..self.sum_weights);
        for (i, weight) in self.weights.iter().enumerate() {
            cumul += *weight as u64;
            index = i;
            if rand <= cumul {
                break;
            }
        }

        (self.setlist[index], index)
    }

    pub fn change_weight(&mut self, index: usize, change: i32) {
        self.weights[index] = if self.weights[index] == 0 && change < 0 {
            0
        } else {
            if change > 0 {
                let change = change.abs() as u32;
                self.sum_weights += change as u64;
                self.weights[index] + change
            } else {
                let change = change.abs() as u32;
                self.sum_weights -= change as u64;
                self.weights[index] - change
            }
        }
    }
}
