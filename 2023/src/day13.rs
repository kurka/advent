use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/13.txt").unwrap());
    println!("Day 13:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<Vec<Vec<char>>> {
    input
        .split("\n\n")
        .map(|grid| grid.lines().map(|line| line.chars().collect()).collect())
        .collect()
}

fn solve_part_a(input: &Vec<Vec<Vec<char>>>) -> usize {
    solve_13(input, 0)
}

fn solve_part_b(input: &Vec<Vec<Vec<char>>>) -> usize {
    solve_13(input, 1)
}

fn solve_13(input: &Vec<Vec<Vec<char>>>, tolerance: usize) -> usize {
    input
        .iter()
        .map(|grid| {
            for i in 0..grid.len() - 1 {
                let mut ii = 0;
                let mut mirrors_replaced = 0;
                while i >= ii && i + ii + 1 < grid.len() {
                    let diff = (0..grid[i - ii].len())
                        .filter(|&j| grid[i - ii][j] != grid[i + ii + 1][j])
                        .count();

                    mirrors_replaced += diff;

                    if mirrors_replaced > tolerance {
                        break;
                    }
                    ii += 1;
                }
                // only score if mirrors_replaced is exactly 1
                if mirrors_replaced == tolerance {
                    return 100 * (i + 1);
                }
            }

            for j in 0..(grid[0].len() - 1) {
                let mut jj = 0;
                let mut mirrors_replaced = 0;
                while j >= jj && j + jj + 1 < grid[0].len() {
                    let diff = (0..grid.len())
                        .filter(|&i| grid[i][j - jj] != grid[i][j + 1 + jj])
                        .count();
                    mirrors_replaced += diff;
                    if mirrors_replaced > tolerance {
                        break;
                    }
                    jj += 1;
                }
                if mirrors_replaced == tolerance {
                    return j + 1;
                }
            }
            println!("nothing found");
            for row in grid {
                println!("{row:?}");
            }
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 405);
        assert_eq!(solve_part_b(&input), 400);
    }
}
