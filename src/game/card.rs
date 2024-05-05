#![allow(non_camel_case_types, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    Attack: u32,
    Defense: u32,
    HasTaunt: bool,
    HasDistortion: bool,
    HasTrample: bool,
    HasFirstStrike: bool,
    Cost: u32,
}

pub struct PlayedCard {
    pub card: Card,
    pub defense_left: u32,
}

impl PlayedCard {
    pub fn new(card: Card) -> PlayedCard {
        PlayedCard {
            card,
            defense_left: card.get_defense(),
        }
    }
}

impl Card {
    pub fn new(
        attack: u32,
        defense: u32,
        provocation: bool,
        distortion: bool,
        trample: bool,
        first_strike: bool,
    ) -> Card {
        Card {
            Attack: attack,
            Defense: defense,
            HasTaunt: provocation,
            HasDistortion: distortion,
            HasTrample: trample,
            HasFirstStrike: first_strike,
            Cost: Self::compute_cost(
                attack,
                defense,
                provocation,
                distortion,
                trample,
                first_strike,
            ),
        }
    }

    pub fn get_attack(&self) -> u32 {
        self.Attack
    }

    pub fn get_defense(&self) -> u32 {
        self.Defense
    }

    pub fn get_taunt(&self) -> bool {
        self.HasTaunt
    }

    pub fn get_distortion(&self) -> bool {
        self.HasDistortion
    }

    pub fn get_trample(&self) -> bool {
        self.HasTrample
    }

    pub fn get_first_strike(&self) -> bool {
        self.HasFirstStrike
    }

    pub fn get_cost(&self) -> u32 {
        self.Cost
    }

    pub fn get_name(&self) -> String {
        let cost_name = match self.Cost {
            1 => "Stagiaire",
            2 => "Poussin",
            3 => "Chaton",
            4 => "Goblin",
            5 => "Prog",
            6 => "Ectolion",
            7 => "Dragon",
            8 => "Axel",
            _ => "[ERROR: COST_NAME]",
        };

        let atk_name = match self.Attack {
            0 => "Pacifiste",
            1..=3 => "Innofensif",
            4..=6 => "Motive",
            7..=9 => "Hargneux",
            10..=12 => "Violent",
            _ => "[ERROR: ATK_NAME]",
        };

        let def_name = match self.Defense {
            1 => "en Mousse",
            2..=4 => "en Carton",
            5..=7 => "en Plastique",
            8..=10 => "en Terre cuite",
            11..=13 => "en Chene",
            14..=16 => "de Metal",
            _ => "[ERROR: DEF_NAME]",
        };

        let prov_name = if self.HasTaunt { " GD" } else { "" };
        let dist_name = if self.HasDistortion {
            " Parlementaire"
        } else {
            ""
        };
        let trample_name = if self.HasTrample { " Geant" } else { "" };
        let f_strike_name = if self.HasFirstStrike { " Fourbe" } else { "" };

        format!(
            "{cost_name} {atk_name}{prov_name}{dist_name}{trample_name}{f_strike_name} {def_name}"
        )
    }

    fn compute_cost(
        atk: u32,
        def: u32,
        prov: bool,
        dist: bool,
        trmpl: bool,
        f_strike: bool,
    ) -> u32 {
        (((atk + def) as f32 / 2.0)
            + (prov as u32 as f32) * 1.5
            + (dist as u32 + trmpl as u32 + f_strike as u32) as f32)
            .ceil() as u32
    }
}
