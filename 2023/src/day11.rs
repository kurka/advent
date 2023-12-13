use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/11.txt").unwrap());
    println!("Day 11:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(j, c)| (c == '#').then_some((i, j)))
        })
        .collect()
}

fn solve_part_a(input: &Vec<(usize, usize)>) -> i64 {
    solve_11(input, 2)
}

fn solve_part_b(input: &Vec<(usize, usize)>) -> i64 {
    solve_11(input, 1000000)
}

fn solve_11(input: &Vec<(usize, usize)>, universe_age: usize) -> i64 {
    // first adjust the coordinates considering empty gaps
    let mut galaxies = input.clone();

    // galaxies is already ordered by row
    let mut last_filled_row = galaxies[0].0;
    let mut offset = 0;
    for i in 0..galaxies.len() {
        let galaxy_row = galaxies[i].0;
        if galaxy_row > last_filled_row {
            offset += (universe_age - 1) * (galaxy_row - last_filled_row - 1);
            last_filled_row = galaxy_row;
        }
        galaxies[i].0 += offset;
    }

    // sort by column, and add empty cols
    galaxies.sort_by_key(|g| g.1);
    let mut last_filled_col = galaxies[0].1;
    let mut offset = 0;
    for i in 0..galaxies.len() {
        let galaxy_col = galaxies[i].1;
        if galaxy_col > last_filled_col {
            offset += (universe_age - 1) * (galaxy_col - last_filled_col - 1);
            last_filled_col = galaxy_col;
        }
        galaxies[i].1 += offset;
    }

    // galaxies.sort_by_key(|g| g.0);

    // compute all distances
    galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, galaxy_i)| {
            galaxies.iter().take(i).map(|galaxy_j| {
                (galaxy_i.0 as i64 - galaxy_j.0 as i64).abs()
                    + (galaxy_i.1 as i64 - galaxy_j.1 as i64).abs()
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 374);
        assert_eq!(solve_11(&input, 10), 1030);
        assert_eq!(solve_11(&input, 100), 8410);
    }
}
