use crate::game::{Game, Strength};

pub fn replace_jokers(cards: &Vec<i32>, bid: i32) -> Game {
    // build the map slice: starting from index [start], each index contains the index of the next joker.
    let mut map: [usize; 5] = [0, 0, 0, 0, 0];
    let mut start = 6;

    for (i, value) in map.iter_mut().enumerate().rev() {
        *value = if cards[i] == 11 {
            let val = start;
            start = i;
            val
        } else {
            6
        };
    }

    // if there are no jokers
    if start == 6 {
        return Game::new(cards.to_owned(), bid)
    }

    // brute force all possible joker replacements
    let mut cont = true;
    let mut replacement: [i32; 5] = [0, 0, 0, 0, 0];
    replacement.fill(2);

    let mut result_strength = Strength::HighCard;

    while cont {
        let original_cards = cards.clone();
        let mut cards = original_cards.clone();

        if !cont {
            break;
        }

        let mut i = start;
        while i < 6 {
            cards[i] = replacement[i];
            i = map[i];
        }

        cards.sort_by(|a, b| a.partial_cmp(&b).unwrap());

        let cand_strength = Game::compute_strength_vec(&cards);

        if cand_strength > result_strength {
            result_strength = cand_strength;
        }

        // 2 to 14 inclusive positional counting
        let mut i = start;
        while i < 6 {
            replacement[i] += 1;
            if replacement[i] == 15 {
                replacement[i] = 2;
                if map[i] == 6 {
                    cont = false;
                }
            } else {
                break;
            }
            i = map[i];
        }
    }

    // replace jokers with low rank
    let mut cards = cards.clone();

    let mut i = start;
    while i < 6 {
        cards[i] = 1;
        i = map[i];
    }

    // enforce the game strength - don't use Game::new
    Game{ cards, bid, strength: Some(result_strength) }
}