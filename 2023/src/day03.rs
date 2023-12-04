use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/03.txt").unwrap());
    println!("Day 03:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn solve_part_a(input: &Vec<Vec<char>>) -> u32 {
    let mut total_sum = 0;
    let mut i = 0;
    while i < input.len() {
        let mut j = 0;
        while j < input[i].len() {
            if input[i][j].is_ascii_digit() {
                // get the complete number starting at j
                let start_j = j;
                let mut number = 0;
                while j < input[i].len() && input[i][j].is_ascii_digit() {
                    number = 10 * number + input[i][j].to_digit(10).unwrap();
                    j += 1;
                }
                let end_j = j - 1;
                // search for a symbol around the number
                let mut has_surrounding_symbol = false;
                for ii in (max(i as i32 - 1, 0) as usize)..=min(i + 1, input.len() - 1) {
                    for jj in
                        (max(start_j as i32 - 1, 0) as usize)..=min(end_j + 1, input[i].len() - 1)
                    {
                        if !input[ii][jj].is_ascii_digit() && input[ii][jj] != '.' {
                            has_surrounding_symbol = true;
                        }
                    }
                }
                if has_surrounding_symbol {
                    total_sum += number;
                }
            } else {
                // not a number, just advance a column
                j += 1;
            }
        }
        i += 1;
    }
    total_sum
}

fn solve_part_b(input: &Vec<Vec<char>>) -> u32 {
    // for every symbol found, store its coordinates as a map keys, and its neighboring numbers as values
    let mut symbols_map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let mut i = 0;
    while i < input.len() {
        let mut j = 0;
        while j < input[i].len() {
            if input[i][j].is_ascii_digit() {
                // get the complete number starting at j
                let start_j = j;
                let mut number = 0;
                while j < input[i].len() && input[i][j].is_ascii_digit() {
                    number = 10 * number + input[i][j].to_digit(10).unwrap();
                    j += 1;
                }
                let end_j = j - 1;
                // search for a symbol around the number
                for ii in (max(i as i32 - 1, 0) as usize)..=min(i + 1, input.len() - 1) {
                    for jj in
                        (max(start_j as i32 - 1, 0) as usize)..=min(end_j + 1, input[i].len() - 1)
                    {
                        if input[ii][jj] == '*' {
                            // map symbol to neighbor number
                            let symbol_neis = symbols_map.entry((ii, jj)).or_default();
                            symbol_neis.push(number);
                        }
                    }
                }
            } else {
                // not a number, just advance a column
                j += 1;
            }
        }
        i += 1;
    }
    symbols_map
        .values()
        .map(|neis| {
            if neis.len() == 2 {
                neis.iter().product()
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 4361);
        assert_eq!(solve_part_b(&input), 467835);
    }
}
