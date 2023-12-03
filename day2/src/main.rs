use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl Draw {
    fn is_valid(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        self.red <= max_red && self.green <= max_green && self.blue <= max_blue
    }
}

#[derive(Debug)]
struct ParseDrawError;

impl FromStr for Draw {
    type Err = ParseDrawError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;
        for category in s.trim().split(',') {
            let (num, color) = category.trim().split_once(' ').ok_or(ParseDrawError)?;
            let parsed_num = num.parse::<u32>().map_err(|_| ParseDrawError)?;

            if color == "red" {
                red = parsed_num;
            } else if color == "green" {
                green = parsed_num;
            } else if color == "blue" {
                blue = parsed_num;
            } else {
                Err(ParseDrawError)?
            }
        }

        Ok(Draw { red, green, blue })
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
    min_to_play: Draw,
}

impl Game {
    fn is_valid(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        for draw in &self.draws {
            if !draw.is_valid(max_red, max_green, max_blue) {
                return false;
            }
        }

        true
    }

    fn power(&self) -> u32 {
        self.min_to_play.red * self.min_to_play.green * self.min_to_play.blue
    }
}

#[derive(Debug)]
struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, draws) = s
            .strip_prefix("Game ")
            .and_then(|s| s.split_once(":"))
            .ok_or(ParseGameError)?;

        let parsed_id = id.parse::<u32>().map_err(|_| ParseGameError)?;
        let mut parsed_draws: Vec<Draw> = Vec::new();
        let mut min_to_play = Draw {
            red: 0,
            green: 0,
            blue: 0,
        };
        for draw in draws.split(';') {
            let cur_draw = draw.parse::<Draw>().map_err(|_| ParseGameError)?;
            if cur_draw.red > min_to_play.red {
                min_to_play.red = cur_draw.red
            }
            if cur_draw.green > min_to_play.green {
                min_to_play.green = cur_draw.green
            }
            if cur_draw.blue > min_to_play.blue {
                min_to_play.blue = cur_draw.blue
            }
            parsed_draws.push(cur_draw);
        }

        Ok(Game {
            id: parsed_id,
            draws: parsed_draws,
            min_to_play,
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file =
        fs::read_to_string(args.get(1).expect("Enter file name")).expect("Failed to read file");

    let mut sum_of_ids: u32 = 0;
    let mut sum_of_powers: u32 = 0;
    let max_red: u32 = 12;
    let max_green: u32 = 13;
    let max_blue: u32 = 14;

    for game in file.split('\n') {
        match game.parse::<Game>() {
            Ok(game) => {
                sum_of_powers += game.power();
                if game.is_valid(max_red, max_green, max_blue) {
                    sum_of_ids += game.id
                }
            }
            Err(_) => (),
        }
    }

    println!(
        "Sum of ids: {}\nSum of powers: {}",
        sum_of_ids, sum_of_powers
    );
}
