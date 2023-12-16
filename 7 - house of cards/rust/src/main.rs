mod parsed_input;
mod game;
mod part1;
mod part2;
mod replace_jokers;

fn main() {
    let mut games = parsed_input::parse_input(false);
    games.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    println!("Part 1: {}", part1::solve_part1(&games));

    let mut games = parsed_input::parse_input(true);
    games.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    println!("Part 2: {}", part2::solve_part2(&games));
}
