use std::{io::{self, BufRead}, fs::File};

use parse_input::parse_input;

use crate::check_game::{sum_games, sum_powers};

mod parse_input;
mod check_game;

fn main() {
    let file = File::open("../input.txt").unwrap();

    let input_file_lines = io::BufReader::new(file).lines();
    let input_lines = input_file_lines.map(|l| l.unwrap());
    let problem = parse_input(input_lines);

    println!("Sum of IDs of Possible Games: {}", sum_games(&problem, 12, 13, 14));
    println!("Sum of Powers: {}", sum_powers(&problem));
}
