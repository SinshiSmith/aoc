pub fn part_1(input: &str) -> u32 {
    let mut tracker = String::new();
    let mut chars = input.chars().enumerate();
    while tracker.len() < 4 {
        let (_, current) = chars.next().unwrap();
        if !tracker.contains(current) {
            tracker.push(current);
        } else {
            let first_appear = tracker.chars().position(|char| char == current).unwrap();
            drop(tracker.drain(..=first_appear));
            tracker.push(current);
        }
    }
    chars.next().unwrap().0 as u32
}

pub fn part_2(input: &str) -> u32 {
    let mut tracker = String::new();
    let mut chars = input.chars().enumerate();
    while tracker.len() < 14 {
        let (_, current) = chars.next().unwrap();
        if !tracker.contains(current) {
            tracker.push(current);
        } else {
            let first_appear = tracker.chars().position(|char| char == current).unwrap();
            drop(tracker.drain(..=first_appear));
            tracker.push(current);
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
    #[test]
    fn message_processed_count() {
        assert_eq!(part_2(INPUT), 26);
    }
}
