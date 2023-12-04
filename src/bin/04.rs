use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{multispace1, newline, u32},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

advent_of_code::solution!(4);

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    owned_numbers: Vec<u32>,
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, id) = u32(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, (winning_numbers, owned_numbers)) = separated_pair(
        separated_list1(multispace1, u32),
        delimited(multispace1, tag("|"), multispace1),
        separated_list1(multispace1, u32),
    )(input)?;
    Ok((
        input,
        Card {
            id,
            winning_numbers,
            owned_numbers,
        },
    ))
}

fn parse_input(input: &str) -> Vec<Card> {
    let (_, cards) = separated_list1(newline, parse_card)(input).unwrap();
    cards
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards = parse_input(input);
    let result: u32 = cards
        .iter()
        .map(|c| {
            let common_numbers: Vec<_> = c
                .owned_numbers
                .iter()
                .filter(|n| c.winning_numbers.contains(n))
                .collect();
            if common_numbers.is_empty() {
                0
            } else {
                1 << (common_numbers.len() as isize - 1)
            }
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = parse_input(input);
    let card_number_of_commons: HashMap<u32, usize> = cards
        .iter()
        .map(|c| {
            let common_numbers: Vec<_> = c
                .owned_numbers
                .iter()
                .filter(|n| c.winning_numbers.contains(n))
                .collect();
            (c.id, common_numbers.len())
        })
        .collect();
    let mut cards_to_see: Vec<u32> = card_number_of_commons.keys().copied().collect();
    let mut cards_seen: Vec<u32> = Vec::new();
    while let Some(card) = cards_to_see.pop() {
        for i in card + 1..=card + card_number_of_commons[&card] as u32 {
            cards_to_see.push(i);
        }
        cards_seen.push(card);
    }
    Some(cards_seen.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
