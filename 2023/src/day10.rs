use std::fs;

#[derive(Clone, Debug)]
struct DayOutput {
    foo: usize,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/10.txt").unwrap());
    println!("Day 10:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn solve_part_a(input: &Vec<Vec<char>>) -> usize {
    let (mut start_i, mut start_j) = (0, 0);
    // find start position
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == 'S' {
                (start_i, start_j) = (i, j);
            }
        }
    }

    // find loop start
    let (mut cur_i, mut cur_j) = (0, 0);
    let mut dir = '.'; // U D L R
    'outer: for (ii, jj, ddir, options) in [
        (-1, 0, 'U', ['|', '7', 'F']),
        (0, 1, 'R', ['-', '7', 'J']),
        (1, 0, 'D', ['|', 'L', 'J']),
        (0, -1, 'L', ['-', 'L', 'F']),
    ] {
        if start_i as i32 + ii < 0
            || start_i as i32 + ii >= input.len() as i32
            || start_j as i32 + jj < 0
            || start_j as i32 + jj >= input[start_i].len() as i32
        {
            continue;
        }

        let new_i = (start_i as i32 + ii) as usize;
        let new_j = (start_j as i32 + jj) as usize;
        for opt in options {
            if input[new_i][new_j] == opt {
                (cur_i, cur_j) = (new_i, new_j);
                dir = ddir;
                break 'outer;
            }
        }
    }

    let mut counter = 1;
    while input[cur_i][cur_j] != 'S' {
        counter += 1;
        ((cur_i, cur_j), dir) = match (input[cur_i][cur_j], dir) {
            ('|', 'U') => ((cur_i - 1, cur_j), 'U'),
            ('|', 'D') => ((cur_i + 1, cur_j), 'D'),
            ('-', 'R') => ((cur_i, cur_j + 1), 'R'),
            ('-', 'L') => ((cur_i, cur_j - 1), 'L'),
            ('L', 'D') => ((cur_i, cur_j + 1), 'R'),
            ('L', 'L') => ((cur_i - 1, cur_j), 'U'),
            ('J', 'D') => ((cur_i, cur_j - 1), 'L'),
            ('J', 'R') => ((cur_i - 1, cur_j), 'U'),
            ('7', 'U') => ((cur_i, cur_j - 1), 'L'),
            ('7', 'R') => ((cur_i + 1, cur_j), 'D'),
            ('F', 'U') => ((cur_i, cur_j + 1), 'R'),
            ('F', 'L') => ((cur_i + 1, cur_j), 'D'),
            _ => unreachable!(),
        }
    }

    counter / 2
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
.....
.S-7.
.|.|.
.L-J.
.....
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 4);

        let sample = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...

";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 8);
        // assert_eq!(solve_part_b(&input), 1337);
    }
}
