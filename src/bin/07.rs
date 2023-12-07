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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
}

impl Hand {
    fn new(card_chars: &str) -> Hand {
        let cards: Vec<Card> = card_chars
            .chars()
            .map(|c| match c {
                'A' => Card::Ace,
                'K' => Card::King,
                'Q' => Card::Queen,
                'J' => Card::Jack,
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
            .collect();
        let two_largest_groups = cards
            .iter()
            .into_group_map_by(|&c| c.clone())
            .values()
            .map(|v| v.len() as u32)
            .sorted()
            .rev()
            .take(2)
            .collect_vec();
        let hand_type = match (two_largest_groups.first(), two_largest_groups.get(1)) {
            (Some(5), _) => HandType::Five,
            (Some(4), _) => HandType::Four,
            (Some(3), Some(2)) => HandType::Full,
            (Some(3), _) => HandType::Three,
            (Some(2), Some(2)) => HandType::TwoPairs,
            (Some(2), _) => HandType::Pair,
            (Some(1), _) => HandType::HighCard,
            _ => panic!("Unknown HandType"),
        };

        Hand { cards, hand_type }
    }

    fn new_with_joker(card_chars: &str) -> Hand {
        let cards: Vec<Card> = card_chars
            .chars()
            .map(|c| match c {
                'A' => Card::Ace,
                'K' => Card::King,
                'Q' => Card::Queen,
                'J' => Card::Joker,
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
            .collect();
        let jokers_count = cards
            .iter()
            .filter(|&c| c.cmp(&Card::Joker) == Ordering::Equal)
            .count();
        let two_largest_groups = cards
            .iter()
            .filter(|&c| c.cmp(&Card::Joker) != Ordering::Equal)
            .into_group_map_by(|&c| c.clone())
            .values()
            .map(|v| v.len())
            .sorted()
            .rev()
            .take(2)
            .collect_vec();
        let hand_type = match (
            two_largest_groups.first().unwrap_or(&0) + jokers_count,
            two_largest_groups.get(1),
        ) {
            (5, _) => HandType::Five,
            (4, _) => HandType::Four,
            (3, Some(2)) => HandType::Full,
            (3, _) => HandType::Three,
            (2, Some(2)) => HandType::TwoPairs,
            (2, _) => HandType::Pair,
            (1, _) => HandType::HighCard,
            _ => panic!("Unknown HandType"),
        };

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

fn parse_position(input: &str) -> IResult<&str, Position> {
    let (input, (card_chars, bid)) = separated_pair(alphanumeric1, tag(" "), u64)(input)?;
    Ok((
        input,
        Position {
            hand: Hand::new(card_chars),
            bid,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Position>> {
    let (input, positions) = separated_list1(newline, parse_position)(input)?;
    Ok((input, positions))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, positions) = parse_input(input).unwrap();
    let result: u64 = positions
        .iter()
        .sorted_by(|p1, p2| p1.hand.cmp(&p2.hand))
        .enumerate()
        .map(|(i, p)| (i + 1) as u64 * p.bid)
        .sum();
    Some(result)
}

fn parse_position_with_jokers(input: &str) -> IResult<&str, Position> {
    let (input, (card_chars, bid)) = separated_pair(alphanumeric1, tag(" "), u64)(input)?;
    Ok((
        input,
        Position {
            hand: Hand::new_with_joker(card_chars),
            bid,
        },
    ))
}

fn parse_input_part_2(input: &str) -> IResult<&str, Vec<Position>> {
    let (input, positions) = separated_list1(newline, parse_position_with_jokers)(input)?;
    Ok((input, positions))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, positions) = parse_input_part_2(input).unwrap();
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
