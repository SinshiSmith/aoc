use std::{collections::HashMap, vec};

use nom::{
    branch,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(u32),
    Multiply(u32),
    Power,
}

#[derive(Debug, Clone, Copy)]
struct Test {
    divisible: u32,
    pass: u32,
    fail: u32,
}

#[derive(Debug)]
struct Monkey {
    operation: Operation,
    test: Test,
    inspected: u32,
}

fn parse_items(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = many1(branch::alt((complete::alpha1, complete::space1, tag(":"))))(input)?;
    let (input, items) = separated_list1(tag(", "), complete::u32)(input)?;
    let (input, _) = newline(input)?;
    Ok((input, items))
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = many1(branch::alt((
        complete::alpha1,
        complete::space1,
        tag(":"),
        tag("="),
    )))(input)?;
    let (input, operation) = branch::alt((tag("*"), tag("+")))(input)?;
    let (input, _) = complete::space1(input)?;
    let (input, value) = branch::alt((complete::digit1, complete::alpha1))(input)?;
    let (input, _) = newline(input)?;

    if value == "old" {
        return Ok((input, Operation::Power));
    }

    match operation {
        "*" => Ok((input, Operation::Multiply(value.parse().unwrap()))),
        "+" => Ok((input, Operation::Add(value.parse().unwrap()))),
        _ => panic!("invalid operation"),
    }
}

fn text_parser(input: &str) -> IResult<&str, ()> {
    let (input, _) = many1(branch::alt((complete::space1, complete::alpha1, tag(":"))))(input)?;
    Ok((input, ()))
}

fn parse_test(input: &str) -> IResult<&str, Test> {
    let (input, _) = text_parser(input)?;
    let (input, divisible) = complete::u32(input)?;
    let (input, _) = newline(input)?;

    let (input, _) = text_parser(input)?;
    let (input, pass) = complete::u32(input)?;
    let (input, _) = newline(input)?;

    let (input, _) = text_parser(input)?;
    let (input, fail) = complete::u32(input)?;

    Ok((
        input,
        Test {
            divisible,
            pass,
            fail,
        },
    ))
}

fn parse_monkey(input: &str) -> IResult<&str, (Monkey, Vec<u32>)> {
    let (input, _) = complete::alpha1(input)?;
    let (input, _) = complete::space1(input)?;
    let (input, _) = complete::digit1(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = newline(input)?;
    let (input, items) = parse_items(input)?;
    let (input, operation) = parse_operation(input)?;
    let (input, test) = parse_test(input)?;

    Ok((
        input,
        (
            Monkey {
                operation,
                test,
                inspected: 0,
            },
            items,
        ),
    ))
}

fn process_input(input: &str) -> IResult<&str, (Vec<Monkey>, HashMap<usize, Vec<u32>>)> {
    let mut items_map: HashMap<usize, Vec<u32>> = HashMap::new();
    let (input, results) = separated_list1(tag("\n\n"), parse_monkey)(input)?;

    let mut monkeys = vec![];
    let mut idx: usize = 0;
    for (monkey, items) in results {
        monkeys.push(monkey);
        items_map.insert(idx, items);
        idx += 1;
    }

    Ok((input, (monkeys, items_map)))
}

fn new_item(operation: Operation, item: &u32) -> u32 {
    match operation {
        Operation::Add(value) => (item + value) / 3,
        Operation::Multiply(value) => (item * value) / 3,
        Operation::Power => (item * item) / 3,
    }
}

fn throw_to(test: Test, item: &u32) -> usize {
    if item % test.divisible == 0 {
        test.pass as usize
    } else {
        test.fail as usize
    }
}

pub fn part_1(input: &str) -> u32 {
    let (_, (mut monkeys, mut items_map)) = process_input(input).unwrap();

    for _ in 0..20 {
        for (idx, monkey) in monkeys.iter_mut().enumerate() {
            let items = items_map.remove(&idx).unwrap();
            items.iter().for_each(|item| {
                monkey.inspected += 1;
                let new_item = new_item(monkey.operation, item);
                items_map
                    .entry(throw_to(monkey.test, &new_item))
                    .and_modify(|current_items| current_items.push(new_item));
            });
            items_map.insert(idx, vec![]);
        }
    }

    let mut inspections = monkeys
        .iter()
        .map(|monkey| monkey.inspected)
        .collect::<Vec<u32>>();
    inspections.sort();
    inspections.iter().rev().take(2).product()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn monkey_business() {
        assert_eq!(part_1(INPUT), 10605);
    }
}
