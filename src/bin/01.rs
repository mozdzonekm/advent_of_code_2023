advent_of_code::solution!(1);

use regex::{Captures, Regex};

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let numbers = line
                    .chars()
                    .filter_map(|c| c.to_digit(10))
                    .collect::<Vec<u32>>();
                10 * numbers.first().unwrap() + numbers.last().unwrap()
            })
            .sum(),
    )
}

fn get_first_digit(line: &str, re: &Regex) -> Option<u32> {
    re.replace_all(line, |cap: &Captures| match &cap[0] {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => panic!("Unknown capture!"),
    })
    .chars()
    .filter_map(|c| c.to_digit(10))
    .collect::<Vec<u32>>()
    .first()
    .copied()
}

fn get_last_digit(line: &str, re_reversed: &Regex) -> Option<u32> {
    re_reversed
        .replace_all(
            line.chars().rev().collect::<String>().as_str(),
            |cap: &Captures| match &cap[0] {
                "eno" => "1",
                "owt" => "2",
                "eerht" => "3",
                "ruof" => "4",
                "evif" => "5",
                "xis" => "6",
                "neves" => "7",
                "thgie" => "8",
                "enin" => "9",
                _ => panic!("Unknown capture!"),
            },
        )
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>()
        .first()
        .copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new("(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let re_reversed = Regex::new("(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
    Some(
        input
            .lines()
            .map(|line| {
                let first_digit = get_first_digit(line, &re);
                let last_digit = get_last_digit(line, &re_reversed);
                10 * first_digit.unwrap() + last_digit.unwrap()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples_b", DAY));
        assert_eq!(result, Some(88));
    }
}
