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
    let target = ['X', 'M', 'A', 'S'];
    let rows = input.len() as i32;
    let cols = input[0].len() as i32;

    let mut count = 0;
    for i in 0..rows {
        for j in 0..cols {
            for delta in [
                (0, -1),
                (0, 1),
                (1, 0),
                (-1, 0),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
            ] {
                let mut found = true;
                for c in 0..target.len() as i32 {
                    let ii = i + c * delta.0;
                    let jj = j + c * delta.1;
                    if ii < 0
                        || ii >= rows
                        || jj < 0
                        || jj >= cols
                        || input[ii as usize][jj as usize] != target[c as usize]
                    {
                        found = false;
                        break;
                    }
                }
                if found {
                    count += 1
                }
            }
        }
    }
    count
}

fn solve_part_b(input: &Vec<Vec<char>>) -> usize {
    let rows = input.len();
    let cols = input[0].len();

    let mut count = 0;
    for i in 1..(rows - 1) {
        for j in 1..(cols - 1) {
            if input[i][j] == 'A'
                && ((input[i - 1][j - 1] == 'M'
                    && input[i - 1][j + 1] == 'M'
                    && input[i + 1][j + 1] == 'S'
                    && input[i + 1][j - 1] == 'S')
                    || (input[i - 1][j - 1] == 'S'
                        && input[i - 1][j + 1] == 'M'
                        && input[i + 1][j + 1] == 'M'
                        && input[i + 1][j - 1] == 'S')
                    || (input[i - 1][j - 1] == 'S'
                        && input[i - 1][j + 1] == 'S'
                        && input[i + 1][j + 1] == 'M'
                        && input[i + 1][j - 1] == 'M')
                    || (input[i - 1][j - 1] == 'M'
                        && input[i - 1][j + 1] == 'S'
                        && input[i + 1][j + 1] == 'S'
                        && input[i + 1][j - 1] == 'M'))
            {
                count += 1
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 18);
        assert_eq!(solve_part_b(&input), 9);
    }
}
