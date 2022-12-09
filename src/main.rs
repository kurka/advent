use std::fs;

fn main() {
    println!("hello world!");
    solve1();
    solve2();
}


fn solve1() {
    let input = parse1(fs::read_to_string("src/input01.in").unwrap());
    println!("Day 1:");
    println!("{}", day1a(&input));
    println!("{}", day1b(&input))
}

fn parse1(input: String) -> Vec<i32> {
    input
        .lines()
        .map(|l| l.parse().unwrap_or_default())
        .collect()
}

fn day1a(calories_list: &Vec<i32>) -> i32 {
    calories_list
        .split(|c| *c == 0)
        .map(|s| (*s).iter().sum())
        .max()
        .unwrap()
}

fn day1b(calories_list: &Vec<i32>) -> i32 {
    let mut calories: Vec<i32> = calories_list
         .split(|c| *c == 0)
         .map(|s| (*s).iter().sum())
         .collect();
    calories.sort();
    calories.reverse();
    calories[..3].iter().sum()

}

struct RPSPlay {
    first_move: char,
    second_move: char,
}

fn solve2() {
    let input = parse2(fs::read_to_string("src/input02.in").unwrap());
    println!("Day 2:");
    println!("{}", day2a(&input));
    println!("{}", day2b(&input))
}

fn parse2(input: String) -> Vec<RPSPlay> {
    input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|mut line_parts| RPSPlay{first_move: line_parts.next().unwrap().chars().next().unwrap(),
                                  second_move: line_parts.next().unwrap().chars().next().unwrap()})
        .collect()
}

fn day2a(strategy_guide: &Vec<RPSPlay>) -> i32 {
    strategy_guide
        .iter()
        .map(|strategy| match (strategy.first_move, strategy.second_move) {
            ('A', 'X') => 1+3,
            ('A', 'Y') => 2+6,
            ('A', 'Z') => 3+0,
            ('B', 'X') => 1+0,
            ('B', 'Y') => 2+3,
            ('B', 'Z') => 3+6,
            ('C', 'X') => 1+6,
            ('C', 'Y') => 2+0,
            ('C', 'Z') => 3+3,
            (_, _) => panic!(),
        }
        )
        .sum()
}

fn day2b(strategy_guide: &Vec<RPSPlay>) -> i32 {
    strategy_guide
        .iter()
        .map(|strategy| match (strategy.first_move, strategy.second_move) {
            ('A', 'X') => 3+0,
            ('A', 'Y') => 1+3,
            ('A', 'Z') => 2+6,
            ('B', 'X') => 1+0,
            ('B', 'Y') => 2+3,
            ('B', 'Z') => 3+6,
            ('C', 'X') => 2+0,
            ('C', 'Y') => 3+3,
            ('C', 'Z') => 1+6,
            (_, _) => panic!(),
        }
        )
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1() {
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

        let input = parse1(sample.to_string());

        assert_eq!(24000, day1a(&input));
        assert_eq!(45000, day1b(&input));
    }

    #[test]
    fn test_day2() {
        let sample = "\
A Y
B X
C Z";
        let input = parse2(sample.to_string());
        assert_eq!(15, day2a(&input));
        assert_eq!(12, day2b(&input));
}
}
