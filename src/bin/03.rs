use std::cmp::{max, min};

advent_of_code::solution!(3);

#[derive(Debug)]
struct Part {
    code: u32,
    line: usize,
    start: usize,
    end: usize,
}

impl Part {
    fn is_connected_with_symbol(&self, board: &Vec<Vec<char>>) -> bool {
        for i in (max(self.line as isize - 1, 0) as usize)..min(self.line + 2, board.len()) {
            for j in (max(self.start as isize - 1, 0) as usize)..min(self.end + 1, board[0].len()) {
                let c = board[i][j];
                if !c.is_numeric() && c != '.' {
                    return true;
                }
            }
        }
        false
    }

    fn is_connected(&self, i: usize, j: usize) -> bool {
        i >= (max(self.line as isize - 1, 0) as usize)
            && i < self.line + 2
            && j >= (max(self.start as isize - 1, 0) as usize)
            && j < self.end + 1
    }
}

fn parse_parts(board: &[Vec<char>]) -> Vec<Part> {
    let mut parts: Vec<Part> = Vec::new();
    let mut current_code = 0;
    let mut start = 0;
    for (i, line) in board.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if c.is_numeric() {
                if current_code == 0 {
                    start = j;
                }
                current_code = 10 * current_code + c.to_digit(10).unwrap();
            } else if current_code != 0 {
                parts.push(Part {
                    code: current_code,
                    line: i,
                    start,
                    end: j,
                });
                current_code = 0;
                start = 0;
            }
        }
        if current_code != 0 {
            parts.push(Part {
                code: current_code,
                line: i,
                start,
                end: line.len() + 1,
            });
            current_code = 0;
            start = 0;
        }
    }
    parts
}

pub fn part_one(input: &str) -> Option<u32> {
    let board = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let parts = parse_parts(&board);

    let result = parts
        .iter()
        .filter(|p| p.is_connected_with_symbol(&board))
        .map(|p| p.code)
        .sum::<u32>();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let board = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let parts = parse_parts(&board);
    let mut gears: Vec<(usize, usize)> = Vec::new();
    for (i, line) in board.iter().enumerate() {
        for (j, &c) in line.iter().enumerate() {
            if c == '*' {
                gears.push((i, j));
            }
        }
    }
    let result: u32 = gears
        .iter()
        .map(|(i, j)| {
            let conneted_parts = parts
                .iter()
                .filter(|p| p.is_connected(*i, *j))
                .collect::<Vec<&Part>>();
            if conneted_parts.len() == 2 {
                conneted_parts[0].code * conneted_parts[1].code
            } else {
                0
            }
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
