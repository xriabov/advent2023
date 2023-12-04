use advent2023::*;

fn get_number(line: &str, part: u32) -> u32 {
    let mut spelled_numbers = vec![
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    if part == 2 {
        spelled_numbers.append(&mut vec![
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ])
    }

    let mut i_min = 10000;
    let mut i_max = 0;
    let mut number = 0;
    let mut numberr = 0;
    for (num, val) in spelled_numbers {
        let i1 = line.find(num).unwrap_or(10000);
        let i2 = line.rfind(num).unwrap_or(0);
        if i1 < i_min {
            i_min = i1;
            number = val;
        }
        if i2 > i_max {
            i_max = i2;
            numberr = val;
        }
    }
    if number == 0 && numberr == 0 {
        panic!("Incorrect line in input");
    } else if number == 0 {
        number = numberr;
    } else if numberr == 0 {
        numberr = number;
    }

    10 * number + numberr
}

fn calibration_number(input: &str, part: u32) -> u32 {
    let mut total = 0;
    for line in input.split("\n") {
        if line.len() == 0 {
            break;
        }

        total += get_number(line, part);
    }

    total
}

fn main() {
    let file = get_input_text().unwrap_or_else(|| {
        println!("Invalid input file");
        std::process::exit(1);
    });

    let total = calibration_number(&file, 2);

    println!("{}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(calibration_number(input, 1), 142);
    }

    #[test]
    fn aoc_2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(calibration_number(input, 2), 281);
    }
}
