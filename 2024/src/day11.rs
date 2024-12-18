use std::fs;

#[derive(Clone, Debug)]
struct DayOutput {
    foo: usize,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/11.txt").unwrap());
    println!("Day 11:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<usize> {
    input
        .trim()
        .split(" ")
        .map(|c| c.parse().unwrap())
        .collect()
}

fn solve_part_a(input: &Vec<usize>) -> usize {
    solve_day11(input, 25)
}

fn solve_part_b(input: &Vec<usize>) -> usize {
    solve_day11(input, 5)
}

fn solve_day11(input: &Vec<usize>, steps: usize) -> usize {
    let mut cur_vec = input.clone();
    for _ in 0..steps {
        let next_vec: Vec<usize> = cur_vec
            .iter()
            .flat_map(|stone| {
                if *stone == 0 {
                    [Some(1), None]
                } else if ((stone.ilog10() + 1) % 2) == 0 {
                    let power = (stone.ilog10() + 1) / 2;
                    let left = stone / (usize::pow(10, power));
                    [Some(left), Some(stone - left * (usize::pow(10, power)))]
                } else {
                    [Some(stone * 2024), None]
                }
            })
            .flatten()
            .collect();
        cur_vec = next_vec;
        println!("{cur_vec:?}")
    }
    cur_vec.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
125 17
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 55312);
    }
}
