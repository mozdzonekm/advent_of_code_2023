use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{alpha1, char, u32};
use nom::multi::{many0, separated_list1};
use nom::IResult;

advent_of_code::solution!(15);

fn hash(instruction: &str) -> u32 {
    instruction
        .chars()
        .map(|c| c as u32)
        .fold(0, |acc, b| ((acc + b) * 17) % 256)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.split(',').map(hash).sum())
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: u32,
}

#[derive(Debug)]
enum Operation {
    Remove,
    Update { focal_length: u32 },
}

#[derive(Debug)]
struct Step {
    label: String,
    operation: Operation,
}

fn parse_step(input: &str) -> IResult<&str, Step> {
    let (input, label) = alpha1(input).map(|(i, l)| (i, l.to_string()))?;
    let (input, sign) = alt((char('-'), char('=')))(input)?;
    let (input, numbers) = many0(u32)(input)?;
    let operation = match sign {
        '-' => Operation::Remove,
        '=' => Operation::Update {
            focal_length: *numbers.first().unwrap(),
        },
        _ => panic!("Unknown char"),
    };
    Ok((input, Step { label, operation }))
}

fn parse(input: &str) -> IResult<&str, Vec<Step>> {
    let (input, steps) = separated_list1(char(','), parse_step)(input)?;
    Ok((input, steps))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, steps) = parse(input).unwrap();
    let mut buckets: Vec<Vec<Lens>> = (0..256).map(|_| Vec::new()).collect_vec();
    for step in steps {
        let bucket: &mut Vec<Lens> = buckets[hash(step.label.as_str()) as usize].as_mut();
        let i = &bucket
            .iter()
            .find_position(|other| other.label == step.label);
        match step.operation {
            Operation::Remove => {
                if let Some((index, _)) = i {
                    let _ = bucket.remove(*index);
                }
            }
            Operation::Update { focal_length } => match i {
                Some((index, _)) => {
                    bucket[*index] = Lens {
                        label: step.label,
                        focal_length,
                    }
                }
                None => bucket.push(Lens {
                    label: step.label,
                    focal_length,
                }),
            },
        };
    }
    let result: u32 = buckets
        .iter()
        .enumerate()
        .map(|(i, bucket)| {
            (i + 1) as u32
                * bucket
                    .iter()
                    .enumerate()
                    .map(|(j, lens)| (j + 1) as u32 * lens.focal_length)
                    .sum::<u32>()
        })
        .sum::<u32>();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }

    #[test]
    fn test_hash() {
        let result = hash("HASH");
        assert_eq!(result, 52);
    }
}
