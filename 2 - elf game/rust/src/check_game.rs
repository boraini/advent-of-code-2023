use std::cmp::max;

use crate::parse_input::{Game, Draw};

pub fn check_game(game: &Game, n_red: u32, n_green: u32, n_blue: u32) -> bool {
    game.draws.iter().all(|d| d.red <= n_red && d.green <= n_green && d.blue <= n_blue)
}

pub fn number_of_cubes_needed(game: &Game) -> Draw {
    game.draws.iter().fold(Draw{ red: 0, green: 0, blue: 0 },
        |acc, d| Draw {
            red: max(acc.red, d.red),
            green: max(acc.green, d.green),
            blue: max(acc.blue, d.blue),
        }
    )
}

pub fn sum_games(games: &Vec<Game>, n_red: u32, n_green: u32, n_blue: u32) -> u32 {
    games.iter().filter(|game| check_game(game, n_red, n_green, n_blue)).fold(0u32, |acc, g| acc + g.id)
}

pub fn sum_powers(games: &Vec<Game>) -> u32 {
    games.iter().map(|game| {
        let values = number_of_cubes_needed(game);

        values.red * values.green * values.blue
    }).fold(0u32, |acc, p| acc + p)
}