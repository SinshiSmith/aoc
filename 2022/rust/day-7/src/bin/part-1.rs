use std::fs::read_to_string;

use day_7::part_1;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("{}", part_1(input.to_string()))
}
