use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/06.txt").unwrap());
    println!("Day 06:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> String {
    input
}

fn solve_part_a(input: &String) -> usize {
    let lines: Vec<&str> = input.lines().collect();

    let operations: Vec<char> = lines[lines.len() - 1]
        .split_ascii_whitespace()
        .map(|op| match op {
            "*" => '*',
            "+" => '+',
            _ => panic!(),
        })
        .collect();
    let n_cols = operations.len();

    let mut cols: Vec<Vec<usize>> = (0..n_cols).map(|_| vec![]).collect();
    for line in &lines[0..(lines.len() - 1)] {
        for (i, number) in line.split_ascii_whitespace().enumerate() {
            cols[i].push(number.parse().unwrap());
        }
    }

    cols.iter()
        .zip(operations.iter())
        .map(|(col, op)| match op {
            '*' => col.iter().product::<usize>(),
            '+' => col.iter().sum(),
            _ => panic!(),
        })
        .sum()
}

fn solve_part_b(input: &String) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut col = cols - 1;
    let mut buffer = vec![];
    let mut total = 0;
    loop {
        let mut digit: usize = 0;
        for row in 0..rows - 1 {
            if grid[row][col] != ' ' {
                digit = 10 * digit + (grid[row][col] as u8 - b'0') as usize
            }
        }
        buffer.push(digit);
        match grid[rows - 1][col] {
            '*' => {
                total += buffer.iter().product::<usize>();
                buffer.clear();
                if col == 0 {
                    break;
                }
                col -= 2
            }
            '+' => {
                total += buffer.iter().sum::<usize>();
                buffer.clear();
                if col == 0 {
                    break;
                }
                col -= 2
            }
            _ => {
                col -= 1;
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 4277556);
        assert_eq!(solve_part_b(&input), 3263827);
    }
}
