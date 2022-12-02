use std::cmp::Reverse;

pub fn part_1(input: String) -> u32 {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|item| item.parse::<u32>().unwrap()).sum())
        .max()
        .expect("no elves found")
}

pub fn part_2(input: String) -> u32 {
    let mut elves: Vec<u32> = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|item| item.parse::<u32>().unwrap()).sum())
        .collect();

    elves.sort_by_key(|elf| Reverse(*elf));

    elves.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn highest_calorie() {
        assert_eq!(part_1(INPUT.to_string()), 24000);
    }
    #[test]
    fn top_3_calorie() {
        assert_eq!(part_2(INPUT.to_string()), 45000);
    }
}
