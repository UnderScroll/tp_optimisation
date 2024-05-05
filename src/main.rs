mod game;

use std::{fs::File, io::Write, time};

use game::{deck::Deck, weighted_setlist::WeightedSetlist, Game};
use rand::Rng;
fn main() {
    let mut winrate_file = File::create("winrate.csv").unwrap();
    winrate_file.set_len(0).unwrap();

    let mut deck_cost_file = File::create("deck_cost.csv").unwrap();
    deck_cost_file.set_len(0).unwrap();

    let mut deck_atk_file = File::create("deck_atk.csv").unwrap();
    deck_atk_file.set_len(0).unwrap();

    let mut deck_def_file = File::create("deck_def.csv").unwrap();
    deck_def_file.set_len(0).unwrap();

    let mut deck_prov_file = File::create("deck_prov.csv").unwrap();
    deck_prov_file.set_len(0).unwrap();

    let mut deck_dist_file = File::create("deck_dist.csv").unwrap();
    deck_dist_file.set_len(0).unwrap();

    let mut deck_trmpl_file = File::create("deck_trmpl.csv").unwrap();
    deck_trmpl_file.set_len(0).unwrap();

    let mut deck_f_strike_file = File::create("deck_f_strike.csv").unwrap();
    deck_f_strike_file.set_len(0).unwrap();

    let mut final_deck_a_file = File::create("deck_final_a.data").unwrap();
    final_deck_a_file.set_len(0).unwrap();
    let mut final_deck_b_file = File::create("deck_final_b.data").unwrap();
    final_deck_b_file.set_len(0).unwrap();

    let mut game = Game::new();

    let start = time::Instant::now();

    let mut previous_total_wins = 0;
    let mut previous_deck = game.p1.base_deck.clone();

    println!("{}", game.setlist.len());

    for _ in 0..1 {
        let mut weighted_setlist = WeightedSetlist::new(game.setlist.clone(), 100);
        let mut change_card;
        let mut index_changed = 0;
        let mut last_index_changed = 0;
        for _ in 0..1000 {
            let n_games: u32 = 5000;

            let (total_wins1, total_turns1, _) = play_games(&mut game, n_games as usize);
            //switch player
            let c = game.p1;
            game.p1 = game.p2;
            game.p2 = c;

            let (total_wins2, total_turns2, _) = play_games(&mut game, n_games as usize);

            //switch player back
            let c = game.p1;
            game.p1 = game.p2;
            game.p2 = c;

            let total_wins = total_wins1 + n_games - total_wins2;
            let avg_turn = (total_turns1 + total_turns2) as f32 / (n_games as f32 * 2.0);

            if previous_total_wins > total_wins {
                log_stats(
                    &mut winrate_file,
                    &mut deck_cost_file,
                    &mut deck_atk_file,
                    &mut deck_def_file,
                    &mut deck_prov_file,
                    &mut deck_dist_file,
                    &mut deck_trmpl_file,
                    &mut deck_f_strike_file,
                    &previous_deck,
                    &previous_total_wins,
                    &avg_turn,
                );

                //Update deck
                game.p1.base_deck = previous_deck.to_owned();
                let len = game.p1.base_deck.len();

                weighted_setlist.change_weight(index_changed, 1);
                weighted_setlist.change_weight(last_index_changed, -1);

                (change_card, last_index_changed) = weighted_setlist.get_rand();
                while game
                    .p1
                    .base_deck
                    .iter()
                    .filter(|card| **card == change_card)
                    .count()
                    > 2
                {
                    (change_card, last_index_changed) = weighted_setlist.get_rand();
                }

                index_changed = rand::thread_rng().gen_range(0..len);
                game.p1.base_deck[index_changed] = change_card;
            } else {
                log_stats(
                    &mut winrate_file,
                    &mut deck_cost_file,
                    &mut deck_atk_file,
                    &mut deck_def_file,
                    &mut deck_prov_file,
                    &mut deck_dist_file,
                    &mut deck_trmpl_file,
                    &mut deck_f_strike_file,
                    &previous_deck,
                    &previous_total_wins,
                    &avg_turn,
                );

                //Update deck
                previous_total_wins = total_wins;
                previous_deck = game.p1.base_deck.to_owned();
                let len = previous_deck.len();

                weighted_setlist.change_weight(index_changed, -1);
                weighted_setlist.change_weight(last_index_changed, 1);

                (change_card, last_index_changed) = weighted_setlist.get_rand();
                while game
                    .p1
                    .base_deck
                    .iter()
                    .filter(|card| **card == change_card)
                    .count()
                    > 2
                {
                    (change_card, last_index_changed) = weighted_setlist.get_rand();
                }

                index_changed = rand::thread_rng().gen_range(0..len);
                game.p1.base_deck[index_changed] = change_card;
            }
        }

        //Switch player optimisation
        game.switch_player();

        previous_total_wins = 0;
        previous_deck = game.p1.base_deck.clone();

        let end = time::Instant::now();
        println!("Elapsed : {:?}", end - start);
    }

    final_deck_a_file
        .write_all(
            format!(
                "{{\n\t\"Cards\": {}}}",
                serde_json::to_string_pretty(&game.p1.deck).unwrap()
            )
            .as_bytes(),
        )
        .unwrap();
    final_deck_b_file
        .write_all(
            format!(
                "{{\n\t\"Cards\": {}}}",
                serde_json::to_string_pretty(&game.p2.deck).unwrap()
            )
            .as_bytes(),
        )
        .unwrap();
}

fn play_games(game: &mut Game, n: usize) -> (u32, u32, [u32; 30]) {
    let mut player1_win_count = 0;
    let mut total_turns = 0;
    let mut deck_cost = [0; 30];
    for _ in 0..n {
        let s = game.play();
        player1_win_count += s.player1_won as u32;
        total_turns += s.nb_turns;
        deck_cost = s.deck_cost;
    }

    (player1_win_count, total_turns, deck_cost)
}

fn log_stats(
    winrate_file: &mut File,
    deck_cost_file: &mut File,
    deck_atk_file: &mut File,
    deck_def_file: &mut File,
    deck_prov_file: &mut File,
    deck_dist_file: &mut File,
    deck_trmpl_file: &mut File,
    deck_f_strike_file: &mut File,
    deck: &Deck,
    win_amnt: &u32,
    avg_turn: &f32,
) {
    winrate_file
        .write_all(format!("{},{}\n", win_amnt, avg_turn).as_bytes())
        .unwrap();

    let mut arr = Vec::with_capacity(30);
    for card in deck {
        arr.push(card.get_cost())
    }
    write_arr_in_file(arr.as_slice(), deck_cost_file);

    let mut arr = Vec::with_capacity(30);
    for card in deck {
        arr.push(card.get_attack())
    }
    write_arr_in_file(arr.as_slice(), deck_atk_file);

    let mut arr = Vec::with_capacity(30);
    for card in deck {
        arr.push(card.get_defense())
    }
    write_arr_in_file(arr.as_slice(), deck_def_file);

    let mut arr = Vec::with_capacity(30);
    for card in deck {
        arr.push(card.get_taunt() as u32)
    }
    write_arr_in_file(arr.as_slice(), deck_prov_file);

    let mut arr = Vec::with_capacity(30);
    for card in deck {
        arr.push(card.get_distortion() as u32)
    }
    write_arr_in_file(arr.as_slice(), deck_dist_file);

    let mut arr = Vec::with_capacity(30);
    for card in deck {
        arr.push(card.get_trample() as u32)
    }
    write_arr_in_file(arr.as_slice(), deck_trmpl_file);

    let mut arr = Vec::with_capacity(30);
    for card in deck {
        arr.push(card.get_first_strike() as u32)
    }
    write_arr_in_file(arr.as_slice(), deck_f_strike_file);
}

fn write_arr_in_file(values: &[u32], file: &mut File) {
    for i in 0..values.len() {
        if i < values.len() - 1 {
            file.write_all(format!("{:?}, ", values[i]).as_bytes())
                .unwrap()
        } else {
            file.write_all(format!("{:?}\n", values[i]).as_bytes())
                .unwrap()
        }
    }
}
