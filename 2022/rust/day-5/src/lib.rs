pub fn part_1(input: String) -> &'static str {
    "good luck"
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
}
