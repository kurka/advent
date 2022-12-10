use std::fs;

struct RPSPlay {
    first_move: char,
    second_move: char,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("src/input02.in").unwrap());
    println!("Day 2:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input))
}

fn parse_input(input: String) -> Vec<RPSPlay> {
    input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|mut line_parts| RPSPlay {
            first_move: line_parts.next().unwrap().chars().next().unwrap(),
            second_move: line_parts.next().unwrap().chars().next().unwrap(),
        })
        .collect()
}

fn solve_part_a(strategy_guide: &Vec<RPSPlay>) -> i32 {
    strategy_guide
        .iter()
        .map(
            |strategy| match (strategy.first_move, strategy.second_move) {
                ('A', 'X') => 1 + 3,
                ('A', 'Y') => 2 + 6,
                ('A', 'Z') => 3 + 0,
                ('B', 'X') => 1 + 0,
                ('B', 'Y') => 2 + 3,
                ('B', 'Z') => 3 + 6,
                ('C', 'X') => 1 + 6,
                ('C', 'Y') => 2 + 0,
                ('C', 'Z') => 3 + 3,
                (_, _) => panic!(),
            },
        )
        .sum()
}

fn solve_part_b(strategy_guide: &Vec<RPSPlay>) -> i32 {
    strategy_guide
        .iter()
        .map(
            |strategy| match (strategy.first_move, strategy.second_move) {
                ('A', 'X') => 3 + 0,
                ('A', 'Y') => 1 + 3,
                ('A', 'Z') => 2 + 6,
                ('B', 'X') => 1 + 0,
                ('B', 'Y') => 2 + 3,
                ('B', 'Z') => 3 + 6,
                ('C', 'X') => 2 + 0,
                ('C', 'Y') => 3 + 3,
                ('C', 'Z') => 1 + 6,
                (_, _) => panic!(),
            },
        )
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
A Y
B X
C Z";
        let input = parse_input(sample.to_string());
        assert_eq!(15, solve_part_a(&input));
        assert_eq!(12, solve_part_b(&input));
    }
}
