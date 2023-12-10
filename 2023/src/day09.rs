use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/09.txt").unwrap());
    println!("Day 09:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.split(" ").map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn solve_part_a(input: &Vec<Vec<i32>>) -> i32 {
    input
        .iter()
        .map(|seq| {
            let mut all_seqs: Vec<Vec<i32>> = vec![seq.clone()];
            while !all_seqs.last().unwrap().iter().all(|&v| v == 0) {
                // let sub_seq = derived_seq(&cur_seq);
                let sub_seq = derived_seq(&all_seqs.last().unwrap());
                all_seqs.push(sub_seq);
            }

            let mut last_els: Vec<i32> = vec![0];
            let n_sub_seqs = all_seqs.len();
            for i in (0..=(n_sub_seqs - 2)).rev() {
                let next = all_seqs[i].last().unwrap() + last_els[n_sub_seqs - i - 2];
                last_els.push(next);
            }
            *last_els.last().unwrap()
        })
        .sum::<i32>()
}

fn derived_seq(seq: &Vec<i32>) -> Vec<i32> {
    (1..seq.len()).map(|i| seq[i] - seq[i - 1]).collect()
}

fn solve_part_b(input: &Vec<Vec<i32>>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 114);
        // assert_eq!(solve_part_b(&input), 1337);
    }
}
