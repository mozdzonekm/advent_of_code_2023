use std::cmp::Ordering;

use itertools::Itertools;

use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, newline, u64};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

advent_of_code::solution!(7);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Joker = 1,
}

fn parse_cards(card_chars: &str, joker_instead_of_jack: bool) -> Vec<Card> {
    card_chars
        .chars()
        .map(|c| match c {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' if joker_instead_of_jack => Card::Joker,
            'J' if !joker_instead_of_jack => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("Unknown Card"),
        })
        .collect::<Vec<Card>>()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    Five = 7,
    Four = 6,
    Full = 5,
    Three = 4,
    TwoPairs = 3,
    Pair = 2,
    HighCard = 1,
}

fn get_hand_type(largest_group_size: Option<u32>, second_group_size: Option<u32>) -> HandType {
    match (largest_group_size, second_group_size) {
        (Some(5), _) => HandType::Five,
        (Some(4), _) => HandType::Four,
        (Some(3), Some(2)) => HandType::Full,
        (Some(3), _) => HandType::Three,
        (Some(2), Some(2)) => HandType::TwoPairs,
        (Some(2), _) => HandType::Pair,
        (Some(1), _) => HandType::HighCard,
        _ => panic!("Unknown HandType"),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
}

impl Hand {
    fn new(card_chars: &str, joker_instead_of_jack: bool) -> Hand {
        let cards = parse_cards(card_chars, joker_instead_of_jack);
        let jokers_count: u32 = cards
            .iter()
            .filter(|&c| joker_instead_of_jack && c.cmp(&Card::Joker) == Ordering::Equal)
            .count() as u32;
        let two_largest_groups = cards
            .iter()
            .filter(|&c| !(joker_instead_of_jack && c.cmp(&Card::Joker) == Ordering::Equal))
            .into_group_map_by(|&c| c.clone())
            .values()
            .map(|v| v.len() as u32)
            .sorted()
            .rev()
            .take(2)
            .collect_vec();
        let hand_type = get_hand_type(
            Some(two_largest_groups.first().unwrap_or(&0) + jokers_count),
            two_largest_groups.get(1).cloned(),
        );

        Hand { cards, hand_type }
    }

    fn cmp(&self, other: &Hand) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                let cards_pairs = self.cards.iter().zip(&other.cards);
                for (c1, c2) in cards_pairs {
                    let order = c1.cmp(c2);
                    if order != Ordering::Equal {
                        return order;
                    }
                }
                Ordering::Equal
            }
        }
    }
}

#[derive(Debug)]
struct Position {
    hand: Hand,
    bid: u64,
}

fn parse_position(joker_instead_of_jack: bool) -> impl Fn(&str) -> IResult<&str, Position> {
    move |input: &str| {
        let (input, (card_chars, bid)) = separated_pair(alphanumeric1, tag(" "), u64)(input)?;
        Ok((
            input,
            Position {
                hand: Hand::new(card_chars, joker_instead_of_jack),
                bid,
            },
        ))
    }
}

fn parse_input(input: &str, joker_instead_of_jack: bool) -> IResult<&str, Vec<Position>> {
    let (input, positions) =
        separated_list1(newline, parse_position(joker_instead_of_jack))(input)?;
    Ok((input, positions))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, positions) = parse_input(input, false).unwrap();
    let result: u64 = positions
        .iter()
        .sorted_by(|p1, p2| p1.hand.cmp(&p2.hand))
        .enumerate()
        .map(|(i, p)| (i + 1) as u64 * p.bid)
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, positions) = parse_input(input, true).unwrap();
    let result: u64 = positions
        .iter()
        .sorted_by(|p1, p2| p1.hand.cmp(&p2.hand))
        .enumerate()
        .map(|(i, p)| (i + 1) as u64 * p.bid)
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
