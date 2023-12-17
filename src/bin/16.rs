use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(16);

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct LightRay {
    position: (isize, isize),
    direction: Direction,
}

impl LightRay {
    fn move_one_tile(&self, board: &Board) -> Vec<LightRay> {
        let next_position = self.next_position();
        match board.position_on_board(next_position) {
            Some(position) => match board.fields[position.0 as usize][position.1 as usize] {
                '.' => vec![LightRay {
                    position,
                    direction: self.direction,
                }],
                '/' => match self.direction {
                    Direction::Up => vec![LightRay {
                        position,
                        direction: Direction::Right,
                    }],
                    Direction::Right => vec![LightRay {
                        position,
                        direction: Direction::Up,
                    }],
                    Direction::Down => vec![LightRay {
                        position,
                        direction: Direction::Left,
                    }],
                    Direction::Left => vec![LightRay {
                        position,
                        direction: Direction::Down,
                    }],
                },
                '\\' => match self.direction {
                    Direction::Up => vec![LightRay {
                        position,
                        direction: Direction::Left,
                    }],
                    Direction::Right => vec![LightRay {
                        position,
                        direction: Direction::Down,
                    }],
                    Direction::Down => vec![LightRay {
                        position,
                        direction: Direction::Right,
                    }],
                    Direction::Left => vec![LightRay {
                        position,
                        direction: Direction::Up,
                    }],
                },
                '|' => match self.direction {
                    Direction::Up | Direction::Down => vec![LightRay {
                        position,
                        direction: self.direction,
                    }],
                    Direction::Right | Direction::Left => vec![
                        LightRay {
                            position,
                            direction: Direction::Up,
                        },
                        LightRay {
                            position,
                            direction: Direction::Down,
                        },
                    ],
                },
                '-' => match self.direction {
                    Direction::Left | Direction::Right => vec![LightRay {
                        position,
                        direction: self.direction,
                    }],
                    Direction::Up | Direction::Down => vec![
                        LightRay {
                            position,
                            direction: Direction::Left,
                        },
                        LightRay {
                            position,
                            direction: Direction::Right,
                        },
                    ],
                },
                _ => panic!("Unknown tile"),
            },
            None => vec![],
        }
    }

    fn next_position(&self) -> (isize, isize) {
        let (y, x) = self.position;
        match self.direction {
            Direction::Up => (y - 1, x),
            Direction::Right => (y, x + 1),
            Direction::Down => (y + 1, x),
            Direction::Left => (y, x - 1),
        }
    }
}

#[derive(Debug)]
struct Board {
    fields: Vec<Vec<char>>,
}

impl Board {
    fn calculate_enlightment(&self, start_arrays: &[LightRay]) -> HashSet<(isize, isize)> {
        let mut enlightment: HashSet<LightRay> = HashSet::new();
        let rays_to_process: &mut Vec<LightRay> = &mut start_arrays.to_vec();
        while let Some(ray) = rays_to_process.pop() {
            if !enlightment.contains(&ray) {
                enlightment.insert(ray);
                let next_rays = ray.move_one_tile(self);
                rays_to_process.append(&mut next_rays.to_vec());
            }
        }
        enlightment
            .iter()
            .map(|ray| ray.position)
            .collect::<HashSet<(isize, isize)>>()
    }

    fn position_on_board(&self, pos: (isize, isize)) -> Option<(isize, isize)> {
        let (dim_y, dim_x) = (self.fields.len(), self.fields.first().unwrap().len());
        let (y, x) = pos;
        if x >= 0 && x < dim_x as isize && y >= 0 && y < dim_y as isize {
            Some((y, x))
        } else {
            None
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let fields = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let board = Board { fields };
    let enlightment = board.calculate_enlightment(&[LightRay {
        position: (0, -1),
        direction: Direction::Right,
    }]);
    Some((enlightment.len() as isize - 1) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let fields = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let board = Board { fields };
    let (dim_y, dim_x) = (board.fields.len(), board.fields.first().unwrap().len());
    let top_max = (0..dim_x as isize)
        .map(|x| {
            let enlightment = board.calculate_enlightment(&[LightRay {
                position: (-1, x),
                direction: Direction::Down,
            }]);
            enlightment.len() as isize - 1
        })
        .max()
        .unwrap();
    let down_max = (0..dim_x as isize)
        .map(|x| {
            let enlightment = board.calculate_enlightment(&[LightRay {
                position: (dim_y as isize, x),
                direction: Direction::Up,
            }]);
            enlightment.len() as isize - 1
        })
        .max()
        .unwrap();
    let left_max = (0..dim_y as isize)
        .map(|y| {
            let enlightment = board.calculate_enlightment(&[LightRay {
                position: (y, -1),
                direction: Direction::Right,
            }]);
            enlightment.len() as isize - 1
        })
        .max()
        .unwrap();
    let right_max = (0..dim_y as isize)
        .map(|y| {
            let enlightment = board.calculate_enlightment(&[LightRay {
                position: (y, dim_x as isize),
                direction: Direction::Left,
            }]);
            enlightment.len() as isize - 1
        })
        .max()
        .unwrap();
    Some(
        (*[top_max, down_max, left_max, right_max]
            .iter()
            .max()
            .unwrap()) as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
