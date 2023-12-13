use std::{
    cmp::{max, min},
    iter::zip,
};

use itertools::{all, Itertools};

advent_of_code::solution!(13);

fn find_horizontal_mirror(lines: &Vec<&str>) -> Option<usize> {
    (1..lines.len())
        .flat_map(|i| {
            let start = max(0, 2 * i as i32 - lines.len() as i32) as usize;
            let end = min(lines.len(), 2 * i);
            let left = lines[start..i].iter().collect_vec();
            let right = lines[i..end].iter().rev().collect_vec();
            if all(zip(left, right), |(a, b)| a == b) {
                Some(i)
            } else {
                None
            }
        })
        .collect_vec()
        .first()
        .copied()
}

fn to_columns(input: &str) -> Vec<String> {
    let chars_with_indices: Vec<Vec<(usize, char)>> = input
        .lines()
        .map(|l| l.char_indices().collect_vec())
        .collect_vec();
    (0..chars_with_indices.first().unwrap().len())
        .map(|i| {
            let column: String = chars_with_indices
                .iter()
                .flat_map(|l| l.iter().filter(|&(pos, _)| *pos == i).map(|&(_, c)| c))
                .collect();
            column
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<usize> {
    let result: usize = input
        .split("\n\n")
        .map(|board| {
            let rows = board.lines().collect_vec();
            let vertical = find_horizontal_mirror(&rows);
            let columns = to_columns(board);
            let horizontal =
                find_horizontal_mirror(&columns.iter().map(|s| s.as_str()).collect_vec());
            100 * vertical.unwrap_or(0) + horizontal.unwrap_or(0)
        })
        .sum();
    Some(result)
}

fn find_first_horizontal_smudge(lines: &Vec<&str>) -> Option<usize> {
    (0..lines.len())
        .flat_map(|i| {
            let start = max(0, 2 * i as i32 - lines.len() as i32) as usize;
            let end = min(lines.len(), 2 * i);
            let left = lines[start..i].iter().collect_vec();
            let right = lines[i..end].iter().rev().collect_vec();
            match zip(&left, &right)
                .map(|(a, b)| {
                    zip(a.chars(), b.chars())
                        .filter(|(c1, c2)| c1 != c2)
                        .collect_vec()
                        .len()
                })
                .filter(|&diff| diff > 0)
                .exactly_one()
            {
                Ok(1) => Some(i),
                _ => None,
            }
        })
        .collect_vec()
        .first()
        .copied()
}

pub fn part_two(input: &str) -> Option<usize> {
    let result: usize = input
        .split("\n\n")
        .map(|board| {
            let rows = board.lines().collect_vec();
            let vertical = find_first_horizontal_smudge(&rows);
            let columns = to_columns(board);
            let horizontal =
                find_first_horizontal_smudge(&columns.iter().map(|s| s.as_str()).collect_vec());
            100 * vertical.unwrap_or(0) + horizontal.unwrap_or(0)
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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
