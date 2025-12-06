use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/04.txt").unwrap());
    println!("Day 04:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn solve_part_a(input: &Vec<Vec<char>>) -> usize {
    let rows = input.len();
    let cols = input[0].len();

    let mut res = 0;
    for i in 0..rows {
        for j in 0..cols {
            if input[i][j] == '.' {
                continue;
            }
            let nw = i > 0 && j > 0 && input[i - 1][j - 1] == '@';
            let n = i > 0 && input[i - 1][j] == '@';
            let ne = i > 0 && j < cols - 1 && input[i - 1][j + 1] == '@';
            let e = j < cols - 1 && input[i][j + 1] == '@';
            let se = i < rows - 1 && j < cols - 1 && input[i + 1][j + 1] == '@';
            let s = i < rows - 1 && input[i + 1][j] == '@';
            let sw = i < rows - 1 && j > 0 && input[i + 1][j - 1] == '@';
            let w = j > 0 && input[i][j - 1] == '@';

            let rolls_sum = nw as usize
                + n as usize
                + ne as usize
                + e as usize
                + se as usize
                + s as usize
                + sw as usize
                + w as usize;
            if rolls_sum < 4 {
                res += 1;
            }
        }
    }
    res
}

fn solve_part_b(input: &Vec<Vec<char>>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 13);
        assert_eq!(solve_part_b(&input), 1337);
    }
}
