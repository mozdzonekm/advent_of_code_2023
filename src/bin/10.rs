use itertools::{enumerate, Itertools};
use petgraph::{adj::NodeIndex, algo::dijkstra, graph::UnGraph};

advent_of_code::solution!(10);

#[derive(Debug)]
struct Board {
    dim_x: usize,
    graph: UnGraph<usize, ()>,
    starting_position: (usize, usize),
}

fn up(x: usize, y: usize, dim_x: usize) -> usize {
    2 * dim_x * y + x
}

fn left(x: usize, y: usize, dim_x: usize) -> usize {
    2 * dim_x * y + dim_x + x
}

fn right(x: usize, y: usize, dim_x: usize) -> usize {
    2 * dim_x * y + dim_x + x + 1
}

fn down(x: usize, y: usize, dim_x: usize) -> usize {
    2 * dim_x * (y + 1) + x
}

impl Board {
    fn from_str(input: &str) -> Board {
        let lines: Vec<&str> = input.lines().collect();
        let dim_x = lines.first().unwrap().len() + 1;
        let edges = enumerate(&lines)
            .flat_map(move |(y, line)| {
                enumerate(line.chars()).filter_map(move |(x, c)| match c {
                    '-' => Some((left(x, y, dim_x), right(x, y, dim_x))),
                    '|' => Some((up(x, y, dim_x), down(x, y, dim_x))),
                    'L' => Some((up(x, y, dim_x), right(x, y, dim_x))),
                    'J' => Some((up(x, y, dim_x), left(x, y, dim_x))),
                    '7' => Some((down(x, y, dim_x), left(x, y, dim_x))),
                    'F' => Some((down(x, y, dim_x), right(x, y, dim_x))),
                    '.' => None,
                    'S' => None,
                    _ => panic!("Unknown character: {c}"),
                })
            })
            .map(|(u, v)| (u as u32, v as u32))
            .collect::<Vec<(u32, u32)>>();
        let starting_position = *enumerate(lines)
            .flat_map(|(y, line)| {
                enumerate(line.chars()).filter_map(move |(x, c)| match c {
                    'S' => Some((x, y)),
                    _ => None,
                })
            })
            .collect::<Vec<(usize, usize)>>()
            .first()
            .unwrap();
        Board {
            dim_x,
            graph: UnGraph::<usize, ()>::from_edges(edges.as_slice()),
            starting_position,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let board = Board::from_str(input);
    let (start_x, start_y) = board.starting_position;
    let possible_starts =
        [up, down, left, right].map(|f| NodeIndex::from(f(start_x, start_y, board.dim_x) as u32));
    let max_distance: f32 = possible_starts
        .iter()
        .map(|&start| {
            let possible_ends = possible_starts
                .iter()
                .filter(|&&v| v != start)
                .collect_vec();
            *dijkstra(&board.graph, start, None, |_| 1)
                .iter()
                .filter(|(k, _)| possible_ends.contains(k))
                .map(|(_, v)| v)
                .max()
                .unwrap_or(&0)
        })
        .sorted()
        .last()
        .unwrap() as f32;
    let result = (max_distance / 2_f32).ceil() as u32;
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
