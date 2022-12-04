use std::{convert::Infallible, str::FromStr};

#[derive(Debug)]
struct Elf(u32, u32);

impl FromStr for Elf {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Infallible> {
        let mut items = s.split("-").map(|id| id.parse::<u32>().unwrap());
        Ok(Elf(items.next().unwrap(), items.next().unwrap()))
    }
}

struct ElfPair(Elf, Elf);

impl FromStr for ElfPair {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Infallible> {
        let mut items = s.split(",").map(|elf| elf.parse::<Elf>().unwrap());
        Ok(ElfPair(items.next().unwrap(), items.next().unwrap()))
    }
}

pub fn part_1(input: String) -> u32 {
    input
        .lines()
        .map(|pair| pair.parse::<ElfPair>().unwrap())
        .filter(|ElfPair(first_elf, second_elf)| {
            let first_elf_range = first_elf.0..=first_elf.1;
            let second_elf_range = second_elf.0..=second_elf.1;

            if first_elf_range.contains(&second_elf.0) && first_elf_range.contains(&second_elf.1) {
                return true;
            }
            if second_elf_range.contains(&first_elf.0) && second_elf_range.contains(&first_elf.1) {
                return true;
            }
            false
        })
        .count() as u32
}

pub fn part_2(input: String) -> u32 {
    input
        .lines()
        .map(|pair| pair.parse::<ElfPair>().unwrap())
        .filter(|ElfPair(first_elf, second_elf)| {
            let first_elf_range = first_elf.0..=first_elf.1;
            let second_elf_range = second_elf.0..=second_elf.1;

            if first_elf_range.contains(&second_elf.0) || second_elf_range.contains(&first_elf.0) {
                true
            } else {
                false
            }
        })
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn how_many_contained() {
        assert_eq!(part_1(INPUT.to_string()), 2);
    }
    #[test]
    fn how_many_overlapped() {
        assert_eq!(part_2(INPUT.to_string()), 4);
    }
}
