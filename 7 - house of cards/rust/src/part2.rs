use crate::game::Game;

pub fn solve_part2(games: &[Game]) -> i32 {
    let mut sum = 0;
    for (i, game) in games.iter().enumerate() {
        sum += (i as i32 + 1) * game.bid;
    }

    sum
}