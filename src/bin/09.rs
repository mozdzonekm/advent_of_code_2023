use itertools::Itertools;

advent_of_code::solution!(9);

fn get_next_sequence(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec()
}

fn get_sequences(numbers: &[i32]) -> Vec<Vec<i32>> {
    let mut sequences: Vec<Vec<i32>> = Vec::new();
    sequences.push(numbers.to_vec().clone());
    let mut current_sequence = numbers.to_vec().clone();
    while !current_sequence.iter().all_equal() {
        current_sequence = get_next_sequence(&current_sequence);
        sequences.push(current_sequence.clone());
    }
    sequences
}

fn predict_next(numbers: &[i32]) -> i32 {
    let sequences = get_sequences(numbers);
    sequences
        .iter()
        .map(|s| s.iter().last().unwrap_or(&0))
        .sum::<i32>()
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect()
}

pub fn part_one(input: &str) -> Option<i32> {
    let result: i32 = parse_input(input)
        .iter()
        .map(|x| predict_next(x.as_slice()))
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let result: i32 = parse_input(input)
        .iter()
        .map(|x| predict_next(x.iter().rev().cloned().collect_vec().as_slice()))
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_get_next_sequence() {
        let result = get_next_sequence(&[10, 13, 16, 21, 30, 45, 68]);
        assert_eq!(result, vec![3, 3, 5, 9, 15, 23]);
    }
}
