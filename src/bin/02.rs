use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline, u32},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(2);

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct CubeSet {
    color: Color,
    number_of_cubes: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    cube_sets: Vec<CubeSet>,
}

#[derive(Debug)]
struct GameMaxReveled {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

fn cube_set(input: &str) -> IResult<&str, CubeSet> {
    let (input, (number_of_cubes, color_raw)) = separated_pair(u32, tag(" "), alpha1)(input)?;
    let color: Color = match color_raw {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        _ => panic!("Unknown color"),
    };
    Ok((
        input,
        CubeSet {
            color,
            number_of_cubes,
        },
    ))
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = u32(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, cube_sets) = separated_list1(alt((tag(", "), tag("; "))), cube_set)(input)?;
    Ok((input, Game { id, cube_sets }))
}

fn process_input(input: &str) -> Vec<Game> {
    let (_, games) = separated_list1(newline, game)(input).unwrap();
    games
}

fn find_max_reveled_color(game: &Game, color: Color) -> u32 {
    game.cube_sets
        .iter()
        .filter(|cs| cs.color == color)
        .map(|cs| cs.number_of_cubes)
        .max()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let games = process_input(input);
    let games_max_reveled_sets = games.iter().map(|game| {
        let max_reveled_red = find_max_reveled_color(game, Color::Red);
        let max_reveled_green = find_max_reveled_color(game, Color::Green);
        let max_reveled_blue = find_max_reveled_color(game, Color::Blue);
        GameMaxReveled {
            id: game.id,
            red: max_reveled_red,
            green: max_reveled_green,
            blue: max_reveled_blue,
        }
    });
    let result = games_max_reveled_sets
        .filter(|g| g.red <= max_red && g.green <= max_green && g.blue <= max_blue)
        .map(|g| g.id)
        .sum::<u32>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = process_input(input);
    let games_max_reveled_sets = games.iter().map(|game| {
        let max_reveled_red = find_max_reveled_color(game, Color::Red);
        let max_reveled_green = find_max_reveled_color(game, Color::Green);
        let max_reveled_blue = find_max_reveled_color(game, Color::Blue);
        GameMaxReveled {
            id: game.id,
            red: max_reveled_red,
            green: max_reveled_green,
            blue: max_reveled_blue,
        }
    });
    let result = games_max_reveled_sets
        .map(|g| g.red * g.green * g.blue)
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
