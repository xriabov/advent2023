use advent2023::*;
use std::{collections::HashMap, process::exit};

fn parse(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let msg = "Invalid file";
    let mut lines = input.lines();
    let turns = lines.next().expect(msg);
    lines.next();

    let mut map = HashMap::new();
    for line in lines {
        let split = line.split_once(" = ").expect(msg);
        let (left, right) = split
            .1
            .strip_prefix('(')
            .expect(msg)
            .strip_suffix(')')
            .expect(msg)
            .split_once(", ")
            .expect(msg);
        map.insert(split.0, (left, right));
    }

    (turns, map)
}

fn parse_2(input: &str) -> (&str, Vec<&str>, HashMap<&str, (&str, &str)>) {
    let msg = "Invalid file";
    let mut lines = input.lines();
    let turns = lines.next().expect(msg);
    let mut starts = vec![];
    lines.next();

    let mut map = HashMap::new();
    for line in lines {
        let split = line.split_once(" = ").expect(msg);
        if *split.0.as_bytes().last().expect(msg) as char == 'A' {
            starts.push(split.0);
        }
        let (left, right) = split
            .1
            .strip_prefix('(')
            .expect(msg)
            .strip_suffix(')')
            .expect(msg)
            .split_once(", ")
            .expect(msg);
        map.insert(split.0, (left, right));
    }

    (turns, starts, map)
}

fn part1(input: &str) -> u32 {
    let (turns, map) = parse(input);
    let turns = turns.as_bytes();
    let mut steps = 0;
    let mut i = 0;
    let mut cur = "AAA";
    loop {
        if cur == "ZZZ" {
            break;
        }

        let (left, right) = map.get(cur).expect("Node not found");
        if turns[i] as char == 'L' {
            cur = left;
        } else if turns[i] as char == 'R' {
            cur = right;
        } else {
            panic!("Unexpected turn");
        }

        steps += 1;
        i += 1;
        if i >= turns.len() {
            i = 0;
        }
    }

    steps
}

fn part2(input: &str) -> u32 {
    let (turns, starts, map) = parse_2(input);
    let turns = turns.as_bytes();
    let mut steps = 0;
    let mut lengths: Vec<u32> = vec![0; starts.len()];
    let mut i = 0;
    let mut cur = starts.clone();
    loop {
        if lengths.iter().all(|l| *l > 0) {
            break;
        }

        for (i, _) in cur
            .iter()
            .enumerate()
            .filter(|(_, e)| *e.as_bytes().last().expect("Wrong input") as char == 'Z')
        {
            lengths[i] = steps;
        }

        let nodes = cur
            .iter()
            .map(|node| map.get(node).expect("Node not found"));
        if turns[i] as char == 'L' {
            cur = nodes.map(|node| node.0).collect();
        } else if turns[i] as char == 'R' {
            cur = nodes.map(|node| node.1).collect();
        } else {
            panic!("Unexpected turn");
        }

        steps += 1;
        i += 1;
        if i >= turns.len() {
            i = 0;
        }
    }

    // fuck it, then you find lcm of those in any calculator on the internet
    dbg!(lengths);
    panic!();
    steps
}

fn main() {
    let input = get_input_text().unwrap_or_else(|| {
        println!("Invalid file");
        exit(1)
    });

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn aoc_1_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(input), 6);
    }

    #[test]
    fn aoc_2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(part2(input), 6);
    }
}
