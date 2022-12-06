use std::collections::VecDeque;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn part_1(input: &str) -> u32 {
    let mut tracker = VecDeque::new();
    let mut chars = input.chars().enumerate();
    while tracker.len() < 4 {
        let (_, current) = chars.next().unwrap();
        if !tracker.contains(&current) {
            tracker.push_back(current);
        } else {
            let first_appear = tracker.iter().position(|char| *char == current).unwrap();
            drop(tracker.drain(..=first_appear));
            tracker.push_back(current);
        }
    }
    chars.next().unwrap().0 as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn processed_count() {
        assert_eq!(part_1(INPUT), 11);
    }
}
