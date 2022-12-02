pub fn part_1(input: String) -> u32 {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|item| item.parse::<u32>().unwrap()).sum())
        .max()
        .expect("no elves found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highest_calorie() {
        let input = "1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000"
            .to_string();

        assert_eq!(part_1(input), 24000);
    }
}
