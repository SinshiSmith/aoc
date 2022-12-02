#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn value(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

enum Result {
    Win,
    Draw,
    Lose,
}

impl Result {
    fn value(&self) -> u32 {
        match self {
            Result::Win => 6,
            Result::Draw => 3,
            Result::Lose => 0,
        }
    }
}

fn turn_to_move(encoded: &str) -> Move {
    match encoded {
        "A" | "X" => Move::Rock,
        "B" | "Y" => Move::Paper,
        "C" | "Z" => Move::Scissors,
        _ => panic!("invalid move"),
    }
}

fn game_score(result: Result, player_move: &Move) -> u32 {
    result.value() + player_move.value()
}

fn game_result(player: &Move, opponent: &Move) -> Result {
    match player {
        Move::Rock => match opponent {
            Move::Scissors => Result::Win,
            Move::Rock => Result::Draw,
            Move::Paper => Result::Lose,
        },
        Move::Paper => match opponent {
            Move::Rock => Result::Win,
            Move::Paper => Result::Draw,
            Move::Scissors => Result::Lose,
        },
        Move::Scissors => match opponent {
            Move::Paper => Result::Win,
            Move::Scissors => Result::Draw,
            Move::Rock => Result::Lose,
        },
    }
}

pub fn part_1(input: String) -> u32 {
    input
        .lines()
        .map(|game| {
            game.split_whitespace()
                .map(|encoded| turn_to_move(encoded))
                .collect::<Vec<Move>>()
        })
        .map(|game_moves| game_score(game_result(&game_moves[1], &game_moves[0]), &game_moves[1]))
        .sum()
}

fn encode_to_result(encoded: &str) -> Result {
    match encoded {
        "Z" => Result::Win,
        "Y" => Result::Draw,
        "X" => Result::Lose,
        _ => panic!("invalid encode"),
    }
}

fn result_to_move(result: &Result, opponent: Move) -> Move {
    match result {
        Result::Draw => opponent,
        Result::Win => match opponent {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        },
        Result::Lose => match opponent {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        },
    }
}

pub fn part_2(input: String) -> u32 {
    let turns = input
        .lines()
        .map(|turn| turn.split_whitespace().collect::<Vec<&str>>());

    let mut score: u32 = 0;

    for turn in turns {
        let opponent_move = turn_to_move(turn[0]);
        let result = encode_to_result(turn[1]);
        let player_move = result_to_move(&result, opponent_move);
        score += result.value() + player_move.value();
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
    B X
    C Z";

    #[test]
    fn total_score() {
        assert_eq!(part_1(INPUT.to_string()), 15);
    }

    #[test]
    fn corrected_total_score() {
        assert_eq!(part_2(INPUT.to_string()), 12);
    }
}
