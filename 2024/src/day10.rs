use std::{collections::HashSet, fs};

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/10.txt").unwrap());
    println!("Day 10:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn solve_part_a(input: &Vec<Vec<u32>>) -> usize {
    solve_day10(input, true)
}

fn solve_part_b(input: &Vec<Vec<u32>>) -> usize {
    solve_day10(input, false)
}

fn solve_day10(input: &Vec<Vec<u32>>, part_a: bool) -> usize {
    let rows = input.len() as i32;
    let cols = input[0].len() as i32;
    let mut trailheads = vec![];
    for i in 0..rows {
        for j in 0..cols {
            if input[i as usize][j as usize] == 0 {
                trailheads.push((i, j));
            }
        }
    }

    trailheads
        .iter()
        .map(|start_pos| {
            let mut paths = 0;
            let mut queue = vec![*start_pos];
            let mut visited = HashSet::new();

            while !queue.is_empty() {
                let node = queue.pop().unwrap();
                let node_val = input[node.0 as usize][node.1 as usize];

                visited.insert(node);
                if node_val == 9 {
                    paths += 1;
                    continue;
                }
                for nei in [
                    (node.0 + 1, node.1),
                    (node.0 - 1, node.1),
                    (node.0, node.1 + 1),
                    (node.0, node.1 - 1),
                ] {
                    if nei.0 < 0
                        || nei.0 >= rows
                        || nei.1 < 0
                        || nei.1 >= cols
                        || (part_a && visited.contains(&nei))
                        || input[nei.0 as usize][nei.1 as usize] != node_val + 1
                    {
                        continue;
                    }
                    queue.push(nei);
                }
            }
            paths
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 36);
        assert_eq!(solve_part_b(&input), 81);
    }
}
