use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[derive(Clone, Debug)]
struct DayOutput {
    foo: usize,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/23.txt").unwrap());
    println!("Day 23:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn solve_part_a(grid: &Vec<Vec<char>>) -> usize {
    let n_rows = grid.len();
    let n_cols = grid[0].len();
    let mut start_col = 0;
    for (i, c) in grid[0].iter().enumerate() {
        if *c == '.' {
            start_col = i
        }
    }

    let mut goal_row = 0;
    for (i, c) in grid[n_rows - 1].iter().enumerate() {
        if *c == '.' {
            goal_row = i
        }
    }

    let left = |(x, y): (usize, usize), forward: bool| -> Option<(usize, usize)> {
        if y > 0 && (grid[x][y - 1] == '.' || grid[x][y - 1] == if forward { '<' } else { '>' }) {
            Some((x, y - 1))
        } else {
            None
        }
    };
    let right = |(x, y): (usize, usize), forward: bool| -> Option<(usize, usize)> {
        if y < n_cols - 1
            && (grid[x][y + 1] == '.' || grid[x][y + 1] == if forward { '>' } else { '<' })
        {
            Some((x, y + 1))
        } else {
            None
        }
    };
    let up = |(x, y): (usize, usize), forward: bool| -> Option<(usize, usize)> {
        if x > 0 && (grid[x - 1][y] == '.' || grid[x - 1][y] == if forward { '^' } else { 'v' }) {
            Some((x - 1, y))
        } else {
            None
        }
    };
    let down = |(x, y): (usize, usize), forward: bool| -> Option<(usize, usize)> {
        if x < n_rows - 1
            && (grid[x + 1][y] == '.' || grid[x + 1][y] == if forward { 'v' } else { '^' })
        {
            Some((x + 1, y))
        } else {
            None
        }
    };

    let count_incoming_paths = |(x, y): (usize, usize)| -> usize {
        let mut valves = 0;
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let xx = x as i32 + dx;
            let yy = y as i32 + dy;
            if xx >= 0
                && xx < n_rows as i32
                && yy >= 0
                && yy < n_cols as i32
                && grid[xx as usize][yy as usize] != '#'
            {
                match (grid[xx as usize][yy as usize], dx, dy) {
                    ('v', -1, 0) => valves += 1,
                    ('^', 1, 0) => valves += 1,
                    ('>', 0, -1) => valves += 1,
                    ('<', 0, 1) => valves += 1,
                    _ => continue,
                }
            } else {
                continue;
            }
        }
        if valves == 0 {
            1
        } else {
            valves
        }
    };

    let mut queue: Vec<((usize, usize), usize)> = vec![((0, start_col), 0)];
    let mut visits: HashSet<(usize, usize)> = HashSet::new();
    let mut node2dists: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    while queue.len() > 0 {
        let (node, qdist) = queue.remove(0);

        let dists = node2dists
            .entry(node)
            .and_modify(|v| v.push(qdist))
            .or_insert(vec![qdist]);

        if count_incoming_paths(node) > dists.len() {
            println!("partial {node:?} {dists:?}");
            continue;
        }

        let dist = *dists.iter().max().unwrap();
        println!(
            "Decision: {node:?} {} {dists:?} {dist}",
            count_incoming_paths(node)
        );
        // println!("Visiting {node:?} {dist}");
        if node == (n_rows - 1, goal_row) {
            return dist;
        }
        // println!(
        //     "l{:?} r{:?} u{:?} d{:?}",
        //     left(node),
        //     right(node),
        //     up(node),
        //     down(node)
        // );
        visits.insert(node);
        for nei in [
            left(node, true),
            right(node, true),
            up(node, true),
            down(node, true),
        ]
        .iter()
        .filter_map(|n| *n)
        {
            if visits.contains(&nei) {
                continue;
            }
            queue.push((nei, dist + 1));
        }
    }
    unreachable!()
}

fn solve_part_b(grid: &Vec<Vec<char>>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 94);
        // assert_eq!(solve_part_b(&input), 154);
    }
}
