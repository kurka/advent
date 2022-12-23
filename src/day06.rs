use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/input06.in").unwrap());
    println!("Day 6:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<u8> {
    input.bytes().collect()
}

fn solve_6(input: &Vec<u8>, window_size: usize) -> usize {
    let mut hash_table: Vec<usize> = vec![0; 26];
    let mut distinct_numbers = 0;

    let mut answer: usize = 0;
    // for (i, &c) in input.iter().enumerate() {
    for i in 0..input.len() {
        // add new count to hash
        let hash_new = (input[i] - b'a') as usize;
        if hash_table[hash_new] == 0 {
            distinct_numbers += 1
        }
        hash_table[hash_new] += 1;

        if i >= window_size {
            // remove i-window_size_th char from counts
            let hash_old = (input[i - window_size] - b'a') as usize;
            hash_table[hash_old] -= 1;
            if hash_table[hash_old] == 0 {
                distinct_numbers -= 1
            }
        }

        if distinct_numbers >= window_size {
            answer = i + 1;
            break;
        }
    }

    answer
}

fn solve_part_a(input: &Vec<u8>) -> usize {
    solve_6(input, 4)
}

fn solve_part_b(input: &Vec<u8>) -> usize {
    solve_6(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let samples = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
        ];
        for (sample, answer_a, answer_b) in samples {
            let input = parse_input(sample.to_string());
            assert_eq!(answer_a, solve_part_a(&input));
            assert_eq!(answer_b, solve_part_b(&input));
        }

        // assert_eq!("MCD", solve_part_b(&input));
    }
}
