use std::fs::read_to_string;

use day_8::part_2;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("{}", part_2(input));
}
