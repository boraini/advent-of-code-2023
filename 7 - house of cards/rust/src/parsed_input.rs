use std::fs::File;
use std::io::{BufReader, BufRead};

use crate::game::Game;
use crate::replace_jokers::replace_jokers;

fn cards_to_numbers(s: &str) -> Vec<i32> {
    s.chars()
        .map(|x: char| if x.is_numeric() {x.to_digit(10).unwrap() as i32} else {match x {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => panic!("Unknown character {} encountered.", x)
        }})
        .collect()
}

pub fn parse_input(do_replace_jokers: bool) -> Vec<Game> {
    let file = File::open("../input.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let mut games: Vec<Game> = Vec::new();

    for line in lines {
        let string = line.unwrap();
        let mut parts = string.split(" ");
        let cards = cards_to_numbers(parts.next().unwrap());
        let bid: i32 = parts.next().unwrap().parse().unwrap();
        let game = if do_replace_jokers {replace_jokers(&cards, bid)} else {Game::new(cards, bid)};
        games.push(game)
    }

    games
}