use std::{collections::HashSet, fs};

#[derive(Clone, Debug)]
struct Range {
    start: usize,
    end: usize,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/02.txt").unwrap());
    println!("Day 02:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<Range> {
    input
        .trim()
        .split(',')
        .map(|range_str| {
            let (lhs, rhs) = range_str.split_once("-").unwrap();
            Range {
                start: lhs.parse().unwrap(),
                end: rhs.parse().unwrap(),
            }
        })
        .collect()
}

fn solve_part_a(input: &Vec<Range>) -> usize {
    solve_day_2(input, false)
}

fn solve_part_b(input: &Vec<Range>) -> usize {
    solve_day_2(input, true)
}

fn next_repeated(number: usize, repetitions: u32) -> usize {
    let n_digits = number.ilog10() + 1;

    let seed_size = (n_digits / repetitions) as u32 + (n_digits % repetitions != 0) as u32;

    if seed_size * repetitions > n_digits {
        // The smallest number produced with seed has more digits than number,
        // so we generate the lowest possible number with that size.
        // Ex: seed_size = 2, reps = 3 produce 101010
        repeat(10_usize.pow(seed_size - 1), repetitions)
    } else {
        // isolate the seed_size first digits of number as the seed
        // Ex: number=123456, seed_size=2 produce seed=12
        let seed = number / 10_usize.pow(n_digits - seed_size);

        let repeated_seed = repeat(seed, repetitions);
        if repeated_seed > number {
            repeated_seed
        } else {
            repeat(seed + 1, repetitions)
        }
    }
}

fn repeat(seed: usize, repetitions: u32) -> usize {
    let mut result = seed;
    let seed_size = seed.ilog10() + 1;
    for _ in 1..repetitions {
        result = result * 10_usize.pow(seed_size) + seed;
    }
    result
}

fn solve_day_2(input: &Vec<Range>, part_b: bool) -> usize {
    let max_reps = if part_b {
        // max_reps is equal to the number of digits of the max_num in all ranges
        let max_num = input.iter().map(|r| r.end).max().unwrap();
        max_num.ilog10()
    } else {
        2
    };

    let mut invalid_ids_sum = 0;
    for range in input {
        let mut accounted_nums: HashSet<usize> = HashSet::new();
        for rep in 2..=max_reps {
            let mut candidate = next_repeated(range.start - 1, rep);
            while candidate <= range.end {
                if !accounted_nums.contains(&candidate) {
                    invalid_ids_sum += candidate;
                    accounted_nums.insert(candidate);
                } else {
                }
                candidate = next_repeated(candidate, rep);
            }
        }
    }
    invalid_ids_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 1227775554);
        assert_eq!(solve_part_b(&input), 4174379265);
    }
}
