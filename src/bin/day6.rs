use std::process::exit;

use advent2023::*;

struct Race {
    time: i64,
    record: i64,
}

fn count_winning_options(race: &Race) -> i64 {
    let d: f64 = (i64::pow(race.time, 2) - 4 * race.record) as f64;
    if d <= 0f64 {
        panic!("Must be positive");
    }

    let start = (-race.time as f64 - f64::sqrt(d)) / 2f64;
    let start = (start + 0.0001).ceil();
    let end = (-race.time as f64 + f64::sqrt(d)) / 2f64;
    end as i64 - start as i64
}

fn vec_from_string(str: &str, msg: &str) -> Vec<i64> {
    str.split_once(':')
        .expect(msg)
        .1
        .split_whitespace()
        .map(|num| num.parse::<i64>().expect(msg))
        .collect()
}

fn parse_input(input: &str) -> Vec<Race> {
    let msg = "Incorrect format";
    let mut lines = input.lines();
    let times = vec_from_string(lines.next().expect(msg), msg);
    let records = vec_from_string(lines.next().expect(msg), msg);

    times
        .iter()
        .zip(records.iter())
        .map(|(t, r)| Race {
            time: *t,
            record: *r,
        })
        .collect()
}

fn parse_split_int(line: &str, msg: &str) -> i64 {
    line.split_once(':')
        .expect(msg)
        .1
        .split_whitespace()
        .map(|s| s.chars())
        .flatten()
        .collect::<String>()
        .parse::<i64>()
        .expect(msg)
}

fn parse_input_2(input: &str) -> Race {
    let msg = "Incorrect format";
    let mut lines = input.lines();
    let time = parse_split_int(lines.next().expect(msg), msg);
    let record = parse_split_int(lines.next().expect(msg), msg);

    Race { time, record }
}

fn part_1(input: &str) -> i64 {
    parse_input(input)
        .iter()
        .map(|race| count_winning_options(&race))
        .fold(1, |acc, x| acc * x)
}

fn part_2(input: &str) -> i64 {
    count_winning_options(&parse_input_2(input))
}

fn main() {
    let input = get_input_text().unwrap_or_else(|| {
        println!("Invalid file");
        exit(1)
    });

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(part_1(input), 288);
    }

    #[test]
    fn aoc_2() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(part_2(input), 71503);
    }

    #[test]
    fn parse_2() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let parsed = parse_input_2(input);
        assert_eq!(parsed.time, 71530);
        assert_eq!(parsed.record, 940200);
    }
}
