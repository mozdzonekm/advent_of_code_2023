use std::iter::zip;

use nom::{
    bytes::complete::tag,
    character::complete::{multispace1, newline, u64},
    multi::separated_list1,
    IResult,
};

advent_of_code::solution!(6);

#[derive(Debug)]
struct Game {
    time: u64,
    distance: u64,
}

impl Game {
    fn get_number_of_possible_wins(&self) -> u32 {
        let delta: i64 = (self.time as i64) * (self.time as i64) - 4 * self.distance as i64;
        if delta >= 0 {
            let t1 = (self.time as f64 - (delta as f64).sqrt()) / 2.0;
            let t2 = (self.time as f64 + (delta as f64).sqrt()) / 2.0;
            // if the roots are integers they do not count as possible wins as a distance would be
            // the same as record. We need to substract two.
            if t2.ceil() == t2.floor() {
                (t2 - t1 - 1.0) as u32
            } else {
                (t2.ceil() - t1.ceil()) as u32
            }
        } else {
            0
        }
    }
}

fn parse_number_list(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = multispace1(input)?;
    let (input, numbers) = separated_list1(multispace1, u64)(input)?;
    Ok((input, numbers))
}

fn parse_input_part_one(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, _) = tag("Time:")(input)?;
    let (input, times) = parse_number_list(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = tag("Distance:")(input)?;
    let (input, distances) = parse_number_list(input)?;
    let games: Vec<Game> = zip(times, distances)
        .map(|(time, distance)| Game { time, distance })
        .collect();
    Ok((input, games))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, games) = parse_input_part_one(input).unwrap();
    let result: u32 = games
        .iter()
        .map(|g| g.get_number_of_possible_wins())
        .reduce(|a, b| a * b)
        .unwrap();
    Some(result)
}

fn concatenate_numbers(numbers: &[u64]) -> u64 {
    let mut tmp = String::from("");
    tmp.extend(numbers.iter().map(|n| n.to_string()));
    tmp.parse().unwrap()
}

fn parse_input_part_two(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Time:")(input)?;
    let (input, times) = parse_number_list(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = tag("Distance:")(input)?;
    let (input, distances) = parse_number_list(input)?;
    Ok((
        input,
        Game {
            time: concatenate_numbers(&times),
            distance: concatenate_numbers(&distances),
        },
    ))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, game) = parse_input_part_two(input).unwrap();
    Some(game.get_number_of_possible_wins())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }

    #[test]
    fn test_get_number_of_possible_wins_even() {
        let game = Game {
            time: 7,
            distance: 9,
        };
        let result = game.get_number_of_possible_wins();
        assert_eq!(result, 4)
    }

    #[test]
    fn test_get_number_of_possible_wins_odd() {
        let game = Game {
            time: 30,
            distance: 200,
        };
        let result = game.get_number_of_possible_wins();
        assert_eq!(result, 9)
    }

    #[test]
    fn test_get_number_of_possible_wins_other() {
        let game = Game {
            time: 15,
            distance: 40,
        };
        let result = game.get_number_of_possible_wins();
        assert_eq!(result, 8)
    }
}
