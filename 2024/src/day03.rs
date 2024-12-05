use regex::Regex;
use std::fs;

#[derive(Debug)]
enum Operation {
    Do,
    Dont,
    Mul(i32, i32),
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/03.txt").unwrap());
    println!("Day 03:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<Operation> {
    let re_op = Regex::new(r"(mul|don't|do)(\([\d,]*?\))").unwrap();
    let re_par_mul = Regex::new(r"\((\d+),(\d+)\)").unwrap();
    let re_par_do = Regex::new(r"\(\)").unwrap();
    let mut ops = vec![];

    for (_, [op_name, op_parenthesis]) in re_op.captures_iter(&input).map(|c| c.extract()) {
        match op_name {
            "do" => {
                if re_par_do.is_match(op_parenthesis) {
                    ops.push(Operation::Do)
                }
            }
            "don't" => {
                if re_par_do.is_match(op_parenthesis) {
                    ops.push(Operation::Dont)
                }
            }
            "mul" => match re_par_mul.captures(op_parenthesis) {
                Some(caps) => ops.push(Operation::Mul(
                    caps[1].parse().unwrap(),
                    caps[2].parse().unwrap(),
                )),
                None => continue,
            },
            _ => unreachable!(),
        }
    }
    ops
}

fn solve_part_a(input: &Vec<Operation>) -> i32 {
    solve_day03(input, true)
}

fn solve_part_b(input: &Vec<Operation>) -> i32 {
    solve_day03(input, false)
}

fn solve_day03(input: &Vec<Operation>, ignore_dos: bool) -> i32 {
    let mut enabled = true;
    let mut res = 0;
    for op in input {
        match op {
            Operation::Mul(a, b) => {
                if enabled {
                    res += a * b
                }
            }
            Operation::Do => enabled = true,
            Operation::Dont => {
                if !ignore_dos {
                    enabled = false
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";
        let input = parse_input(sample.to_string());
        assert_eq!(solve_part_a(&input), 161);

        let sample = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";
        let input = parse_input(sample.to_string());
        assert_eq!(solve_part_b(&input), 48);
    }
}
