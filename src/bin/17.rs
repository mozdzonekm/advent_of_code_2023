use itertools::Itertools;
use petgraph::{algo::dijkstra, graph::DiGraph};

advent_of_code::solution!(17);

#[derive(Debug)]
struct Board {
    graph: DiGraph<u32, u32>,
    start: u32,
    target: u32,
}

fn prepare_graph(
    input: &str,
    min_steps_in_one_direction: u32,
    max_steps_in_one_direction: u32,
) -> Board {
    let city_blocks = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();
    let dim_y = city_blocks.len() as u32;
    let dim_x = city_blocks.first().unwrap().len() as u32;
    let board_size = dim_y * dim_x;
    let direction_size = max_steps_in_one_direction * board_size;
    let mut edges = (0..dim_y)
        .cartesian_product(0..dim_x)
        .flat_map(|(y, x)| {
            let mut edges_to_add: Vec<(u32, u32, u32)> = Vec::new();
            for layer in 1..max_steps_in_one_direction {
                // horizontal moves
                if x > 0 {
                    edges_to_add.push((
                        (layer - 1) * board_size + y * dim_x + (x - 1),
                        layer * board_size + y * dim_x + x,
                        city_blocks[y as usize][x as usize],
                    ));
                }
                if x + 1 < dim_x {
                    edges_to_add.push((
                        2 * direction_size + (layer - 1) * board_size + y * dim_x + (x + 1),
                        2 * direction_size + layer * board_size + y * dim_x + x,
                        city_blocks[y as usize][x as usize],
                    ));
                }
                // vertical moves
                if y > 0 {
                    edges_to_add.push((
                        direction_size + (layer - 1) * board_size + (y - 1) * dim_x + x,
                        direction_size + layer * board_size + y * dim_x + x,
                        city_blocks[y as usize][x as usize],
                    ));
                }
                if y + 1 < dim_y {
                    edges_to_add.push((
                        3 * direction_size + (layer - 1) * board_size + (y + 1) * dim_x + x,
                        3 * direction_size + layer * board_size + y * dim_x + x,
                        city_blocks[y as usize][x as usize],
                    ));
                }
            }
            // turns
            for layer in (min_steps_in_one_direction - 1)..max_steps_in_one_direction {
                if x > 0 {
                    edges_to_add.push((
                        direction_size + layer * board_size + y * dim_x + (x - 1),
                        y * dim_x + x,
                        city_blocks[y as usize][x as usize],
                    ));
                    edges_to_add.push((
                        3 * direction_size + layer * board_size + y * dim_x + (x - 1),
                        y * dim_x + x,
                        city_blocks[y as usize][x as usize],
                    ));
                }
                if x + 1 < dim_x {
                    edges_to_add.push((
                        direction_size + layer * board_size + y * dim_x + (x + 1),
                        2 * direction_size + y * dim_x + x,
                        city_blocks[y as usize][x as usize],
                    ));
                    edges_to_add.push((
                        3 * direction_size + layer * board_size + y * dim_x + (x + 1),
                        2 * direction_size + y * dim_x + x,
                        city_blocks[y as usize][x as usize],
                    ));
                }
                if y > 0 {
                    edges_to_add.push((
                        layer * board_size + (y - 1) * dim_x + x,
                        direction_size + y * dim_x + x,
                        city_blocks[y as usize][x as usize],
                    ));
                    edges_to_add.push((
                        2 * direction_size + layer * board_size + (y - 1) * dim_x + x,
                        direction_size + y * dim_x + x,
                        city_blocks[y as usize][x as usize],
                    ));
                }
                if y + 1 < dim_y {
                    edges_to_add.push((
                        layer * board_size + (y + 1) * dim_x + x,
                        3 * direction_size + y * dim_x + x,
                        city_blocks[y as usize][x as usize],
                    ));
                    edges_to_add.push((
                        2 * direction_size + layer * board_size + (y + 1) * dim_x + x,
                        3 * direction_size + y * dim_x + x,
                        city_blocks[y as usize][x as usize],
                    ));
                }
            }
            edges_to_add
        })
        .collect_vec();
    let target = 4 * direction_size;
    for layer in (min_steps_in_one_direction - 1)..max_steps_in_one_direction {
        edges.push((
            direction_size + layer * board_size + (dim_y - 1) * dim_x + dim_x - 1,
            target,
            0,
        ));
        edges.push((
            layer * board_size + (dim_y - 1) * dim_x + dim_x - 1,
            target,
            0,
        ));
    }
    Board {
        graph: DiGraph::from_edges(edges),
        start: 0,
        target,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let board = prepare_graph(input, 1, 3);
    let path = dijkstra(&board.graph, board.start.into(), None, |e| *e.weight());
    Some(path[&petgraph::graph::NodeIndex::from(board.target)])
}

pub fn part_two(input: &str) -> Option<u32> {
    let board = prepare_graph(input, 4, 10);
    let path = dijkstra(&board.graph, board.start.into(), None, |e| *e.weight());
    Some(path[&petgraph::graph::NodeIndex::from(board.target)])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
