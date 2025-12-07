use std::{collections::HashSet, fs};

#[derive(Clone, Debug)]
struct DayInput {
    grid: Vec<Vec<char>>,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/07.txt").unwrap());
    println!("Day 07:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> DayInput {
    DayInput {
        grid: input.lines().map(|line| line.chars().collect()).collect(),
    }
}

fn solve_part_a(input: &DayInput) -> usize {
    let grid = &input.grid;
    let rows = grid.len();
    let cols = grid[0].len();

    let initial_beam = grid[0]
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c == 'S')
        .next()
        .unwrap()
        .0;

    let mut active_beams: HashSet<usize> = HashSet::from([initial_beam]);
    let mut splits = 0;

    for i in 0..rows {
        // let mut next_active_beans =
        for j in 0..cols {
            if grid[i][j] == '^' && active_beams.contains(&j) {
                splits += 1;
                active_beams.remove(&j);
                if j > 0 {
                    active_beams.insert(j - 1);
                }
                if j < cols - 1 {
                    active_beams.insert(j + 1);
                }
            }
            // active_beams = next_active_beans.clone();
        }
    }
    splits
}

fn solve_part_b(input: &DayInput) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 21);
        assert_eq!(solve_part_b(&input), 40);
    }
}
