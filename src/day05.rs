use std::fs;

#[derive(Clone, Debug)]
struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

#[derive(Clone, Debug)]
struct Input05 {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/input05.in").unwrap());
    println!("Day 5:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input))
}

fn parse_input(input: String) -> Input05 {
    let (stacks_input, instructions_input) = input.split_once("\n\n").unwrap();

    let n_stacks = (stacks_input.lines().last().unwrap().len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); n_stacks];

    for line in stacks_input.lines().rev().skip(1) {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }

    let mut instructions: Vec<Instruction> = Vec::new();
    for line in instructions_input.lines() {
        let line_parts: Vec<&str> = line.split_whitespace().collect();
        instructions.push(Instruction {
            quantity: line_parts[1].parse().unwrap(),
            from: line_parts[3].parse::<usize>().unwrap() - 1,
            to: line_parts[5].parse::<usize>().unwrap() - 1,
        })
    }
    Input05 {
        stacks,
        instructions,
    }
}

fn solve_5(input: &Input05, reverse: bool) -> String {
    let mut stacks = input.stacks.clone();
    for instruction in &input.instructions {
        let stack_size = stacks[instruction.from].len();
        let mut drain: Vec<char> = stacks[instruction.from]
            .drain((stack_size - instruction.quantity)..)
            .collect();
        if reverse {
            drain.reverse();
        }
        stacks[instruction.to].append(&mut drain);
    }

    stacks
        .iter()
        .map(|stack| stack.last().unwrap().clone())
        .collect()
}

fn solve_part_a(input: &Input05) -> String {
    solve_5(input, true)
}

fn solve_part_b(input: &Input05) -> String {
    solve_5(input, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let input = parse_input(sample.to_string());

        assert_eq!("CMZ", solve_part_a(&input));
        assert_eq!("MCD", solve_part_b(&input));
    }
}
