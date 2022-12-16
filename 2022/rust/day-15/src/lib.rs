use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair},
    IResult,
};
#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}
#[derive(Debug)]
struct Sensor {
    coord: Coord,
    beacon: Coord,
}
fn coord_parser(input: &str) -> IResult<&str, Coord> {
    let (input, (x, y)) = separated_pair(
        preceded(tag("="), complete::i32),
        tag(", "),
        preceded(tag("y="), complete::i32),
    )(input)?;
    Ok((input, Coord { x, y }))
}

fn sensor_parser(input: &str) -> IResult<&str, Sensor> {
    let (input, coord) = preceded(
        many1(alt((complete::alpha1, complete::space1))),
        coord_parser,
    )(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, beacon) = preceded(
        many1(alt((complete::alpha1, complete::space1))),
        coord_parser,
    )(input)?;

    Ok((input, Sensor { coord, beacon }))
}

fn input_parser(input: &str) -> Vec<Sensor> {
    separated_list1(newline, sensor_parser)(input).unwrap().1
}

pub fn part_1(input: &str, target: i32) -> usize {
    let sensors = input_parser(input);
    let mut ranges = vec![];

    sensors
        .iter()
        .filter(|Sensor { coord, beacon }| {
            let distance = i32::abs(coord.x - beacon.x) + i32::abs(coord.y - beacon.y);
            let distance_to_target = i32::abs(coord.y - target);
            distance_to_target <= distance
        })
        .for_each(|Sensor { coord, beacon }| {
            let distance = i32::abs(coord.x - beacon.x) + i32::abs(coord.y - beacon.y);
            let distance_to_target = i32::abs(coord.y - target);

            let x_range =
                coord.x - distance + distance_to_target..coord.x + distance - distance_to_target;

            ranges.push(x_range);
        });

    ranges
        .into_iter()
        .reduce(|accum, current| accum.start.min(current.start)..accum.end.max(current.end))
        .unwrap()
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn it_works() {
        assert_eq!(part_1(INPUT, 10), 26);
    }
}
