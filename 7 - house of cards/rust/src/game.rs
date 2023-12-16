use std::fmt::Debug;

#[derive(PartialEq, PartialOrd, Eq, Clone, Debug)]
pub enum Strength {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(PartialEq, Debug)]
pub struct Game {
    pub cards: Vec<i32>,
    pub bid: i32,
    pub strength: Option<Strength>,
}

impl Game {   
    pub fn new(cards: Vec<i32>, bid: i32) -> Game {
        let mut game = Game { cards, bid, strength: None };

        game.compute_strength();

        game
    }

    pub fn compute_strength_vec(sorted: &Vec<i32>) -> Strength {
        if sorted[0] == sorted[4] { return Strength::FiveOfAKind; }
        if sorted[0] == sorted[3] || sorted[1] == sorted[4] { return Strength::FourOfAKind; }
        if sorted[0] == sorted[2] {
            if sorted[3] == sorted[4] { return Strength::FullHouse; }
            return Strength::ThreeOfAKind;
        }
        if sorted[2] == sorted[4] {
            if sorted[0] == sorted[1] { return Strength::FullHouse; }
            return Strength::ThreeOfAKind;
        }
        if sorted[1] == sorted[3] {
            return Strength::ThreeOfAKind;
        }
        if sorted[0] == sorted[1] && (sorted[2] == sorted[3] || sorted[3] == sorted[4]) || (sorted[1] == sorted[2] && sorted[3] == sorted[4]) {
            return Strength::TwoPair;
        }
        if sorted[0] == sorted[1] || sorted[1] == sorted[2] || sorted[2] == sorted[3] || sorted[3] == sorted[4] {
            return Strength::OnePair;
        }
        return Strength::HighCard;
    } 

    fn compute_strength_internal(&self) -> Strength {
        let mut sorted = self.cards.clone();
        sorted.sort();
        Game::compute_strength_vec(&sorted)
    }

    pub fn compute_strength(&mut self) {
        self.strength = Some(self.compute_strength_internal());
    }

    pub fn get_strength(&self) -> Strength {
        match &self.strength {
            Some(x) => x.clone(),
            None => panic!("Strength was not precomputed."),
        }
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.get_strength().partial_cmp(&other.get_strength()) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.cards.partial_cmp(&other.cards)
    }
}
