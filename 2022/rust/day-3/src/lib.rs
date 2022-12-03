pub fn part_1(input: String) -> u32 {
    input
        .lines()
        .map(|sack| sack.split_at(sack.len() / 2))
        .map(|(first, second)| first.chars().find(|char| second.contains(*char)).unwrap())
        .map(|char| {
            let start_value = if char.is_lowercase() { 0 } else { 26 };
            char.to_digit(36).unwrap() - 9 + start_value
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
}
