use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/03.txt").unwrap());
    println!("Day 03:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c as usize - '0' as usize).collect())
        .collect()
}

fn solve_part_a(input: &Vec<Vec<usize>>) -> usize {
    input
        .iter()
        .map(|bank| {
            let mut left = 0;
            for i in 1..(bank.len() - 1) {
                if bank[i] > bank[left] {
                    left = i;
                }
            }
            let mut right = left + 1;
            for i in (left + 2)..bank.len() {
                if bank[i] > bank[right] {
                    right = i;
                }
            }
            bank[left] * 10 + bank[right]
        })
        .sum()
}

fn solve_part_b(input: &Vec<Vec<usize>>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

        let input = parse_input(sample.to_string());
        println!("{input:?}");

        assert_eq!(solve_part_a(&input), 357);
        assert_eq!(solve_part_b(&input), 1337);
    }
}
