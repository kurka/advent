use std::fs;

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
type Op = i32;

#[derive(Clone, Debug)]
struct DayOutput {
    reg_a: i32,
    reg_b: i32,
    reg_c: i32,
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
    let regs: Vec<i32> = registers_str
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
    // Vec<i32> {
    let mut reg_a = input.reg_a;
    let mut reg_b = input.reg_b;
    let mut reg_c = input.reg_c;

    let mut pointer = 0;
    let mut output: Vec<i32> = vec![];
    let combo = |op: i32, reg_a: i32, reg_b: i32, reg_c: i32| -> i32 {
        match op {
            0..=3 => op,
            4 => reg_a,
            5 => reg_b,
            6 => reg_c,
            _ => panic!(),
        }
    };

    while pointer < input.instructions.len() {
        let (inst, op) = input.instructions[pointer];
        match inst {
            Inst::Adv => reg_a = reg_a / 2_i32.pow(combo(op, reg_a, reg_b, reg_c) as u32),
            Inst::Bxl => reg_b = reg_b ^ op,
            Inst::Bst => reg_b = combo(op, reg_a, reg_b, reg_c) % 8,
            Inst::Jnz => {}
            Inst::Bxc => reg_b = reg_b ^ reg_c,
            Inst::Out => output.push(combo(op, reg_a, reg_b, reg_c) % 8),
            Inst::Bdv => reg_b = reg_a / 2_i32.pow(combo(op, reg_a, reg_b, reg_c) as u32),
            Inst::Cdv => reg_c = reg_a / 2_i32.pow(combo(op, reg_a, reg_b, reg_c) as u32),
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

fn solve_part_b(input: &DayOutput) -> usize {
    todo!()
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
        // assert_eq!(solve_part_b(&input), 1337);
    }
}
