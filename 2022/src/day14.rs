use std::collections::HashSet;
use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/input14.in").unwrap());
    println!("Day 14:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> HashSet<(i32, i32)> {
    let mut walls: HashSet<(i32, i32)> = HashSet::with_capacity(50000);
    for line in input.lines() {
        let mut parts = line.split(" -> ");
        let mut start = parse_coordinate(parts.next().unwrap());
        for p in parts {
            let end = parse_coordinate(p);
            let ref_pos = if start.0 < end.0 { &start } else { &end };
            for x in 0..=(end.0 - start.0).abs() {
                walls.insert((ref_pos.0 + x, ref_pos.1));
            }
            let ref_pos = if start.1 < end.1 { &start } else { &end };
            for y in 0..=(end.1 - start.1).abs() {
                walls.insert((ref_pos.0, ref_pos.1 + y));
            }
            start = end;
        }
    }
    walls
}

fn parse_coordinate(coord: &str) -> (i32, i32) {
    let (x, y) = coord.split_once(",").unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

fn solve_part_a(walls: &HashSet<(i32, i32)>) -> usize {
    solve_14(walls, true)
}

fn solve_part_b(walls: &HashSet<(i32, i32)>) -> usize {
    solve_14(walls, false)
}

fn solve_14(walls: &HashSet<(i32, i32)>, part_a: bool) -> usize {
    let mut max_y = 0;
    for c in walls {
        if c.1 > max_y {
            max_y = c.1;
        }
    }
    let solids = &mut walls.clone();
    let start = (500, 0);
    let mut count = 0;
    loop {
        let added = drop(start, solids, max_y, part_a);
        // println!("{count} {added}");

        count += added;

        if added == 0 || solids.contains(&start) {
            break;
        }
    }
    /*
    let mut min_x = 1000000000;
    let mut max_x = 0;
    for c in solids.iter() {
        if c.0 > max_x {
            max_x = c.0
        }
        if c.0 < min_x {
            min_x = c.0
        }
    }
    for y in 0..=max_y + 2 {
        for x in min_x..=max_x {
            if walls.contains(&(x, y)) {
                print!("#");
            } else if solids.contains(&(x, y)) {
                print!("o");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    */

    count
}

fn drop(sand: (i32, i32), solids: &mut HashSet<(i32, i32)>, max_y: i32, part_a: bool) -> usize {
    //println!("{:?} {:?}", sand, max_y);
    if part_a && sand.1 > max_y {
        return 0;
    }

    if !part_a && sand.1 == max_y + 1 {
        solids.insert(sand);
        return 1;
    }

    if !solids.contains(&(sand.0, sand.1 + 1)) {
        return drop((sand.0, sand.1 + 1), solids, max_y, part_a);
    }

    let mut added = 0;
    if !solids.contains(&(sand.0 - 1, sand.1 + 1)) {
        let added_left = drop((sand.0 - 1, sand.1 + 1), solids, max_y, part_a);
        if added_left == 0 {
            return 0;
        }
        added += added_left;
    }
    if solids.contains(&(sand.0 - 1, sand.1 + 1)) && !solids.contains(&(sand.0 + 1, sand.1 + 1)) {
        let added_right = drop((sand.0 + 1, sand.1 + 1), solids, max_y, part_a);
        if added_right == 0 {
            return 0;
        }
        added += added_right;
    }

    if solids.contains(&(sand.0 - 1, sand.1 + 1))
        && solids.contains(&(sand.0, sand.1 + 1))
        && solids.contains(&(sand.0 + 1, sand.1 + 1))
    {
        solids.insert(sand);
        added += 1;
    }
    return added;
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

        assert_eq!(24, solve_part_a(&input));
        assert_eq!(93, solve_part_b(&input));
    }
}
