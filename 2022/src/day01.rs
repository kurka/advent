use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/input01.in").unwrap());
    println!("Day 1:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input))
}

fn parse_input(input: String) -> Vec<i32> {
    input
        .lines()
        .map(|l| l.parse().unwrap_or_default())
        .collect()
}

fn solve_part_a(calories_list: &Vec<i32>) -> i32 {
    calories_list
        .split(|c| *c == 0)
        .map(|s| (*s).iter().sum())
        .max()
        .unwrap()
}

fn solve_part_b(calories_list: &Vec<i32>) -> i32 {
    let mut calories: Vec<i32> = calories_list
        .split(|c| *c == 0)
        .map(|s| (*s).iter().sum())
        .collect();
    calories.sort();
    calories.reverse();
    calories[..3].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let input = parse_input(sample.to_string());

        assert_eq!(24000, solve_part_a(&input));
        assert_eq!(45000, solve_part_b(&input));
    }
}
