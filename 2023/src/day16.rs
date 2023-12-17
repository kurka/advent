use std::{collections::HashSet, fs};

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/16.txt").unwrap());
    println!("Day 16:");
    println!("{}", solve_part_a(&input));
    // println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn solve_part_a(input: &Vec<Vec<char>>) -> usize {
    let mut visits: HashSet<(usize, usize, char)> = HashSet::new();
    let mut start_pos: Vec<(usize, usize, char)> = vec![(0, 0, 'R')];
    let n_rows = input.len();
    let n_cols = input[0].len();

    while start_pos.len() > 0 {
        let next_start = start_pos.pop();
        let mut cur_pos = next_start;
        while let Some((x, y, dir)) = cur_pos {
            // println!("Visiting {cur_pos:?} {} ", input[x][y]);
            if !visits.insert((x, y, dir)) {
                break;
            }
            cur_pos = match (input[x][y], x, y, dir) {
                ('.', x, y, 'R') => (y + 1 < n_cols).then(|| (x, y + 1, 'R')),
                ('.', x, y, 'L') => (y > 0).then(|| (x, y - 1, 'L')),
                ('.', x, y, 'D') => (x + 1 < n_rows).then(|| (x + 1, y, 'D')),
                ('.', x, y, 'U') => (x > 0).then(|| (x - 1, y, 'U')),
                ('/', x, y, 'R') => (x > 0).then(|| (x - 1, y, 'U')),
                ('/', x, y, 'L') => (x + 1 < n_rows).then(|| (x + 1, y, 'D')),
                ('/', x, y, 'D') => (y > 0).then(|| (x, y - 1, 'L')),
                ('/', x, y, 'U') => (y + 1 < n_cols).then(|| (x, y + 1, 'R')),
                ('\\', x, y, 'R') => (x + 1 < n_rows).then(|| (x + 1, y, 'D')),
                ('\\', x, y, 'L') => (x > 0).then(|| (x - 1, y, 'U')),
                ('\\', x, y, 'D') => (y + 1 < n_cols).then(|| (x, y + 1, 'R')),
                ('\\', x, y, 'U') => (y > 0).then(|| (x, y - 1, 'L')),
                ('|', x, y, 'R') | ('|', x, y, 'L') => {
                    if x > 0 {
                        start_pos.push((x - 1, y, 'U'));
                    }
                    if x + 1 < n_rows {
                        start_pos.push((x + 1, y, 'D'));
                    }
                    None
                }
                ('|', x, y, 'D') => (x + 1 < n_rows).then(|| (x + 1, y, 'D')),
                ('|', x, y, 'U') => (x > 0).then(|| (x - 1, y, 'U')),
                ('-', x, y, 'R') => (y + 1 < n_cols).then(|| (x, y + 1, 'R')),
                ('-', x, y, 'L') => (y > 0).then(|| (x, y - 1, 'L')),
                ('-', x, y, 'U') | ('-', x, y, 'D') => {
                    if y > 0 {
                        start_pos.push((x, y - 1, 'L'));
                    }
                    if x + 1 < n_cols {
                        start_pos.push((x, y + 1, 'R'));
                    }
                    None
                }
                _ => unreachable!(),
            };
        }
    }
    let unique_visits: HashSet<(usize, usize)> =
        visits.into_iter().map(|(x, y, _)| (x, y)).collect();
    unique_visits.len()
}

fn _solve_part_b(input: &Vec<Vec<char>>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 46);
        // assert_eq!(solve_part_b(&input), 1337);
    }
}
