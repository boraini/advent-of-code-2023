use parsed_input::parse_input;

use crate::{part1::solve_part1, parsed_input::parse_input_no_kerning};

mod parsed_input;
mod util;
mod part1;

fn main() {
    let (times, distances) = parse_input();
    let (time, distance) = parse_input_no_kerning();

    println!("Part 1 solution is {}.", solve_part1(&times, &distances));
    println!("Part 2 solution is {}.", solve_part1(&time, &distance));
}
