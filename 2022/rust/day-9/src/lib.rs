use std::{collections::HashSet, convert::Infallible, str::FromStr};

enum Move {
    X(i32),
    Y(i32),
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl FromStr for Move {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Infallible> {
        let mut line = s.split(" ");
        let direction = line.next().unwrap();
        let amount = line.next().unwrap().parse::<i32>().unwrap();

        match direction {
            "U" => Ok(Move::Y(amount)),
            "D" => Ok(Move::Y(-amount)),
            "L" => Ok(Move::X(-amount)),
            "R" => Ok(Move::X(amount)),
            _ => panic!("invalid move"),
        }
    }
}

fn move_head(amount: i32, direction: char, head: &Position) -> Position {
    match direction {
        'x' => Position {
            x: head.x + amount,
            y: head.y,
        },
        'y' => Position {
            x: head.x,
            y: head.y + amount,
        },
        _ => panic!("invalid direction"),
    }
}

fn move_tail(head: &Position, tail: &Position) -> Position {
    let x_direction = if head.x - tail.x > 0 { 1 } else { -1 };
    let y_direction = if head.y - tail.y > 0 { 1 } else { -1 };
    let x_diff = i32::abs(head.x - tail.x);
    let y_diff = i32::abs(head.y - tail.y);

    match (x_diff, y_diff) {
        (0, 0) | (0, 1) | (1, 0) | (1, 1) => Position {
            x: tail.x,
            y: tail.y,
        },

        (0, _) => Position {
            x: tail.x,
            y: tail.y + y_direction,
        },

        (_, 0) => Position {
            x: tail.x + x_direction,
            y: tail.y,
        },
        _ => Position {
            x: tail.x + x_direction,
            y: tail.y + y_direction,
        },
    }
}

fn process_move(move_amount: i32) -> (i32, i32) {
    (if move_amount > 0 { 1 } else { -1 }, i32::abs(move_amount))
}

pub fn part_1(input: &str) -> usize {
    let moves = input.lines().map(|line| line.parse::<Move>().unwrap());
    let mut head = Position::default();
    let mut tail = Position::default();

    let mut visited: HashSet<Position> = HashSet::new();

    for head_move in moves {
        match head_move {
            Move::X(amount) => {
                let (direction, repeat) = process_move(amount);
                for _ in 0..repeat {
                    head = move_head(direction, 'x', &head);
                    tail = move_tail(&head, &tail);
                    visited.insert(Position {
                        x: tail.x,
                        y: tail.y,
                    });
                }
            }
            Move::Y(amount) => {
                let (direction, repeat) = process_move(amount);
                for _ in 0..repeat {
                    head = move_head(direction, 'y', &head);
                    tail = move_tail(&head, &tail);
                    visited.insert(Position {
                        x: tail.x,
                        y: tail.y,
                    });
                }
            }
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn visited_at_once() {
        let result = part_1(INPUT);
        assert_eq!(result, 13);
    }
}
