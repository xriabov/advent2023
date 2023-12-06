use std::process::exit;

use advent2023::*;

fn traverse_single(input: &str) -> Vec<u64> {
    let mut lines = input.lines();

    let mut source: Vec<u64> = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    lines.next();
    lines.next();
    let mut new_source = vec![0; source.len()];
    while let Some(l) = lines.next() {
        if l.is_empty() {
            lines.next();
            source
                .iter_mut()
                .zip(new_source.iter())
                .for_each(|(source, target)| {
                    if *target != 0 {
                        *source = *target;
                    }
                });
            new_source = vec![0; source.len()];
            continue;
        }

        let nums: Vec<u64> = l
            .split_whitespace()
            .map(|num| num.parse::<u64>().unwrap())
            .collect();

        for (i, s) in source.iter().enumerate() {
            if s >= &nums[1] && s <= &(&nums[1] + &nums[2]) {
                new_source[i] = nums[0] + s - nums[1];
            }
        }
    }

    source
}

#[derive(Debug)]
struct Range {
    start: u64,
    len: u64,
}

#[derive(Debug)]
enum DifferenceOutput {
    Zero,
    Part(Range),
    Contains((Range, Range)),
    NoIntersection,
}

impl Range {
    /// Returns the last element of a range
    fn end(&self) -> u64 {
        self.start + self.len - 1
    }

    fn difference(&self, range: &Range) -> DifferenceOutput {
        if self.end() < range.start || self.start > range.end() {
            return DifferenceOutput::NoIntersection;
        } else if self.start < range.start && self.end() > range.end() {
            let left_part = Range {
                start: self.start,
                len: range.start + 1 - self.start,
            };

            let right_part = Range {
                start: range.end(),
                len: self.end() + 1 - range.end(),
            };
            return DifferenceOutput::Contains((left_part, right_part));
        } else if self.start >= range.start && self.end() <= range.end() {
            return DifferenceOutput::Zero;
        } else if self.start < range.start {
            let range = Range {
                start: self.start,
                len: range.start + 1 - self.start,
            };
            return DifferenceOutput::Part(range);
        } else {
            let range = Range {
                start: range.end(),
                len: self.end() - range.end() + 1,
            };
            return DifferenceOutput::Part(range);
        }
    }

    /// Assuming that ranges have intersection
    fn common(&self, range: &Range) -> Range {
        Range {
            start: u64::max(self.start, range.start),
            len: u64::min(self.end(), range.end()) - u64::max(self.start, range.start) + 1,
        }
    }

    fn map(&self, source: u64, mapped: u64) -> Range {
        Range {
            start: u64::abs_diff(self.start, source) + mapped,
            len: self.len,
        }
    }
}

fn traverse_range(input: &str) -> Vec<u64> {
    let mut lines = input.lines();

    let source: Vec<u64> = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    let mut source: Vec<Range> = (&source)
        .chunks(2)
        .map(|chunk| Range {
            start: chunk[0],
            len: chunk[1],
        })
        .collect();

    lines.next();
    lines.next();
    let mut common: Vec<Range> = vec![];
    let mut mapped: Vec<Range> = vec![];
    while let Some(l) = lines.next() {
        if l.is_empty() {
            lines.next();
            source.append(&mut mapped);
            mapped = vec![];
            continue;
        }

        let nums: Vec<u64> = l
            .split_whitespace()
            .map(|num| num.parse::<u64>().unwrap())
            .collect();

        source.retain(|s| {
            if s.end() >= nums[1] && s.start <= nums[1] + nums[2] - 1 {
                let range = Range {
                    start: nums[1],
                    len: nums[2],
                };
                match s.difference(&range) {
                    DifferenceOutput::Zero => {
                        mapped.push(s.map(nums[1], nums[0]));
                        return false;
                    }
                    DifferenceOutput::Part(diff) => {
                        common.push(diff);
                        mapped.push(s.common(&range).map(nums[1], nums[0]));
                        return false;
                    }
                    DifferenceOutput::NoIntersection => {
                        panic!("bug in detecting intersections");
                    }
                    DifferenceOutput::Contains((rleft, rright)) => {
                        common.push(rleft);
                        common.push(rright);
                        mapped.push(s.common(&range).map(nums[1], nums[0]));
                        return false;
                    }
                }
            }
            true
        });

        source.append(&mut common);
        common = vec![];
    }

    source.append(&mut mapped);
    source.iter().map(|r| r.start).collect()
}

fn lowest_location<F>(input: &str, traversal_function: F) -> u64
where
    F: Fn(&str) -> Vec<u64>,
{
    let parsed = traversal_function(input);
    let mut min = parsed[0];
    for elem in parsed.iter() {
        if elem < &min {
            min = *elem;
        }
    }

    min
}

fn main() {
    let input = get_input_text().unwrap_or_else(|| {
        println!("Invalid file");
        exit(1)
    });

    println!("{}", lowest_location(&input, traverse_single));
    println!("{}", lowest_location(&input, traverse_range));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(lowest_location(input, traverse_single), 35);
    }

    #[test]
    fn aoc_2() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(lowest_location(input, traverse_range), 46);
    }

    #[test]
    fn custom_2() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48
";
        assert_eq!(lowest_location(input, traverse_range), 57);
    }
}
