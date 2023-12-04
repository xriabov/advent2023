use advent2023::*;
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

fn parse_games(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| line.parse::<Game>())
        .filter(|res| res.is_ok())
        .map(|ok| ok.unwrap())
        .collect()
}

fn sum_of_ids(games: &Vec<Game>, red: u32, green: u32, blue: u32) -> u32 {
    games
        .iter()
        .filter(|game| game.is_valid(red, green, blue))
        .map(|game| game.id)
        .sum()
}

fn sum_of_powers(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .map(|game| game.min_to_play.red * game.min_to_play.green * game.min_to_play.blue)
        .sum()
}

fn main() {
    let file = get_input_text().unwrap_or_else(|| {
        println!("Invalid input file");
        std::process::exit(1);
    });

    let games = parse_games(&file);
    let sum_of_ids: u32 = sum_of_ids(&games, 12, 13, 14);
    let sum_of_powers: u32 = sum_of_powers(&games);
    println!(
        "Sum of ids: {}\nSum of powers: {}",
        sum_of_ids, sum_of_powers
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let games = parse_games(input);
        assert_eq!(sum_of_ids(&games, 12, 13, 14), 8);
    }

    #[test]
    fn aoc_2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let games = parse_games(input);
        assert_eq!(sum_of_powers(&games), 2286);
    }
}
