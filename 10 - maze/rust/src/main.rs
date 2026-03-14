mod has_connection;
mod parsed_input;
mod part1;

fn main() {
    let input = parsed_input::parse_input();

    let mut i = 0;
    let mut j = 0;
    for line in &input {
        let i_opt = line.find('S');
        match i_opt {
            Some(idx) => {
                j = idx;
                break;
            },
            None => {}
        }
        i += 1;
    }
    println!("Part 1: {}", part1::solve(&input, i, j));
}
