use std::collections::HashSet;
use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/input03.in").unwrap());
    println!("Day 3:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input))
}

fn parse_input(input: String) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

fn score_letter(letter: &u8) -> i32 {
    if *letter >= b'a' {
        (letter - b'a') as i32 + 1
    } else {
        (letter - b'A') as i32 + 27
    }
}

fn solve_part_a(rucksacks: &Vec<String>) -> i32 {
    rucksacks
        .iter()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(bag1, bag2)| {
            let a: HashSet<u8> = HashSet::from_iter(bag1.bytes());
            let b = HashSet::from_iter(bag2.bytes());
            (a.intersection(&b)).map(score_letter).sum::<i32>()
        })
        .sum()
}

fn solve_part_b(rucksacks: &Vec<String>) -> i32 {
    rucksacks
        .chunks(3)
        .map(|chunk| {
            chunk
                .iter()
                .map(|bag| HashSet::<u8>::from_iter(bag.bytes()))
                .reduce(|a, b| a.intersection(&b).copied().collect())
                .unwrap()
                .iter()
                .map(score_letter)
                .sum::<i32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let input = parse_input(sample.to_string());

        assert_eq!(157, solve_part_a(&input));
        assert_eq!(70, solve_part_b(&input));
    }
}
