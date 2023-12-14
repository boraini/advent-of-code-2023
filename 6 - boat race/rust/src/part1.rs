use crate::util::find_distance;

pub fn solve_part1(times: &[i64], distances: &[i64]) -> i64 {
    let mut product = 1;
    for (time, distance) in times.iter().zip(distances.iter()) {
        let mut hold_time = 0;
        let mut count = 0;
        while &hold_time <= time {
            let cand = find_distance(time, &hold_time);
            if &cand > distance {
                count += 1;
            }
            // println!("You go {} millimeters if you hold for {} milliseconds.", cand, hold_time);
            hold_time += 1;
        }
        if count >= 0 {
            product *= count;
        }
    }
    
    product
}