use std::{env, fs};

fn get_number(line: &str) -> u32 {
    let spelled_numbers = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
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
        panic!("WTF?");
    } else if number == 0 {
        number = numberr;
    } else if numberr == 0 {
        numberr = number;
    }

    10 * number + numberr
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = args.get(1).expect("Enter file name");
    let file = fs::read_to_string(file_name).unwrap();
    let mut total = 0;
    for line in file.split("\n") {
        if line.len() == 0 {
            break;
        }

        total += get_number(line);
    }

    println!("{}", total);
}
