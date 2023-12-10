use advent2023::*;
use std::process::exit;

#[derive(Debug)]
struct Hand<'a> {
    hand: &'a str,
    bid: u32,
}

#[derive(PartialEq, Eq, Debug)]
enum HandType {
    Five,
    Four,
    Full,
    Three,
    TwoPairs,
    Pair,
    High,
}

impl HandType {
    fn num(&self) -> u32 {
        match self {
            Self::Five => 7,
            Self::Four => 6,
            Self::Full => 5,
            Self::Three => 4,
            Self::TwoPairs => 3,
            Self::Pair => 2,
            Self::High => 1,
        }
    }
}

impl<'a> Hand<'a> {
    fn value(&self) -> HandType {
        let mut vals = vec![0; 14];
        for ch in self.hand.chars() {
            vals[get_val(&ch, 1) as usize] += 1;
        }
        let max = vals
            .iter()
            .fold(0, |acc, x| if *x > acc { *x } else { acc });

        if max == 5 {
            HandType::Five
        } else if max == 4 {
            HandType::Four
        } else if max == 3 {
            if vals.iter().any(|v| *v == 2) {
                HandType::Full
            } else {
                HandType::Three
            }
        } else if max == 2 {
            if vals.iter().filter(|v| **v == 2).count() == 2 {
                HandType::TwoPairs
            } else {
                HandType::Pair
            }
        } else {
            HandType::High
        }
    }

    fn value_2(&self) -> HandType {
        let mut vals = vec![0; 14];
        for ch in self.hand.chars() {
            vals[get_val(&ch, 2) as usize] += 1;
        }

        let max = vals[2..]
            .iter()
            .fold(0, |acc, x| if *x > acc { *x } else { acc });

        if max + vals[1] == 5 {
            HandType::Five
        } else if max + vals[1] == 4 {
            HandType::Four
        } else if max + vals[1] == 3 {
            if vals[2..].iter().any(|v| *v == 2) {
                if vals[1] == 1 && vals[2..].iter().filter(|v| **v == 2).count() != 2 {
                    return HandType::Three;
                }

                HandType::Full
            } else {
                HandType::Three
            }
        } else if max + vals[1] == 2 {
            if vals.iter().filter(|v| **v == 2).count() == 2 {
                HandType::TwoPairs
            } else {
                HandType::Pair
            }
        } else {
            HandType::High
        }
    }
}

fn get_val(ch: &char, part: u32) -> u32 {
    if part == 1 {
        match ch {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'J' => 10,
            'T' => 9,
            '9' => 8,
            '8' => 7,
            '7' => 6,
            '6' => 5,
            '5' => 4,
            '4' => 3,
            '3' => 2,
            '2' => 1,
            _ => 0,
        }
    } else {
        match ch {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            'J' => 1,
            _ => 0,
        }
    }
}

fn cmp_hands<'a>(hand: &Hand<'a>, hand2: &Hand<'a>, part: u32) -> std::cmp::Ordering {
    let v1;
    let v2;
    if part == 1 {
        v1 = hand.value();
        v2 = hand2.value();
    } else {
        v1 = hand.value_2();
        v2 = hand2.value_2();
    }

    if v1 != v2 {
        v1.num().cmp(&v2.num())
    } else {
        for (a, b) in hand.hand.chars().zip(hand2.hand.chars()) {
            if a == b {
                continue;
            } else {
                return u32::cmp(&get_val(&a, part), &get_val(&b, part));
            }
        }

        return std::cmp::Ordering::Equal;
    }
}

fn parse_input(input: &str) -> Vec<Hand> {
    let msg = "Incorrect format";
    input
        .lines()
        .map(|l| {
            let split = l.split_once(' ').expect(msg);
            Hand {
                hand: split.0,
                bid: split.1.parse::<u32>().expect(msg),
            }
        })
        .collect()
}

fn sort_hands(mut hands: Vec<Hand>, part: u32) -> Vec<Hand> {
    hands.sort_by(|a, b| cmp_hands(a, b, part));
    hands
}

fn get_sum_of_values(hands: Vec<Hand>) -> u32 {
    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, h)| acc + h.bid * (i as u32 + 1))
}

fn part_1(input: &str) -> u32 {
    get_sum_of_values(sort_hands(parse_input(input), 1))
}

fn part_2(input: &str) -> u32 {
    get_sum_of_values(sort_hands(parse_input(input), 2))
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
    use std::cmp::Ordering;

    use super::*;

    #[test]
    fn aoc_1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(part_1(input), 6440);
    }

    #[test]
    fn aoc_2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(part_2(input), 5905);
    }

    #[test]
    fn value_2_test() {
        let hand = Hand {
            hand: "JJT85",
            bid: 0,
        };

        let hand_2 = Hand {
            hand: "JATTA",
            bid: 0,
        };

        let hand_3 = Hand {
            hand: "JATAA",
            bid: 0,
        };

        let hand_4 = Hand {
            hand: "JJJAT",
            bid: 0,
        };

        let hand_5 = Hand {
            hand: "JAA32",
            bid: 0,
        };

        assert_eq!(hand.value_2(), HandType::Three);
        assert_eq!(hand_2.value_2(), HandType::Full);
        assert_eq!(hand_3.value_2(), HandType::Four);
        assert_eq!(hand_4.value_2(), HandType::Four);
        assert_eq!(hand_5.value_2(), HandType::Three);
    }

    #[test]
    fn cmp_test() {
        let hand1 = Hand {
            hand: "JKKK2",
            bid: 0,
        };

        let hand2 = Hand {
            hand: "22224",
            bid: 0,
        };

        assert_eq!(cmp_hands(&hand1, &hand2, 2), Ordering::Less);
    }
}
