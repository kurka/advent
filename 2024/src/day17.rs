use std::{
    collections::{HashMap, VecDeque},
    fs,
};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Inst {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}
type Op = i64;

#[derive(Clone, Debug)]
struct DayOutput {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    instructions: Vec<(Inst, Op)>,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/17.txt").unwrap());
    println!("Day 17:");
    println!("{:?}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> DayOutput {
    let (registers_str, program_str) = input.split_once("\n\n").unwrap();
    let regs: Vec<i64> = registers_str
        .lines()
        .map(|reg_line| reg_line.split_at(12).1.parse().unwrap())
        .collect();

    let mut insts_str = program_str.split_at(9).1.split(",");
    let mut insts: Vec<(Inst, Op)> = vec![];
    while let (Some(inst_str), Some(op_str)) = (insts_str.next(), insts_str.next()) {
        insts.push((
            match inst_str {
                "0" => Inst::Adv,
                "1" => Inst::Bxl,
                "2" => Inst::Bst,
                "3" => Inst::Jnz,
                "4" => Inst::Bxc,
                "5" => Inst::Out,
                "6" => Inst::Bdv,
                "7" => Inst::Cdv,
                _ => panic!(),
            },
            op_str.trim().parse().unwrap(),
        ))
    }
    DayOutput {
        reg_a: regs[0],
        reg_b: regs[1],
        reg_c: regs[2],
        instructions: insts,
    }
}

fn solve_part_a(input: &DayOutput) -> String {
    run_program(input.reg_a, input.reg_b, input.reg_c, &input.instructions)
}

fn run_program(reg_a: i64, reg_b: i64, reg_c: i64, instructions: &Vec<(Inst, Op)>) -> String {
    let mut reg_a = reg_a;
    let mut reg_b = reg_b;
    let mut reg_c = reg_c;

    let mut pointer = 0;
    let mut output: Vec<i64> = vec![];
    let combo = |op: i64, reg_a: i64, reg_b: i64, reg_c: i64| -> i64 {
        match op {
            0..=3 => op,
            4 => reg_a,
            5 => reg_b,
            6 => reg_c,
            _ => panic!(),
        }
    };

    while pointer < instructions.len() {
        let (inst, op) = instructions[pointer];
        match inst {
            Inst::Adv => reg_a = reg_a / 2_i64.pow(combo(op, reg_a, reg_b, reg_c) as u32),
            Inst::Bxl => reg_b = reg_b ^ op,
            Inst::Bst => reg_b = combo(op, reg_a, reg_b, reg_c) % 8,
            Inst::Jnz => {}
            Inst::Bxc => reg_b = reg_b ^ reg_c,
            Inst::Out => output.push(combo(op, reg_a, reg_b, reg_c) % 8),
            Inst::Bdv => reg_b = reg_a / 2_i64.pow(combo(op, reg_a, reg_b, reg_c) as u32),
            Inst::Cdv => reg_c = reg_a / 2_i64.pow(combo(op, reg_a, reg_b, reg_c) as u32),
        }
        if inst == Inst::Jnz && reg_a != 0 {
            pointer = op as usize
        } else {
            pointer += 1
        }
    }
    output
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn solve_part_b(input: &DayOutput) -> i64 {
    let mut patterns: HashMap<i64, Vec<i64>> = HashMap::new();

    // store the resulting 3 bits for each possible 10 bits input
    for a in 0_i64..0b10_000_000_000_i64 {
        // one-liner version of the program
        // let out = ((((a % 0b1000) ^ 0b0001) ^ 0b101) ^ (a / 1 << ((a % 0b1000) ^ 0b0001))) % 0b1000;
        let out = (((a % 0b1000) ^ 0b0100) ^ (a / (1 << ((a % 0b1000) ^ 0b0001)))) % 0b1000;
        patterns.entry(out).or_default().push(a);
    }
    let target_sequence = vec![2, 4, 1, 1, 7, 5, 1, 5, 4, 0, 0, 3, 5, 5, 3, 0];

    // run a bfs to find valid paths for solution
    let mut queue = VecDeque::new();
    queue.extend(patterns[&target_sequence[0]].clone());

    for (target_pos, target) in target_sequence[1..].iter().enumerate() {
        // println!("Searching {target} {:?}", queue);
        let level_size = queue.len();
        for _ in 0..level_size {
            let candidate = queue.pop_front().unwrap();
            let common_bits = candidate >> (3 * (target_pos + 1));
            for pattern in <Vec<i64> as Clone>::clone(&patterns[&target]).into_iter() {
                // check if first 7 bits of pattern is equal to last 7 from candidate
                if pattern % (1 << 7) == common_bits {
                    // add 3 last bits of pattern at the end of candidate
                    queue.push_back(candidate + ((pattern >> 7) << (10 + 3 * target_pos)));
                }
            }
            // break;
        }
        // break;
    }

    queue.into_iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), "4,6,3,5,6,3,5,2,1,0");
    }
}
