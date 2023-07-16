use std::{
    fs,
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

#[derive(Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: Rc<dyn Fn(i64) -> i64>,
    test: i64,
    test_true: usize,
    test_false: usize,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/input11.in").unwrap());
    println!("Day 11:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input))
}

fn parse_operation(expression: &str) -> impl Fn(i64) -> i64 {
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
    move |old| op(old, rhs.unwrap_or(old))
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
            operation: Rc::new(parse_operation(&chunk[2][23..])),
            test: chunk[3][21..].parse().unwrap(),
            test_true: chunk[4][29..].parse().unwrap(),
            test_false: chunk[5][30..].parse().unwrap(),
        })
        .collect()
}

fn monkey_simulation(original_monkeys: &Vec<Monkey>, rounds: i32, divide_by_3: bool) -> usize {
    let mut monkeys: Vec<Monkey> = original_monkeys.clone();
    let mut counter = vec![0; monkeys.len()];
    // compute the lcm of all monkeys' test values. In this particular case,
    // since all values are prime, lcm = product(values)
    let lcm = monkeys.iter().fold(1, |acc, m| acc * m.test);
    for _round in 0..rounds {
        for i in 0..(monkeys.len()) {
            let monkey = &monkeys[i];
            let items = &monkey.items;
            let operation = &monkey.operation;
            let test = monkey.test;
            let test_true = monkey.test_true;
            let test_false = monkey.test_false;

            counter[i] += items.len();
            let (mut trues, mut falses): (Vec<i64>, Vec<i64>) = items
                .iter()
                .map(|i| (operation(*i) / if divide_by_3 { 3 } else { 1 }) % lcm)
                .partition(|v| v % test == 0);

            monkeys[i].items.clear();
            monkeys[test_true].items.append(&mut trues);
            monkeys[test_false].items.append(&mut falses);
        }
    }
    counter.sort();
    counter.reverse();
    counter[0] * counter[1]
}

fn solve_part_a(monkeys: &Vec<Monkey>) -> usize {
    monkey_simulation(monkeys, 20, true)
}

fn solve_part_b(monkeys: &Vec<Monkey>) -> usize {
    monkey_simulation(monkeys, 10000, false)
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

        assert_eq!(10605, solve_part_a(&input));

        assert_eq!(2713310158, solve_part_b(&input));
    }
}
