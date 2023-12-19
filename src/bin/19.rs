use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::newline,
        complete::{alpha1, alphanumeric1, char, u32},
    },
    combinator::opt,
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

advent_of_code::solution!(19);

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn get_by_name(&self, field_name: &str) -> u32 {
        match field_name {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => panic!("Unknown field"),
        }
    }
}

#[derive(Debug, Clone)]
enum State {
    Accept,
    Reject,
    WorkflowName { name: String },
}

#[derive(Debug)]
struct Condition {
    field: String,
    operator: char,
    value: u32,
}

impl Condition {
    fn evaluate(&self, part: &Part) -> bool {
        let field_value = part.get_by_name(self.field.as_str());
        match self.operator {
            '<' => field_value < self.value,
            '>' => field_value > self.value,
            _ => panic!("Unknown operator"),
        }
    }
}

#[derive(Debug)]
struct Rule {
    target: State,
    condition: Option<Condition>,
}

impl Rule {
    fn apply(&self, part: &Part) -> Option<State> {
        match &self.condition {
            Some(condition) => {
                if condition.evaluate(part) {
                    Some(self.target.clone())
                } else {
                    None
                }
            }
            None => Some(self.target.clone()),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn apply(&self, part: &Part) -> State {
        self.rules
            .iter()
            .flat_map(|rule| rule.apply(part))
            .collect_vec()
            .first()
            .unwrap()
            .clone()
    }
}

fn parse_condition(input: &str) -> IResult<&str, Condition> {
    let (input, field) = alpha1(input)?;
    let (input, operator) = alt((char('>'), char('<')))(input)?;
    let (input, value) = alphanumeric1(input)?;
    let (input, _) = tag(":")(input)?;
    Ok((
        input,
        Condition {
            field: field.to_string(),
            operator,
            value: value.parse().unwrap(),
        },
    ))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, condition) = opt(parse_condition)(input)?;
    let (input, target) = alpha1(input)?;
    Ok((
        input,
        Rule {
            condition,
            target: match target {
                "A" => State::Accept,
                "R" => State::Reject,
                rule_name => State::WorkflowName {
                    name: rule_name.to_string(),
                },
            },
        },
    ))
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, name) = alpha1(input).map(|(i, n)| (i, n.to_string()))?;
    let (input, rules) =
        delimited(tag("{"), separated_list1(tag(","), parse_rule), tag("}"))(input)?;
    Ok((input, Workflow { name, rules }))
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let (input, _) = tag("{x=")(input)?;
    let (input, x) = u32(input)?;
    let (input, _) = tag(",m=")(input)?;
    let (input, m) = u32(input)?;
    let (input, _) = tag(",a=")(input)?;
    let (input, a) = u32(input)?;
    let (input, _) = tag(",s=")(input)?;
    let (input, s) = u32(input)?;
    let (input, _) = tag("}")(input)?;
    Ok((input, Part { x, m, a, s }))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Workflow>, Vec<Part>)> {
    let (input, workflows) = separated_list1(newline, parse_workflow)(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, parts) = separated_list1(newline, parse_part)(input)?;
    Ok((input, (workflows, parts)))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, (workflows, parts)) = parse_input(input).unwrap();
    let workflow_map: HashMap<String, &Workflow> =
        workflows.iter().map(|w| (w.name.clone(), w)).collect();
    let start = State::WorkflowName {
        name: "in".to_string(),
    };
    let result = parts
        .iter()
        .map(|part| {
            let mut state = start.clone();
            while let Some(workflow_name) = match state.clone() {
                State::Accept => None,
                State::Reject => None,
                State::WorkflowName { name } => Some(name),
            } {
                state = workflow_map[&workflow_name].apply(part);
            }
            match state {
                State::Accept => part.x + part.m + part.a + part.s,
                State::Reject => 0,
                State::WorkflowName { name } => panic!("Invalid state: WorkflowName {name}"),
            }
        })
        .sum();
    Some(result)
}

pub fn part_two(_: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
