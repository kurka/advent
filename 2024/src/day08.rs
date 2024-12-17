use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Clone, Debug)]
struct Day08 {
    antennas_map: HashMap<char, Vec<(i32, i32)>>,
    grid_rows: i32,
    grid_cols: i32,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/08.txt").unwrap());
    println!("Day 08:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Day08 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let grid_rows = grid.len() as i32;
    let grid_cols = grid[0].len() as i32;
    let mut antennas_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for i in 0..grid_rows {
        for j in 0..grid_cols {
            let gchar = grid[i as usize][j as usize];
            if gchar != '.' {
                antennas_map
                    .entry(gchar)
                    .and_modify(|v| v.push((i, j)))
                    .or_insert(vec![(i, j)]);
            }
        }
    }
    Day08 {
        antennas_map,
        grid_rows,
        grid_cols,
    }
}

fn solve_part_a(input: &Day08) -> usize {
    solve_day08(input, false)
}
fn solve_part_b(input: &Day08) -> usize {
    solve_day08(input, true)
}

fn solve_day08(input: &Day08, part_b: bool) -> usize {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for group in input.antennas_map.values() {
        for a in 0..group.len() {
            for b in a + 1..group.len() {
                let ant_a = group[a];
                let ant_b = group[b];
                let (left, right) = if ant_a.0 <= ant_b.0 {
                    (ant_a, ant_b)
                } else {
                    (ant_b, ant_a)
                };

                for i in if part_b { 0.. } else { 1.. } {
                    if left.0 - i * (right.0 - left.0) < 0
                        || left.0 - i * (right.0 - left.0) >= input.grid_rows
                        || left.1 - i * (right.1 - left.1) < 0
                        || left.1 - i * (right.1 - left.1) >= input.grid_cols
                    {
                        break;
                    }
                    antinodes.insert((
                        left.0 - i * (right.0 - left.0),
                        left.1 - i * (right.1 - left.1),
                    ));

                    if !part_b {
                        break;
                    }
                }

                for i in if part_b { 0.. } else { 1.. } {
                    if right.0 + i * (right.0 - left.0) < 0
                        || right.0 + i * (right.0 - left.0) >= input.grid_rows
                        || right.1 + i * (right.1 - left.1) < 0
                        || right.1 + i * (right.1 - left.1) >= input.grid_cols
                    {
                        break;
                    }
                    antinodes.insert((
                        right.0 + i * (right.0 - left.0),
                        right.1 + i * (right.1 - left.1),
                    ));

                    if !part_b {
                        break;
                    }
                }
            }
        }
    }
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

        // ['#', '#', '.', '.', '.', '.', '#', '.', '.', '.', '.', '#']
        // ['.', '#', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.']
        // ['.', '.', '#', '.', '#', '.', '.', '.', '.', '.', '#', '.']
        // ['.', '.', '#', '#', '.', '.', '.', '.', '.', '.', '.', '.']
        // ['.', '.', '.', '.', '#', '.', '.', '.', '.', '#', '.', '.']
        // ['.', '#', '.', '.', '.', '#', '#', '.', '.', '.', '.', '#']
        // ['.', '.', '.', '#', '.', '.', '#', '.', '.', '.', '.', '.']
        // ['#', '.', '.', '.', '.', '#', '.', '#', '.', '.', '.', '.']
        // ['.', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.']
        // ['.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.']
        // ['.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.']
        // ['.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '#', '#']

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 14);
        assert_eq!(solve_part_b(&input), 34);
    }
}
