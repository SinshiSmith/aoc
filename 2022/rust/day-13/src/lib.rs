use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum SignalItem {
    Number(u8),
    List(Vec<SignalItem>),
}

impl SignalItem {
    fn get(&self) -> Vec<u8> {
        match self {
            SignalItem::Number(value) => vec![*value],
            SignalItem::List(list) => {
                if list.len() == 0 {
                    vec![0]
                } else {
                    list.iter().flat_map(|item| item.get()).collect::<Vec<_>>()
                }
            }
        }
    }
}

#[derive(Debug)]
struct FlatSignal {
    items: Vec<u8>,
}

impl From<&SignalPart> for FlatSignal {
    fn from(signal_part: &SignalPart) -> Self {
        Self {
            items: signal_part
                .items
                .iter()
                .flat_map(|item| item.get())
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SignalPart {
    items: Vec<SignalItem>,
}

#[derive(Debug)]
struct Pair {
    left: SignalPart,
    right: SignalPart,
}

fn nested_list_parser(input: &str) -> IResult<&str, SignalItem> {
    let (input, item) = delimited(
        tag("["),
        separated_list0(
            tag(","),
            alt((
                complete::u8.map(|number| SignalItem::Number(number)),
                nested_list_parser,
            )),
        )
        .map(|list| SignalItem::List(list)),
        tag("]"),
    )(input)?;
    Ok((input, item))
}

fn list_parser(input: &str) -> IResult<&str, SignalPart> {
    let (input, _) = tag("[")(input)?;
    if input.chars().next().unwrap() == ']' {
        let (input, _) = tag("]")(input)?;
        return Ok((input, SignalPart { items: vec![] }));
    }
    let (input, items) = separated_list1(
        tag(","),
        alt((
            complete::u8.map(|d| SignalItem::Number(d)),
            nested_list_parser,
        )),
    )(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, SignalPart { items }))
}

fn pair_parser(input: &str) -> IResult<&str, Pair> {
    let (input, (left, right)) = separated_pair(list_parser, newline, list_parser)(input)?;
    Ok((input, Pair { left, right }))
}

fn input_parser(input: &str) -> Vec<Pair> {
    separated_list1(tag("\n\n"), pair_parser)(input).unwrap().1
}

fn is_number_equal(left: &u8, right: &u8) -> Option<bool> {
    if left == right {
        None
    } else {
        Some(left < right)
    }
}

fn is_ordered(left: Vec<&SignalItem>, right: Vec<&SignalItem>, is_inner: bool) -> Option<bool> {
    let mut right_items = right.iter();
    for left_item in &left {
        match right_items.next() {
            Some(right_item) => match (left_item, right_item) {
                (SignalItem::Number(left_number), SignalItem::Number(right_number)) => {
                    if let Some(equal) = is_number_equal(left_number, right_number) {
                        return Some(equal);
                    } else {
                        continue;
                    }
                }
                (SignalItem::Number(_), SignalItem::List(right_list)) => {
                    if let Some(equal) = is_ordered(
                        vec![left_item],
                        right_list.iter().collect::<Vec<&SignalItem>>(),
                        true,
                    ) {
                        return Some(equal);
                    } else {
                        continue;
                    }
                }
                (SignalItem::List(left_list), SignalItem::Number(_)) => {
                    if let Some(equal) = is_ordered(
                        left_list.iter().collect::<Vec<&SignalItem>>(),
                        vec![right_item],
                        true,
                    ) {
                        return Some(equal);
                    } else {
                        continue;
                    }
                }
                (SignalItem::List(left_list), SignalItem::List(right_list)) => {
                    if let Some(equal) = is_ordered(
                        left_list.iter().collect::<Vec<&SignalItem>>(),
                        right_list.iter().collect::<Vec<&SignalItem>>(),
                        true,
                    ) {
                        return Some(equal);
                    } else {
                        continue;
                    }
                }
            },
            None => return Some(false),
        }
    }

    if is_inner && left.len() == right.len() {
        return None;
    }

    Some(left.len() < right.len())
}

pub fn part_1(input: &str) -> usize {
    let pairs = input_parser(input);
    pairs
        .iter()
        .enumerate()
        .filter_map(|(idx, pair)| {
            let left = pair.left.items.iter().collect::<Vec<&SignalItem>>();
            let right = pair.right.items.iter().collect::<Vec<&SignalItem>>();
            if is_ordered(left, right, false).unwrap() {
                Some(idx + 1)
            } else {
                None
            }
        })
        .inspect(|idx| {
            dbg!(idx);
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let mut pairs = input_parser(input);
    pairs.push(Pair {
        left: SignalPart {
            items: vec![SignalItem::Number(2)],
        },
        right: SignalPart {
            items: vec![SignalItem::Number(6)],
        },
    });
    let mut signals = pairs
        .iter()
        .flat_map(|pair| [&pair.left, &pair.right])
        .map(|signal| FlatSignal::from(signal).items)
        .collect::<Vec<_>>();
    signals.sort();
    // dbg!(&signals);
    signals
        .iter()
        .enumerate()
        .filter_map(|(idx, signal)| {
            if signal.len() == 1 {
                if signal[0] == 2 || signal[0] == 6 {
                    dbg!(signal);
                    return Some(idx + 1);
                }
            }
            None
        })
        .inspect(|item| {
            dbg!(item);
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn in_order() {
        assert_eq!(part_1(INPUT), 13);
    }

    #[test]
    fn decoder_key() {
        assert_eq!(part_2(INPUT), 141);
    }
}
