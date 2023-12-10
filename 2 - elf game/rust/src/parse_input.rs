pub struct Draw {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

pub struct Game {
    pub id: u32,
    pub draws: Vec<Draw>,
}

impl ToString for Draw {
    fn to_string(&self) -> String {
        format!("Draw red: {} green: {} blue: {}", self.red, self.green, self.blue)
    }
}

pub fn parse_input(lines: impl Iterator<Item = String>) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();

    for line in lines {
        let mut colon_iter = line.split(": ");
        let game_part = colon_iter.next().unwrap();
        let draws_part = colon_iter.next().unwrap();

        let game_id: u32 = game_part.rsplit(" ").next().unwrap().parse().unwrap();

        let mut draws: Vec<Draw> = Vec::new();

        for draw_str in draws_part.split("; ") {
            let mut red = 0u32;
            let mut green = 0u32;
            let mut blue = 0u32;

            for color_str in draw_str.split(", ") {
                let mut kv = color_str.split(" ");
                let n: u32 = kv.next().unwrap().parse().unwrap();
                let color = kv.next().unwrap();

                match color {
                    "red" => red = n,
                    "green" => green = n,
                    "blue" => blue = n,
                    &_ => todo!(),
                }
            }

            let draw = Draw { red, green, blue };

            draws.push(draw);
        }

        let game = Game { id: game_id, draws };

        games.push(game);
    }

    return games;
}