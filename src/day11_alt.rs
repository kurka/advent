/* I spent a lot of time developing an alternative strategy for day 11
where Items are stored in a fixed array, and monkeys just store a
pointer to which items they currently have access to (actually, they
just store a pointer to the first object they have access to - the other
ones can be found by following a linked list of addresses also stored in
the Item array).

I thought this would be more efficient than the previous solution where
items were memory objects that were "thrown around" at each iteration,
through append operations inside each monkey objects (each monkey had a
vector that constantly increased or decreased in their turns).

However, to my surprise, this new solution ended up being slower than
the original! So, leaving it here to record my efforts, but will leave
the original one as official answer. */

use std::{
    fs,
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

#[derive(Clone)]
struct Monkey {
    operation: Rc<dyn Fn(i64) -> i64>,
    test: i64,
    test_true: usize,
    test_false: usize,
    idx_first_item: Option<usize>,
    idx_last_item: Option<usize>,
    holding_items: usize,
}

#[derive(Clone, Debug)]
struct Item {
    value: i64,
    next_pos: Option<usize>,
}

pub fn solve() {
    let (input, items) = parse_input(fs::read_to_string("inputs/input11.in").unwrap());
    println!("Day 11:");
    println!("{}", solve_part_a(&input, &items));
    println!("{}", solve_part_b(&input, &items))
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

fn parse_input(input: String) -> (Vec<Monkey>, Vec<Item>) {
    let mut items: Vec<Item> = vec![];
    let mut monkeys: Vec<Monkey> = vec![];
    input
        .lines()
        .collect::<Vec<&str>>() // arg! couldn't find a better way to use chunks below! :(
        .chunks(7)
        .for_each(|chunk| {
            // TODO: improve!
            let cur_idx = items.len();
            let m_items: Vec<i64> = chunk[1][18..]
                .split(", ")
                .map(|si| si.parse().unwrap())
                .collect();
            let mut m_items_pos: Vec<Item> = m_items
                .iter()
                .enumerate()
                .map(|(i, item)| Item {
                    value: *item,
                    next_pos: if i < m_items.len() - 1 {
                        Option::Some(cur_idx + i + 1)
                    } else {
                        Option::None
                    },
                })
                .collect();
            let m_items_size = m_items_pos.len();
            items.append(&mut m_items_pos);
            let monkey = Monkey {
                operation: Rc::new(parse_operation(&chunk[2][23..])),
                test: chunk[3][21..].parse().unwrap(),
                test_true: chunk[4][29..].parse().unwrap(),
                test_false: chunk[5][30..].parse().unwrap(),
                idx_first_item: Option::Some(cur_idx),
                idx_last_item: Option::Some(cur_idx + m_items_size - 1),
                holding_items: m_items.len(),
            };
            monkeys.push(monkey);
        });
    // println!("{items:?}");
    (monkeys, items)
}

fn monkey_simulation(
    original_monkeys: &Vec<Monkey>,
    original_items: &Vec<Item>,
    rounds: i32,
    divide_by_3: bool,
) -> usize {
    let mut monkeys: Vec<Monkey> = original_monkeys.clone();
    let mut items: Vec<Item> = original_items.clone();
    let mut counter = vec![0; monkeys.len()];
    // compute the lcm of all monkeys' test values. In this particular case,
    // since all values are prime, lcm = product(values)
    let lcm = monkeys.iter().fold(1, |acc, m| acc * m.test);
    for _round in 0..rounds {
        for i in 0..(monkeys.len()) {
            let operation = monkeys[i].operation.clone();
            let test = monkeys[i].test;
            let test_true = monkeys[i].test_true;
            let test_false = monkeys[i].test_false;

            counter[i] += monkeys[i].holding_items;
            let mut maybe_cur_idx = monkeys[i].idx_first_item.clone();

            while let Some(cur_idx) = maybe_cur_idx {
                let old_value = items[cur_idx].value;
                let new_value = (operation(old_value) / if divide_by_3 { 3 } else { 1 }) % lcm;
                let target_monkey = if new_value % test == 0 {
                    test_true
                } else {
                    test_false
                };
                // move item to target_monkey:
                if monkeys[target_monkey].holding_items == 0 {
                    // if target_monkey didn't have any items, consider it its first item
                    monkeys[target_monkey].idx_first_item = maybe_cur_idx;
                } else {
                    // otherwise, point target_monkey's current last item to the new last item
                    items[monkeys[target_monkey].idx_last_item.unwrap()].next_pos = maybe_cur_idx;
                }
                // regardless of the amount of items in target_monkey, point idx_last_item to this item
                monkeys[target_monkey].idx_last_item = maybe_cur_idx;
                monkeys[target_monkey].holding_items += 1;
                maybe_cur_idx = items[cur_idx].next_pos.clone();
                items[cur_idx] = Item {
                    value: new_value,
                    next_pos: None,
                };
            }

            monkeys[i].idx_first_item = None;
            monkeys[i].idx_last_item = None;
            monkeys[i].holding_items = 0;
        }
    }
    counter.sort();
    counter.reverse();
    counter[0] * counter[1]
}

fn solve_part_a(monkeys: &Vec<Monkey>, items: &Vec<Item>) -> usize {
    monkey_simulation(monkeys, items, 20, true)
}

fn solve_part_b(monkeys: &Vec<Monkey>, items: &Vec<Item>) -> usize {
    monkey_simulation(monkeys, items, 10000, false)
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

        let (input, items) = parse_input(sample.to_string());

        assert_eq!(10605, solve_part_a(&input, &items));

        assert_eq!(2713310158, solve_part_b(&input, &items));
    }
}
