struct Instruction {
    repeat: usize,
    value: i32,
}

fn parse_instruction(instruction: &str) -> Instruction {
    let mut instruction = instruction.split(" ");
    let instruction_type = instruction.next().unwrap();
    if instruction_type == "noop" {
        Instruction {
            repeat: 1,
            value: 0,
        }
    } else {
        Instruction {
            repeat: 2,
            value: instruction.next().unwrap().parse::<i32>().unwrap(),
        }
    }
}

pub fn part_1(input: &str) -> i32 {
    let mut register = 1;
    let mut instructions = input.lines().flat_map(|line| {
        let instruction = parse_instruction(line);
        let cycles = std::iter::repeat(register).take(instruction.repeat);
        register += instruction.value;
        cycles
    });

    let signal_strength_cycles = [20, 40, 40, 40, 40, 40];
    let mut multiplier = 0;

    signal_strength_cycles
        .map(|cycle| {
            let signal_strength =
                instructions.nth(cycle - 1).unwrap() * (cycle + multiplier) as i32;
            multiplier += cycle;
            signal_strength
        })
        .iter()
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn signal_strength_sum() {
        assert_eq!(part_1(INPUT), 13140);
    }
}
