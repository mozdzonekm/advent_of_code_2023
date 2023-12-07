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

pub fn part_one(input: &str) -> Option<u32> {
    let (_, cards) = separated_list1(newline, parse_card)(input).unwrap();
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
                (2_u32).pow(common_numbers.len() as u32 - 1)
            }
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, cards) = separated_list1(newline, parse_card)(input).unwrap();
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
    let mut number_of_cards: HashMap<u32, u32> = card_number_of_commons
        .keys()
        .map(|&card_id| (card_id, 1_u32))
        .collect();
    for card_id in cards.iter().map(|c| c.id) {
        for i in card_id + 1..=card_id + card_number_of_commons[&card_id] as u32 {
            *number_of_cards.entry(i).or_insert(1) += number_of_cards[&card_id];
        }
    }
    Some(number_of_cards.values().sum())
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
