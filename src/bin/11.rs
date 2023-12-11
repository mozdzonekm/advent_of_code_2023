use std::cmp::{max, min};

use itertools::{all, enumerate, Itertools};

advent_of_code::solution!(11);

type Position = (usize, usize);

pub fn part_one(input: &str) -> Option<usize> {
    let maze: Vec<&str> = input.lines().collect();
    let galaxies: Vec<Position> = enumerate(&maze)
        .flat_map(|(i, row)| {
            row.char_indices().filter_map(move |(j, c)| match c {
                '.' => None,
                _ => Some((j, i)),
            })
        })
        .collect();
    let empty_rows: Vec<usize> = enumerate(&maze)
        .filter(|(_, row)| all(row.chars(), |c| c == '.'))
        .map(|(i, _)| i)
        .collect();
    let empty_columns: Vec<usize> = (0..maze.len())
        .filter(|j| {
            let column = &maze
                .iter()
                .map(|row| row.chars().collect_vec()[*j])
                .collect_vec();
            all(column, |&c| c == '.')
        })
        .collect();
    let result: usize = galaxies
        .into_iter()
        .combinations(2)
        .map(|combinations| {
            let (x1, y1) = combinations.first().unwrap();
            let (x2, y2) = combinations.get(1).unwrap();
            let passed_empty_rows: usize = empty_rows
                .iter()
                .filter(|&i| i > min(y1, y2) && i < max(y1, y2))
                .count();
            let passed_empty_columns: usize = empty_columns
                .iter()
                .filter(|&j| j > min(x1, x2) && j < max(x1, x2))
                .count();
            ((*x2 as i32 - *x1 as i32).abs() + (*y2 as i32 - *y1 as i32).abs()) as usize
                + passed_empty_columns
                + passed_empty_rows
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let maze: Vec<&str> = input.lines().collect();
    let galaxies: Vec<Position> = enumerate(&maze)
        .flat_map(|(i, row)| {
            row.char_indices().filter_map(move |(j, c)| match c {
                '.' => None,
                _ => Some((j, i)),
            })
        })
        .collect();
    let empty_rows: Vec<usize> = enumerate(&maze)
        .filter(|(_, row)| all(row.chars(), |c| c == '.'))
        .map(|(i, _)| i)
        .collect();
    let empty_columns: Vec<usize> = (0..maze.len())
        .filter(|j| {
            let column = &maze
                .iter()
                .map(|row| row.chars().collect_vec()[*j])
                .collect_vec();
            all(column, |&c| c == '.')
        })
        .collect();
    let result: usize = galaxies
        .into_iter()
        .combinations(2)
        .map(|combinations| {
            let (x1, y1) = combinations.first().unwrap();
            let (x2, y2) = combinations.get(1).unwrap();
            let passed_empty_rows: usize = empty_rows
                .iter()
                .filter(|&i| i > min(y1, y2) && i < max(y1, y2))
                .count();
            let passed_empty_columns: usize = empty_columns
                .iter()
                .filter(|&j| j > min(x1, x2) && j < max(x1, x2))
                .count();
            ((*x2 as i32 - *x1 as i32).abs() + (*y2 as i32 - *y1 as i32).abs()) as usize
                + (1000000 - 1) * passed_empty_columns
                + (1000000 - 1) * passed_empty_rows
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
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
