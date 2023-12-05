use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace1, newline, u64},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(5);

#[derive(Debug)]
struct MappingEntry {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

#[derive(Debug)]
struct Mapping {
    entries: Vec<MappingEntry>,
}

impl Mapping {
    fn get_destination(&self, source: u64) -> u64 {
        for e in self.entries.iter() {
            if source >= e.source_start && source < e.source_start + e.length {
                return source - e.source_start + e.destination_start;
            }
        }
        source
    }
}

fn parse_mapping_entry(input: &str) -> IResult<&str, MappingEntry> {
    let (input, entries) = separated_list1(tag(" "), u64)(input)?;
    if entries.len() == 3 {
        Ok((
            input,
            MappingEntry {
                destination_start: entries[0],
                source_start: entries[1],
                length: entries[2],
            },
        ))
    } else {
        panic!("Wrong number of entries in the mapping")
    }
}

fn parse_mapping(input: &str) -> IResult<&str, Mapping> {
    let (input, _) = alpha1(input)?;
    let (input, _) = tag("-to-")(input)?;
    let (input, _) = alpha1(input)?;
    let (input, _) = tag(" map:\n")(input)?;
    let (input, entries) = separated_list1(newline, parse_mapping_entry)(input)?;
    Ok((input, Mapping { entries }))
}
fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(multispace1, u64)(input)?;
    Ok((input, seeds))
}

fn parse_part_one(input: &str) -> IResult<&str, (Vec<u64>, Vec<Mapping>)> {
    let (input, seeds) = parse_seeds(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, mappings) = separated_list1(tag("\n\n"), parse_mapping)(input)?;
    Ok((input, (seeds, mappings)))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, (seeds, mappings)) = parse_part_one(input).unwrap();
    let result: u64 = seeds
        .iter()
        .map(|seed| {
            let mut location = *seed;
            for m in mappings.iter() {
                location = m.get_destination(location);
            }
            location
        })
        .min()
        .unwrap();
    Some(result)
}

type SeedRange = (u64, u64);

fn parse_seed_ranges(input: &str) -> IResult<&str, Vec<SeedRange>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seed_ranges) =
        separated_list1(multispace1, separated_pair(u64, tag(" "), u64))(input)?;
    Ok((input, seed_ranges))
}

fn parse_part_two(input: &str) -> IResult<&str, (Vec<SeedRange>, Vec<Mapping>)> {
    let (input, seed_ranges) = parse_seed_ranges(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, mappings) = separated_list1(tag("\n\n"), parse_mapping)(input)?;
    Ok((input, (seed_ranges, mappings)))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, (seed_ranges, mappings)) = parse_part_two(input).unwrap();
    let result = seed_ranges
        .iter()
        .map(|sr| {
            let (start, length) = *sr;
            (start..start + length)
                .map(|seed| {
                    let mut location = seed;
                    for m in mappings.iter() {
                        location = m.get_destination(location);
                    }
                    location
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
