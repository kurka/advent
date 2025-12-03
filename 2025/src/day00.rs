use std::fs;

#[derive(Clone, Debug)]
struct DayOutput {
    foo: usize,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/00.txt").unwrap());
    println!("Day 00:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> DayOutput {
    DayOutput { foo: 42 }
}

fn solve_part_a(input: &DayOutput) -> usize {
    todo!()
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
Add test input here
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 42);
        assert_eq!(solve_part_b(&input), 1337);
    }
}
