advent_of_code::solution!(1);

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

fn starts_with_digit(input: &str) -> Option<u32> {
    if input.starts_with('1') || input.starts_with("one") {
        Some(1)
    } else if input.starts_with('2') || input.starts_with("two") {
        Some(2)
    } else if input.starts_with('3') || input.starts_with("three") {
        Some(3)
    } else if input.starts_with('4') || input.starts_with("four") {
        Some(4)
    } else if input.starts_with('5') || input.starts_with("five") {
        Some(5)
    } else if input.starts_with('6') || input.starts_with("six") {
        Some(6)
    } else if input.starts_with('7') || input.starts_with("seven") {
        Some(7)
    } else if input.starts_with('8') || input.starts_with("eight") {
        Some(8)
    } else if input.starts_with('9') || input.starts_with("nine") {
        Some(9)
    } else {
        None
    }
}

fn get_first_digit(line: &str, check_order: impl Iterator<Item = usize>) -> Option<u32> {
    for i in check_order {
        let digit = starts_with_digit(&line[i..]);
        if digit.is_some() {
            return digit;
        }
    }
    dbg!(&line);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let first_digit = get_first_digit(line, 0..line.len());
                let last_digit = get_first_digit(line, (0..line.len()).rev());
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
