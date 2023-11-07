use regex::Regex;
use std::collections::HashSet;
use std::fs;

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug)]
struct SensorBeacon {
    sensor: Point,
    beacon: Point,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/input15.in").unwrap());
    println!("Day 15:");
    println!("{}", solve_part_a(&input, 2000000));
    // println!("{}", solve_part_b(&input))
}

fn parse_input(input: String) -> Vec<SensorBeacon> {
    let re = Regex::new(
        r"Sensor at x=(?<sx>-?\d+), y=(?<sy>-?\d+): closest beacon is at x=(?<bx>-?\d+), y=(?<by>-?\d+)",
    )
    .unwrap();
    input
        .lines()
        .map(|line| {
            let (_, [sx, sy, bx, by]) = re.captures(line).unwrap().extract();
            SensorBeacon {
                sensor: Point {
                    x: sx.parse().unwrap(),
                    y: sy.parse().unwrap(),
                },
                beacon: Point {
                    x: bx.parse().unwrap(),
                    y: by.parse().unwrap(),
                },
            }
        })
        .collect()
}

fn solve_part_a(input: &Vec<SensorBeacon>, target_row: i32) -> usize {
    let mut no_beacon_pos: HashSet<i32> = HashSet::new();
    for sb in input {
        let budget = (sb.sensor.x - sb.beacon.x).abs() + (sb.sensor.y - sb.beacon.y).abs();
        let vdist = (sb.sensor.y - target_row).abs();

        // println!("{sb:?} {vdist}, {budget}, {no_beacon_pos:?}");
        // if vdist > budget {
        //     continue;
        // }
        for dx in 0..=budget - vdist {
            no_beacon_pos.insert(sb.sensor.x + dx);
            no_beacon_pos.insert(sb.sensor.x - dx);
        }
    }

    for sb in input {
        if sb.beacon.y == target_row {
            no_beacon_pos.remove(&sb.beacon.x);
        }
    }

    no_beacon_pos.len()
}

fn solve_part_b(input: &Vec<SensorBeacon>) -> usize {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3\
";

        let input = parse_input(sample.to_string());

        assert_eq!(26, solve_part_a(&input, 10));
        assert_eq!(0, solve_part_b(&input));
    }
}
