use itertools::Itertools;

advent_of_code::solution!(9);

fn get_next_sequence(numbers: Vec<i32>) -> Vec<i32> {
    numbers
        .iter()
        .zip(numbers.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect_vec()
}

fn get_sequences(numbers: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sequences: Vec<Vec<i32>> = Vec::new();
    sequences.push(numbers.clone());
    let mut current_sequence = numbers;
    while !current_sequence.iter().all_equal() {
        current_sequence = get_next_sequence(current_sequence);
        sequences.push(current_sequence.clone());
    }
    sequences
}

pub fn part_one(input: &str) -> Option<i32> {
    let result = input
        .lines()
        .map(|l| {
            let numbers: Vec<i32> = l.split_whitespace().map(|n| n.parse().unwrap()).collect();
            let sequences = get_sequences(numbers);
            sequences
                .iter()
                .map(|s| s.iter().last().unwrap_or(&0))
                .sum::<i32>()
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let result = input
        .lines()
        .map(|l| {
            let numbers: Vec<i32> = l.split_whitespace().map(|n| n.parse().unwrap()).collect();
            let sequences = get_sequences(numbers);
            sequences
                .iter()
                .map(|s| s.first().unwrap_or(&0))
                .rev()
                .cloned()
                .reduce(|a, b| b - a)
                .unwrap_or(0)
        })
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
        let result = get_next_sequence(vec![10, 13, 16, 21, 30, 45, 68]);
        assert_eq!(result, vec![3, 3, 5, 9, 15, 23]);
    }
}
