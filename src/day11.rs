use std::{
    fs,
    ops::{Add, Div, Mul, Sub},
};

struct Monkey {
    items: Vec<i32>,
    operation: Box<dyn Fn(i32) -> i32>,
    test: i32,
    test_true: usize,
    test_false: usize,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/input11.in").unwrap());
    println!("Day 10:");
    println!("{}", solve_part_a(input));
    // println!("{}", solve_part_b(&input))
}

fn parse_operation(expression: &str) -> impl Fn(i32) -> i32 {
    let (op_str, rhs_str) = expression.split_once(' ').unwrap();

    let op = match op_str {
        "+" => Add::add,
        "-" => Sub::sub,
        "/" => Div::div,
        "*" => Mul::mul,
        _ => panic!("invalid operation: {op_str}"),
    };

    let rhs = match rhs_str {
        "old" => Option::None,
        _ => Option::Some(rhs_str.parse().unwrap()),
    };
    move |old| op(old, rhs.unwrap_or(old)) / 3
}

fn parse_input(input: String) -> Vec<Monkey> {
    input
        .lines()
        .collect::<Vec<&str>>() // arg! couldn't find a better way to use chunks below! :(
        .chunks(7)
        .map(|chunk| Monkey {
            items: chunk[1][18..]
                .split(", ")
                .map(|si| si.parse().unwrap())
                .collect(),
            operation: Box::new(parse_operation(&chunk[2][23..])),
            test: chunk[3][21..].parse().unwrap(),
            test_true: chunk[4][29..].parse().unwrap(),
            test_false: chunk[5][30..].parse().unwrap(),
        })
        .collect()
}

fn solve_part_a(mut monkeys: Vec<Monkey>) -> usize {
    let mut counter = vec![0; monkeys.len()];
    for _round in 0..20 {
        for i in 0..(monkeys.len()) {
            let monkey = &monkeys[i];
            let items = &monkey.items;
            let operation = &monkey.operation;
            let test = monkey.test;
            let test_true = monkey.test_true;
            let test_false = monkey.test_false;

            counter[i] += items.len();
            let (mut trues, mut falses): (Vec<i32>, Vec<i32>) = items
                .iter()
                .map(|v| (operation)(*v))
                .partition(|v| v % test == 0);

            monkeys[i].items.clear();
            monkeys[test_true].items.append(&mut trues);
            monkeys[test_false].items.append(&mut falses);
        }
    }
    // println!("{counter:?}");
    counter.sort();
    counter.reverse();
    counter[0] * counter[1]
}

fn solve_part_b(_monkeys: &Vec<Monkey>) -> Vec<Vec<char>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

        let input = parse_input(sample.to_string());

        assert_eq!(10605, solve_part_a(input));

        // assert_eq!(part2_ans, solve_part_b(&input));
    }
}
