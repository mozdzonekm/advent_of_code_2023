use std::collections::VecDeque;

use itertools::Itertools;

advent_of_code::solution!(14);

fn to_columns(input: &str) -> Vec<Vec<char>> {
    let chars_with_indices: Vec<Vec<(usize, char)>> = input
        .lines()
        .map(|l| l.char_indices().collect_vec())
        .collect_vec();
    (0..chars_with_indices.first().unwrap().len())
        .map(|i| {
            chars_with_indices
                .iter()
                .flat_map(|l| l.iter().filter(|&(pos, _)| *pos == i).map(|&(_, c)| c))
                .collect()
        })
        .collect_vec()
}

fn calculate_column_value(col: &Vec<char>) -> u32 {
    let mut hases_positions: VecDeque<isize> = col
        .iter()
        .enumerate()
        .filter(|&(_, c)| *c == '#')
        .map(|(i, _)| i as isize)
        .collect();
    hases_positions.push_front(-1);
    hases_positions.push_back(col.len() as isize);
    hases_positions
        .iter()
        .tuple_windows()
        .map(|(low, high)| {
            let stones_count_in_segment = col
                .iter()
                .enumerate()
                .filter(|&(i, c)| i as isize > *low && (i as isize) < *high && *c == 'O')
                .count() as u32;

            (0..stones_count_in_segment)
                .map(|i| (col.len() as isize - low - 1 - i as isize) as u32)
                .sum::<u32>()
        })
        .sum::<u32>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let columns = to_columns(input);
    let result = columns.iter().map(calculate_column_value).sum();

    Some(result)
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_calculate_column_value() {
        let column = vec!['O', 'O', '.', 'O', '.', 'O', '.', '.', '#', '#'];
        let result = calculate_column_value(&column);
        assert_eq!(result, 34);
    }
}
