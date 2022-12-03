use itertools::Itertools;

fn char_to_value(char: char) -> u32 {
    let start_value = if char.is_lowercase() { 0 } else { 26 };
    char.to_digit(36).unwrap() - 9 + start_value
}

pub fn part_1(input: String) -> u32 {
    input
        .lines()
        .map(|sack| sack.split_at(sack.len() / 2))
        .map(|(first, second)| first.chars().find(|char| second.contains(*char)).unwrap())
        .map(|char| char_to_value(char))
        .sum()
}

pub fn part_2(input: String) -> u32 {
    input
        .lines()
        .tuples::<(_, _, _)>()
        .map(|(first, second, third)| {
            first
                .chars()
                .find(|char| second.contains(*char) && third.contains(*char))
                .map(|char| char_to_value(char))
                .unwrap()
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn priority_sum() {
        assert_eq!(part_1(INPUT.to_string()), 157);
    }

    #[test]
    fn badge_sum() {
        assert_eq!(part_2(INPUT.to_string()), 70);
    }
}
