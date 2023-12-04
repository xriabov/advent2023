use advent2023::*;

#[derive(Debug)]
struct Number {
    y: usize,
    x1: usize,
    x2: usize,
    val: u32,
}

#[derive(Debug)]
struct Symbol {
    y: usize,
    x: usize,
    is_gear: bool,
    nums: Option<Vec<u32>>,
}

fn parse_input(input: &str) -> (Vec<Number>, Vec<Symbol>) {
    let mut numbers = vec![];
    let mut symbols = vec![];
    for (i, mut line) in input.lines().map(|l| l.chars()).enumerate() {
        let mut j = 0;
        let mut num = 0;
        let mut last_start_j = 0;
        loop {
            let c = line.next();
            if num != 0 && (c.is_none() || !c.unwrap().is_digit(10)) {
                numbers.push(Number {
                    y: i,
                    x1: last_start_j,
                    x2: j - 1,
                    val: num,
                });
                num = 0;
                last_start_j = 0;
            }

            if c.is_none() {
                break;
            }

            let c = c.unwrap();

            if c.is_digit(10) {
                if num == 0 {
                    last_start_j = j;
                }
                num *= 10;
                num += c.to_digit(10).unwrap();
            } else if c != '.' {
                symbols.push(Symbol {
                    y: i,
                    x: j,
                    is_gear: c == '*',
                    nums: None,
                });
            }

            j += 1;
        }

        if num != 0 {
            numbers.push(Number {
                y: i,
                x1: last_start_j,
                x2: j - 1,
                val: num,
            });
        }
    }

    populate_numbers(&numbers, &mut symbols);

    (numbers, symbols)
}

fn populate_numbers(nums: &Vec<Number>, symbols: &mut Vec<Symbol>) {
    for mut sym in symbols {
        sym.nums = Some(
            nums.iter()
                .filter(|num| {
                    usize::abs_diff(sym.y, num.y) < 2 && sym.x + 1 >= num.x1 && sym.x <= num.x2 + 1
                })
                .map(|num| num.val)
                .collect(),
        );
    }
}

fn detail_parts_sum(nums: &Vec<Number>, symbols: &Vec<Symbol>) -> u32 {
    nums.iter()
        .filter(|num| {
            symbols.iter().any(|sym| {
                usize::abs_diff(sym.y, num.y) < 2 && sym.x + 1 >= num.x1 && sym.x <= num.x2 + 1
            })
        })
        .map(|num| num.val)
        .sum()
}

fn gear_ratio_sum(symbols: &Vec<Symbol>) -> u32 {
    symbols
        .iter()
        .filter(|sym| sym.is_gear)
        .map(|sym| sym.nums.as_ref().expect("Must be already populated"))
        .filter(|num| num.len() == 2)
        .map(|num| num[0] * num[1])
        .sum()
}

fn main() {
    let file = get_input_text().unwrap_or_else(|| {
        println!("Invalid input file");
        std::process::exit(1);
    });

    let (num, sym) = parse_input(&file);
    dbg!(&num);
    let sum = detail_parts_sum(&num, &sym);
    let gears = gear_ratio_sum(&sym);
    println!("new: {} {}", sum, gears);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let (num, sym) = parse_input(&input);
        assert_eq!(detail_parts_sum(&num, &sym), 4361);
    }

    #[test]
    fn reddit_1() {
        let input = "12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56";
        let (num, sym) = parse_input(&input);
        assert_eq!(detail_parts_sum(&num, &sym), 925);
    }

    #[test]
    fn aoc_2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let (_, sym) = parse_input(&input);
        assert_eq!(gear_ratio_sum(&sym), 467835);
    }

    #[test]
    fn reddit_2() {
        let input = "12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56";
        let (_, sym) = parse_input(&input);
        assert_eq!(gear_ratio_sum(&sym), 6756);
    }
}
