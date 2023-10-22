use std::collections::HashSet;
use std::fs;

#[derive(Eq, Hash, PartialEq, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/input13.in").unwrap());
    println!("Day 14:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> HashSet<Coordinate> {
    let mut walls: HashSet<Coordinate> = HashSet::new();
    for line in input.lines() {
        let mut parts = line.split(" -> ");
        let mut start = parse_coordinate(parts.next().unwrap());
        for p in parts {
            let end = parse_coordinate(p);
            let ref_pos = if start.x < end.x { &start } else { &end };
            for x in 0..=(end.x - start.x).abs() {
                walls.insert(Coordinate {
                    x: ref_pos.x + x,
                    y: ref_pos.y,
                });
            }
            let ref_pos = if start.y < end.y { &start } else { &end };
            for y in 0..=(end.y - start.y).abs() {
                walls.insert(Coordinate {
                    x: ref_pos.x,
                    y: ref_pos.y + y,
                });
            }
            start = end;
        }
    }
    println!("{walls:?}");
    println!("{:?}", walls.len());
    walls
}

fn parse_coordinate(coord: &str) -> Coordinate {
    let (x, y) = coord.split_once(",").unwrap();
    Coordinate {
        x: x.parse().unwrap(),
        y: y.parse().unwrap(),
    }
}

fn solve_part_a(_walls: &HashSet<Coordinate>) -> usize {
    todo!()
}

fn solve_part_b(_walls: &HashSet<Coordinate>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9\
";

        let input = parse_input(sample.to_string());

        assert_eq!(13, solve_part_a(&input));
        assert_eq!(140, solve_part_b(&input));
    }
}
