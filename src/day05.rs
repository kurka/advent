use std::fs;

#[allow(dead_code)]
#[derive(Debug)]
struct Instruction {
    quantity: i32,
    from: i32,
    to: i32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Input05 {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

pub fn solve() {
    let _input = parse_input(fs::read_to_string("src/input05.in").unwrap());
    println!("Day 5:");
    // println!("{}", solve_part_a(&input));
    // println!("{}", solve_part_b(&input))
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
    for line in instructions_input.lines().skip(1) {
        let line_parts: Vec<&str> = line.split_whitespace().collect();
        instructions.push(Instruction {
            quantity: line_parts[1].parse().unwrap(),
            from: line_parts[3].parse().unwrap(),
            to: line_parts[5].parse().unwrap(),
        })
    }
    Input05 {
        stacks,
        instructions,
    }
}

// fn solve_part_a(pairs: &Vec<((i32, i32), (i32, i32))>) -> usize {
//     pairs
//         .iter()
//         .filter(|((p11, p12), (p21, p22))| (p11 >= p21 && p12 <= p22) || (p21 >= p11 && p22 <= p12))
//         .count()
// }

// fn solve_part_b(pairs: &Vec<((i32, i32), (i32, i32))>) -> usize {
//     pairs
//         .iter()
//         .filter(|((p11, p12), (p21, p22))| (p11 <= p21 && p12 >= p21) || (p21 <= p11 && p22 >= p11))
//         .count()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let input = parse_input(sample.to_string());
        println!("{:?}", input);

        // assert_eq!("CMZ", solve_part_a(&input));
        // assert_eq!(4, solve_part_b(&input));
    }
}
