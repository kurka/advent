use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point3D {
    x: usize,
    y: usize,
    z: usize,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/22.txt").unwrap());
    println!("Day 22:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<(Point3D, Point3D)> {
    let mut parsed_input: Vec<(Point3D, Point3D)> = input
        .lines()
        .map(|line| {
            let (pa, pb) = line.split_once('~').unwrap();
            let pa: Vec<&str> = pa.split(',').collect();
            let pb: Vec<&str> = pb.split(',').collect();
            (
                Point3D {
                    x: pa[0].parse().unwrap(),
                    y: pa[1].parse().unwrap(),
                    z: pa[2].parse().unwrap(),
                },
                Point3D {
                    x: pb[0].parse().unwrap(),
                    y: pb[1].parse().unwrap(),
                    z: pb[2].parse().unwrap(),
                },
            )
        })
        .collect();
    parsed_input.sort_by_key(|(pa, _)| pa.z);
    // checking some dataset properties
    assert!(
        parsed_input
            .iter()
            .all(|(pa, pb)| pa.x <= pb.x && pa.y <= pb.y && pa.z <= pb.z),
        "values are not always increasing in a tupple"
    );
    assert!(
        parsed_input
            .iter()
            .all(|(pa, pb)| [pb.x - pa.x, pb.y - pa.y, pb.z - pa.z]
                .iter()
                .filter(|x| **x != 0)
                .count()
                <= 1),
        "more than one axis changed value in a tuple"
    );
    parsed_input
}

fn solve_part_a(input: &Vec<(Point3D, Point3D)>) -> usize {
    // create a grid representing a 2D floor
    let (mut max_x, mut max_y) = (0, 0);
    for (pa, pb) in input {
        max_x = cmp::max(max_x, pa.x);
        max_x = cmp::max(max_x, pb.x);
        max_y = cmp::max(max_y, pa.y);
        max_y = cmp::max(max_y, pb.y);
    }

    // store top_view_grid as (piece_id, height) tuple grid
    let mut top_view_grid: Vec<Vec<(usize, usize)>> = vec![vec![(0, 0); max_y + 1]; max_x + 1];
    // store the state of each piece (false if free, true if blocking)
    let mut piece_state: HashMap<usize, bool> = input
        .iter()
        .enumerate()
        .map(|(i, _)| (i + 1, false))
        .collect();

    for (i, (pa, pb)) in input.iter().enumerate() {
        let mut possible_collisions: HashSet<(usize, usize)> = HashSet::new();
        // check for possible colisions
        for x in pa.x..=pb.x {
            for y in pa.y..=pb.y {
                // identify either other pieces are below it
                if top_view_grid[x][y].0 != 0 {
                    possible_collisions.insert(top_view_grid[x][y]);
                }
            }
        }
        let max_collision_height = possible_collisions
            .iter()
            .map(|(_, height)| *height)
            .max()
            .unwrap_or(0);
        for x in pa.x..=pb.x {
            for y in pa.y..=pb.y {
                top_view_grid[x][y] = (i + 1, max_collision_height + pb.z - pa.z + 1)
            }
        }
        let real_collisions: Vec<&(usize, usize)> = possible_collisions
            .iter()
            .filter(|(_, height)| *height == max_collision_height)
            .collect();
        if real_collisions.len() == 1 {
            piece_state.insert(real_collisions[0].0, true);
        }
    }
    // for line in top_view_grid {
    //     println!("{line:?}");
    // }

    // println!("{piece_state:?}");

    piece_state
        .iter()
        .filter(|(_, isblocking)| !**isblocking)
        .count()
}

fn solve_part_b(_input: &Vec<(Point3D, Point3D)>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 5);
        // assert_eq!(solve_part_b(&input), 1337);
    }
}
