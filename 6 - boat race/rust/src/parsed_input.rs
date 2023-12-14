use std::io::{BufRead, BufReader};
use std::fs::File;

fn get_numbers(a: String) -> Vec<i64> {
    let parts = a.split_whitespace();
    parts.skip(1).map(|x| x.parse().unwrap()).collect()
}

fn get_number(a: String) -> i64 {
    let parts = a.split_whitespace();
    let s: String = parts.skip(1).collect();
    println!("{}", s);
    s.parse().unwrap()
}

fn parse_raw() -> (String, String) {
    let file = File::open("../input.txt").unwrap();
    let mut lines = BufReader::new(file).lines();
    let raw_times = lines.next().unwrap().unwrap();
    let raw_distances = lines.next().unwrap().unwrap();
    (raw_times, raw_distances)
}

pub fn parse_input() -> (Vec<i64>, Vec<i64>) {
    let (raw_times, raw_distances) = parse_raw();
    let times = get_numbers(raw_times);
    let distances = get_numbers(raw_distances);
    (times, distances)
}

pub fn parse_input_no_kerning() -> (Vec<i64>, Vec<i64>) {
    let (raw_times, raw_distances) = parse_raw();
    let time = get_number(raw_times);
    let distance = get_number(raw_distances);
    (vec!(time), vec!(distance))
}