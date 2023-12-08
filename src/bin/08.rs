advent_of_code::solution!(8);

use num::integer::lcm;

use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, newline},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

type Instruction<'a> = (&'a str, (&'a str, &'a str));

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, node_id) = alphanumeric1(input)?;
    let (input, _) = tag(" = ")(input)?;
    let (input, (left, right)) = delimited(
        tag("("),
        separated_pair(alphanumeric1, tag(", "), alphanumeric1),
        tag(")"),
    )(input)?;
    Ok((input, (node_id, (left, right))))
}

fn parse_input(input: &str) -> IResult<&str, (&str, Vec<Instruction>)> {
    let (input, moves) = alpha1(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, instructions) = separated_list1(newline, parse_instruction)(input)?;
    Ok((input, (moves, instructions)))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, (moves, instructions)) = parse_input(input).unwrap();
    let instructions_map: HashMap<&str, (&str, &str)> = instructions.into_iter().collect();
    let mut move_counter = 0;
    let mut i = 0;
    let mut currend_node = "AAA";
    while currend_node != "ZZZ" {
        let direction = moves.chars().nth(i);
        let (left, right) = instructions_map[currend_node];
        match direction {
            Some('L') => currend_node = left,
            Some('R') => currend_node = right,
            _ => panic!("Unknown direction"),
        }
        i += 1;
        if i == moves.len() {
            i = 0;
        }
        move_counter += 1;
    }
    Some(move_counter)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, (moves, instructions)) = parse_input(input).unwrap();
    let instructions_map: HashMap<&str, (&str, &str)> = instructions.into_iter().collect();
    let starting_positions: Vec<&str> = instructions_map
        .keys()
        .filter(|&&node| node.ends_with('A'))
        .cloned()
        .collect();
    let min_z_distances = starting_positions
        .iter()
        .map(|&p| {
            let mut move_counter = 0;
            let mut i = 0;
            let mut currend_node = p;
            while !currend_node.ends_with('Z') {
                let direction = moves.chars().nth(i);
                let (left, right) = instructions_map[currend_node];
                match direction {
                    Some('L') => currend_node = left,
                    Some('R') => currend_node = right,
                    _ => panic!("Unknown direction"),
                }
                i += 1;
                if i == moves.len() {
                    i = 0;
                }
                move_counter += 1;
            }
            move_counter
        })
        .collect::<Vec<u64>>();
    min_z_distances.into_iter().reduce(lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples_b", DAY));
        assert_eq!(result, Some(6));
    }
}
