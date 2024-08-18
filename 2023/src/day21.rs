use std::collections::HashSet;
use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/21.txt").unwrap());
    println!("Day 21:");
    println!("{}", solve_part_a(&input, 64));
    println!("{}", solve_part_b(&input, 26501365));
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn solve_part_a(grid: &Vec<Vec<char>>, steps: usize) -> usize {
    count_paths(grid, steps, false)
}

fn solve_part_b(grid: &Vec<Vec<char>>, steps: usize) -> usize {
    count_paths(grid, steps, true)
}

fn count_paths(grid: &Vec<Vec<char>>, steps: usize, infinite_grid: bool) -> usize {
    let n_rows = grid.len() as i32;
    let n_cols = grid[0].len() as i32;

    // find positions of S
    let mut start_pos: (i32, i32) = (0, 0);
    for i in 0..n_rows as usize {
        for j in 0..n_cols as usize {
            if grid[i][j] == 'S' {
                start_pos = (i as i32, j as i32);
            }
        }
    }

    let even_visits: &mut HashSet<(i32, i32)> = &mut HashSet::new();
    let odd_visits: &mut HashSet<(i32, i32)> = &mut HashSet::new();

    let mut frontier = vec![start_pos];
    for step in 0..steps {
        println!(
            "Step {}, front: {}, odd: {} even: {}",
            step,
            frontier.len(),
            odd_visits.len(),
            even_visits.len()
        );
        let mut new_frontier = vec![];
        for (rfront, cfront) in frontier {
            for (rdiff, cdiff) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                if !infinite_grid
                    && ((rdiff == -1 && rfront == 0)
                        || (rdiff == 1 && rfront == n_rows - 1)
                        || (cdiff == -1 && cfront == 0)
                        || (cdiff == 1 && cfront == n_cols - 1))
                {
                    // position is out of bounds
                    continue;
                }
                let nei_pos = (rfront + rdiff, cfront + cdiff);
                // println!(
                //     "{} {} {} {}",
                //     rfront + rdiff,
                //     (rfront + rdiff).rem_euclid(n_rows),
                //     cfront + cdiff,
                //     (cfront + cdiff).rem_euclid(n_cols)
                // );
                if grid[nei_pos.0.rem_euclid(n_rows) as usize]
                    [nei_pos.1.rem_euclid(n_cols) as usize]
                    == '#'
                {
                    // hit a rock
                    continue;
                }

                let visits_set: &mut HashSet<(i32, i32)> = if step % 2 == 0 {
                    odd_visits
                } else {
                    even_visits
                };

                if !visits_set.contains(&nei_pos) {
                    visits_set.insert(nei_pos);
                    new_frontier.push(nei_pos);
                }
            }
        }
        frontier = new_frontier;
    }
    if steps % 2 == 0 {
        even_visits.len()
    } else {
        odd_visits.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input, 6), 16);
        assert_eq!(solve_part_b(&input, 6), 16);
        assert_eq!(solve_part_b(&input, 50), 1594);
        assert_eq!(solve_part_b(&input, 100), 6536);
        assert_eq!(solve_part_b(&input, 500), 167004);
        assert_eq!(solve_part_b(&input, 1000), 668697);
        assert_eq!(solve_part_b(&input, 5000), 16733044);
    }
}
