use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn parse_input() -> Vec<String> {
    let f = File::open("../input_short.txt").unwrap();
    BufReader::new(f).lines().map(|l| l.unwrap()).collect()
}