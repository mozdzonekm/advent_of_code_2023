use std::{
    borrow::BorrowMut,
    collections::{hash_map::Entry, HashMap, VecDeque},
};

use itertools::{all, Itertools};

advent_of_code::solution!(12);

fn parse(input: &str) -> Vec<(Vec<char>, Vec<u32>)> {
    input
        .lines()
        .map(|l| {
            let (parts, numbers) = l.split_once(' ').unwrap();
            let parsed_numbers: Vec<u32> = numbers
                .split(',')
                .filter_map(|p| p.parse().ok())
                .collect_vec();
            (parts.chars().collect_vec(), parsed_numbers)
        })
        .collect_vec()
}

fn solve(
    parts: &mut VecDeque<char>,
    numbers: &mut VecDeque<u32>,
    mem: &mut HashMap<(VecDeque<char>, VecDeque<u32>), usize>,
) -> usize {
    match mem.entry((parts.clone(), numbers.clone())) {
        Entry::Occupied(result) => *result.get(),
        Entry::Vacant(_) => {
            let c = parts.pop_front();
            let solutions = match c {
                Some('#') => match numbers.pop_front() {
                    Some(n) => solve_after_placing_broken_part(n, parts, numbers, mem),
                    None => 0,
                },
                Some('?') => {
                    solve(parts, numbers, mem)
                        + match numbers.pop_front() {
                            Some(n) => solve_after_placing_broken_part(n, parts, numbers, mem),
                            None => 0,
                        }
                }
                Some('.') => solve(parts, numbers, mem),
                None => {
                    if numbers.is_empty() {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("Unknown character"),
            };
            if let Some(used_part) = c {
                parts.push_front(used_part);
            }
            mem.insert((parts.clone(), numbers.clone()), solutions);
            solutions
        }
    }
}

fn solve_after_placing_broken_part(
    n: u32,
    parts: &mut VecDeque<char>,
    numbers: &mut VecDeque<u32>,
    mem: &mut HashMap<(VecDeque<char>, VecDeque<u32>), usize>,
) -> usize {
    let parts_to_use = (n - 1) as usize;
    let mut sub_solutions = 0;
    if parts.len() >= parts_to_use {
        let mut remaining: VecDeque<char> = parts.split_off(parts_to_use);
        if all(parts.iter(), |c| *c == '#' || *c == '?') {
            // next char has to be .
            let c = remaining.pop_front();
            sub_solutions += match c {
                Some('.') | Some('?') => solve(remaining.borrow_mut(), numbers, mem),
                Some('#') => 0,
                None => {
                    if numbers.is_empty() {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("Unknown character"),
            };
            if let Some(used_part) = c {
                remaining.push_front(used_part);
            }
        }
        parts.append(remaining.borrow_mut());
    }
    numbers.push_front(n);
    sub_solutions
}

pub fn part_one(input: &str) -> Option<usize> {
    let rows = parse(input);
    let result: usize = rows
        .iter()
        .map(|(parts, numbers)| {
            let mut parts_dq: VecDeque<char> = parts.iter().cloned().collect::<VecDeque<char>>();
            let mut numbers_dq: VecDeque<u32> = numbers.iter().cloned().collect();
            let mut mem: HashMap<(VecDeque<char>, VecDeque<u32>), usize> = HashMap::new();
            solve(
                parts_dq.borrow_mut(),
                numbers_dq.borrow_mut(),
                mem.borrow_mut(),
            )
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let rows = parse(input);
    let result: usize = rows
        .iter()
        .map(|(parts, numbers)| {
            let mut m_parts = parts.clone();
            m_parts.push('?');
            let mut parts_multiplied = m_parts.repeat(5);
            parts_multiplied.pop();
            let numbers_multiplied = numbers.repeat(5);
            let mut parts_dq: VecDeque<char> =
                parts_multiplied.iter().cloned().collect::<VecDeque<char>>();
            let mut numbers_dq: VecDeque<u32> = numbers_multiplied.iter().cloned().collect();
            let mut mem: HashMap<(VecDeque<char>, VecDeque<u32>), usize> = HashMap::new();
            solve(
                parts_dq.borrow_mut(),
                numbers_dq.borrow_mut(),
                mem.borrow_mut(),
            )
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }

    #[test]
    fn test_solve_simple() {
        let mut parts: VecDeque<char> = "???".chars().collect();
        let mut numbers: VecDeque<u32> = [1].into_iter().collect();
        let mut mem: HashMap<(VecDeque<char>, VecDeque<u32>), usize> = HashMap::new();
        let result = solve(parts.borrow_mut(), numbers.borrow_mut(), mem.borrow_mut());
        assert_eq!(result, 3);
    }

    #[test]
    fn test_solve_medium() {
        let mut parts: VecDeque<char> = "?###????????".chars().collect();
        let mut numbers: VecDeque<u32> = [3, 2, 1].into_iter().collect();
        let mut mem: HashMap<(VecDeque<char>, VecDeque<u32>), usize> = HashMap::new();
        let result = solve(parts.borrow_mut(), numbers.borrow_mut(), mem.borrow_mut());
        assert_eq!(result, 10);
    }

    #[test]
    fn test_solve_complex() {
        let mut parts: VecDeque<char> = "????.######..#####.".chars().collect();
        let mut numbers: VecDeque<u32> = [1, 6, 5].into_iter().collect();
        let mut mem: HashMap<(VecDeque<char>, VecDeque<u32>), usize> = HashMap::new();
        let result = solve(parts.borrow_mut(), numbers.borrow_mut(), mem.borrow_mut());
        assert_eq!(result, 4);
    }
}
