use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
enum Unit {
    Sand,
    Rock,
}

fn point_parser(input: &str) -> IResult<&str, Point> {
    let (input, (x, y)) = separated_pair(
        complete::digit1.map(|result: &str| result.parse::<u32>().unwrap()),
        tag(","),
        complete::digit1.map(|result: &str| result.parse::<u32>().unwrap()),
    )(input)?;
    Ok((input, Point { x, y }))
}

fn line_parser(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(tag(" -> "), point_parser)(input)
}

fn input_parser(input: &str) -> Vec<Vec<Point>> {
    separated_list1(newline, line_parser)(input).unwrap().1
}

fn area_map(input: Vec<Vec<Point>>) -> (HashMap<Point, Unit>, u32) {
    let mut unit_map: HashMap<Point, Unit> = HashMap::new();
    let mut max_y = 0;

    for line in input {
        for i in 0..(line.len() - 1) {
            let (first, second) = (line.get(i).unwrap(), line.get(i + 1).unwrap());

            let x_range = if first.x < second.x {
                first.x..=second.x
            } else {
                second.x..=first.x
            };
            for x in x_range {
                let y_range = if first.y < second.y {
                    first.y..=second.y
                } else {
                    second.y..=first.y
                };

                for y in y_range {
                    if y > max_y {
                        max_y = y;
                    }
                    unit_map.insert(Point { x, y }, Unit::Rock);
                }
            }
        }
    }
    (unit_map, max_y)
}

fn next_point(current_point: &Point, unit_map: &HashMap<Point, Unit>) -> Option<Point> {
    let y = current_point.y + 1;
    let down = Point {
        x: current_point.x,
        y,
    };
    let left_down = Point {
        x: current_point.x - 1,
        y,
    };
    let right_down = Point {
        x: current_point.x + 1,
        y,
    };

    if unit_map.get(&down).is_none() {
        return Some(down);
    }

    if unit_map.get(&left_down).is_none() {
        return Some(left_down);
    }

    if unit_map.get(&right_down).is_none() {
        return Some(right_down);
    }
    None
}

pub fn part_1(input: &str) -> usize {
    let parsed = input_parser(input);
    let (mut unit_map, max_y) = area_map(parsed);
    let mut all_rested = false;

    while !all_rested {
        let mut rested = false;
        let mut current_point = Point { x: 500, y: 0 };

        while !rested {
            if current_point.y > max_y {
                rested = true;
                all_rested = true;
            }

            if let Some(next_point) = next_point(&current_point, &unit_map) {
                current_point = next_point;
            } else {
                rested = true;
            }
        }

        unit_map.insert(current_point, Unit::Sand);
    }

    unit_map
        .iter()
        .filter(|(_, unit)| match unit {
            Unit::Sand => true,
            _ => false,
        })
        .count()
        - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn rested_sand_count() {
        assert_eq!(part_1(INPUT), 24);
    }
}
