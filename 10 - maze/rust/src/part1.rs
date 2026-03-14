use crate::has_connection::{HAS_CONNECTION, Connection};

pub fn solve(input: &Vec<String>, start_i: usize, start_j: usize) -> usize {
    let mut steps = 0;
    'eachdirection: for c in vec!(Connection::Top, Connection::Left, Connection::Bottom, Connection::Right) {
        let mut i = start_i;
        let mut j = start_j;
        let mut current_symbol = input[i].chars().nth(j).unwrap();
        let mut current_direction = c.clone();
        let mut my_steps = 0;
        println!{"My steps: {}, Direction: {}", my_steps, &c};
        // do-while loop
        while {
            let connections = HAS_CONNECTION.get(&current_symbol).unwrap();
            current_direction = connections[current_direction.clone()].clone();
            match current_direction {
                Connection::Left => if i > 0 {i -= 1} else {continue 'eachdirection;},
                Connection::Right => {i += 1},
                Connection::Top => if j > 0 {j -= 1} else {continue 'eachdirection;},
                Connection::Bottom => {j += 1},
                Connection::NoConnection => continue 'eachdirection,
            };
            println!("Pointer moved.");
            let current_symbol_option = if i >= input.len() {None} else {input[i].chars().nth(j)};
            current_symbol = match current_symbol_option {
                Some(s) => s,
                None => {
                    println!("Dead end!");
                    continue 'eachdirection;
                },
            };
            println!("Stepped.");
            my_steps += 1;
            println!("{} {} vs {} {}", i, j, start_i, start_j);
            i != start_i || j != start_j
        } {
            println!("Continuing!");
        }
        println!{"My steps: {}, Direction: {}", my_steps, &c};
        if my_steps > steps {steps = my_steps;}
    }
    return (steps - 1) / 2;
}