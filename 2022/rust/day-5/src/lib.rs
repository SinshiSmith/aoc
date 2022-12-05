use std::collections::HashMap;

use nom::{
    branch,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{many1, separated_list1},
    IResult,
};

fn crate_parser(input: &str) -> IResult<&str, char> {
    let (input, _) = tag("[")(input)?;
    let (input, crate_name) = complete::anychar(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, crate_name))
}

fn empty_slot(input: &str) -> IResult<&str, char> {
    let (input, _) = tag("   ")(input)?;
    Ok((input, ' '))
}

fn crate_or_empty(input: &str) -> IResult<&str, char> {
    let (input, result) = branch::alt((crate_parser, empty_slot))(input)?;
    Ok((input, result))
}

fn crate_line_parser(input: &str) -> IResult<&str, Vec<char>> {
    let (input, crates) = separated_list1(tag(" "), crate_or_empty)(input)?;
    Ok((input, crates))
}
fn total_crates(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (input, crates) = separated_list1(newline, crate_line_parser)(input)?;
    let (input, _) = newline(input)?;
    Ok((input, crates))
}
fn crate_number_parser(input: &str) -> IResult<&str, u8> {
    let (input, _) = complete::space0(input)?;
    let (input, digit) = complete::u8(input)?;
    let (input, _) = complete::space0(input)?;
    Ok((input, digit))
}

fn crate_number_parsing(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, result) = many1(crate_number_parser)(input)?;
    Ok((input, result))
}

fn instruction_part(input: &str) -> IResult<&str, u32> {
    let (input, _) = many1(complete::alpha1)(input)?;
    let (input, _) = complete::space1(input)?;
    let (input, digit) = complete::u32(input)?;
    let (input, _) = complete::space0(input)?;
    Ok((input, digit))
}

fn instruction(input: &str) -> IResult<&str, (u32, u32, u32)> {
    let (input, amount) = instruction_part(input)?;
    let (input, from) = instruction_part(input)?;
    let (input, to) = instruction_part(input)?;

    Ok((input, (amount, from, to)))
}

fn instruction_list(input: &str) -> IResult<&str, Vec<(u32, u32, u32)>> {
    let (input, list) = separated_list1(newline, instruction)(input)?;
    Ok((input, list))
}

fn input_parser(input: &str) -> IResult<&str, Vec<(u32, u32, u32)>> {
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let (input, instructions) = instruction_list(input)?;
    Ok((input, instructions))
}

pub fn part_1(input: String) -> String {
    let mut stack_hash_map: HashMap<usize, Vec<char>> = HashMap::new();

    let (input, crates) = total_crates(&input).unwrap();
    let (input, _) = crate_number_parsing(input).unwrap();
    let (_, instructions) = input_parser(&input).unwrap();

    for row in crates.iter().rev() {
        for (idx, crate_name) in row.iter().enumerate() {
            if !crate_name.is_whitespace() {
                stack_hash_map
                    .entry(idx + 1)
                    .or_insert(Vec::new())
                    .push(*crate_name);
            }
        }
    }

    for (amount, from, to) in instructions {
        let from_stack = stack_hash_map.get_mut(&(from as usize)).unwrap();
        let amount = from_stack.len() - (amount as usize);
        let mut moved = from_stack.drain(amount..).rev().collect();
        stack_hash_map
            .entry(to as usize)
            .and_modify(|stack| stack.append(&mut moved));
    }

    let mut answer = "".to_string();
    for key in 1..=stack_hash_map.len() {
        answer.push(*stack_hash_map.get(&key).unwrap().last().unwrap());
    }

    answer
}

pub fn part_2(input: String) -> String {
    let mut stack_hash_map: HashMap<usize, Vec<char>> = HashMap::new();

    let (input, crates) = total_crates(&input).unwrap();
    let (input, _) = crate_number_parsing(input).unwrap();
    let (_, instructions) = input_parser(&input).unwrap();

    for row in crates.iter().rev() {
        for (idx, crate_name) in row.iter().enumerate() {
            if !crate_name.is_whitespace() {
                stack_hash_map
                    .entry(idx + 1)
                    .or_insert(Vec::new())
                    .push(*crate_name);
            }
        }
    }

    for (amount, from, to) in instructions {
        let from_stack = stack_hash_map.get_mut(&(from as usize)).unwrap();
        let amount = from_stack.len() - (amount as usize);
        let mut moved = from_stack.drain(amount..).collect();
        stack_hash_map
            .entry(to as usize)
            .and_modify(|stack| stack.append(&mut moved));
    }

    let mut answer = "".to_string();
    for key in 1..=stack_hash_map.len() {
        answer.push(*stack_hash_map.get(&key).unwrap().last().unwrap());
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
    [N] [C]    
    [Z] [M] [P]
     1   2   3 
    
    move 1 from 2 to 1
    move 3 from 1 to 3
    move 2 from 2 to 1
    move 1 from 1 to 2";

    #[test]
    fn top_crates() {
        assert_eq!(part_1(INPUT.to_string()), "CMZ");
    }

    #[test]
    fn top_crates_corrected() {
        assert_eq!(part_1(INPUT.to_string()), "MCD");
    }
}
