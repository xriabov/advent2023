use advent2023::*;

fn str_to_vec(snum: &str) -> Vec<u32> {
    snum.split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

fn winning_cards(input: &str) -> Vec<usize> {
    input
        .lines()
        .filter(|l| l.len() > 1)
        .map(|line| {
            line.split_once(':')
                .unwrap_or(("", "|"))
                .1
                .split_once('|')
                .unwrap()
        })
        .map(|(win, mine)| (str_to_vec(win), str_to_vec(mine)))
        .map(|(w, m)| m.iter().filter(|n| w.iter().any(|n2| *n == n2)).count())
        .collect()
}

fn total_worth(input: &str) -> u32 {
    winning_cards(input)
        .iter()
        .filter(|count| &0 < *count)
        .map(|count| (2 as u32).pow(*count as u32 - 1))
        .sum()
}

fn count_cards(input: &str) -> u32 {
    let arr = winning_cards(input);
    let mut quantities: Vec<u32> = vec![1; dbg!(arr.len())];

    for (i, n) in arr.iter().enumerate() {
        for j in i + 1..i + n + 1 {
            quantities[j] += quantities[i];
        }
    }

    arr.iter().zip(quantities.iter()).for_each(|p| {
        println!("{} {}", p.0, p.1);
    });

    quantities.iter().sum()
}

fn main() {
    let input = get_input_text().unwrap_or_else(|| {
        println!("Invalid input file");
        std::process::exit(1);
    });

    println!("{}", total_worth(&input));
    println!("{}", count_cards(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(total_worth(&input), 13);
    }

    #[test]
    fn aoc_2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(count_cards(&input), 30);
    }

    #[test]
    fn custom_2() {
        let input = ": 1 | 1
: 1 | 1
: 1 | 0";
        assert_eq!(count_cards(&input), 6);
    }
}
