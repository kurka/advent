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
    solve_day_3(input, 2)
}

fn solve_part_b(input: &Vec<Vec<usize>>) -> usize {
    solve_day_3(input, 12)
}

fn solve_day_3(input: &Vec<Vec<usize>>, total_batteries: usize) -> usize {
    input
        .iter()
        .map(|bank| {
            let mut joltage = vec![];
            let mut start = 0;
            for b in 0..total_batteries {
                let mut sel = start;
                for i in (start + 1)..(bank.len() - total_batteries + b + 1) {
                    // greedly select the leftmost highest number in the bank.
                    if bank[i] > bank[sel] {
                        sel = i;
                    }
                }
                start = sel + 1;
                joltage.push(bank[sel])
            }
            joltage
                .into_iter()
                .reduce(|result, num| result * 10 + num)
                .unwrap()
        })
        .sum()
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
        assert_eq!(solve_part_b(&input), 3121910778619);
    }
}
