use std::collections::HashMap;
use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/01.txt").unwrap());
    println!("Day 01:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            line.split_once("   ")
                .map(|num| (num.0.parse::<i32>().unwrap(), num.1.parse::<i32>().unwrap()))
                .unwrap()
        })
        .unzip()
}

fn solve_part_a(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut left_list = input.0.clone();
    let mut right_list = input.1.clone();
    left_list.sort();
    right_list.sort();

    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(ea, eb)| (eb - ea).abs())
        .sum()
}

fn solve_part_b(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut left_counter: HashMap<i32, i32> = HashMap::new();
    let mut right_counter: HashMap<i32, i32> = HashMap::new();

    for el in &input.0 {
        left_counter
            .entry(*el)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    for el in &input.1 {
        right_counter
            .entry(*el)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    left_counter
        .iter()
        .map(|(e, count)| count * e * right_counter.get(e).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
        let sample = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 11);
        assert_eq!(solve_part_b(&input), 31);
    }
}
