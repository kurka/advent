use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/14.txt").unwrap());
    println!("Day 14:");
    println!("{}", solve_part_a(&input));
    // println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn solve_part_a(grid: &Vec<Vec<char>>) -> usize {
    let n_rows = grid.len();
    let n_cols = grid[0].len();
    let mut rock_pos = vec![n_rows + 1; n_cols];

    let mut load = 0;
    for i in 0..n_rows {
        for j in 0..n_cols {
            match grid[i][j] {
                'O' => {
                    load += rock_pos[j] - 1;
                    rock_pos[j] -= 1;
                }
                '#' => rock_pos[j] = n_rows - i,
                _ => {}
            }
        }
    }
    load
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
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 136);
        // assert_eq!(solve_part_b(&input), 1337);
    }
}
