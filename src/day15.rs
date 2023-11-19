use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
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
    println!("{}", solve_part_b(&input, 4000000));
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

fn solve_part_a(input: &Vec<SensorBeacon>, target_row: i64) -> usize {
    let mut no_beacon_pos: HashSet<i64> = HashSet::new();
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

fn solve_part_b(input: &Vec<SensorBeacon>, max_c: i64) -> i64 {
    let diamonds: Vec<(Point, Point, Point, Point)> = input
        .iter()
        .map(|sb| {
            let budget = (sb.sensor.x - sb.beacon.x).abs() + (sb.sensor.y - sb.beacon.y).abs();
            (
                Point {
                    x: sb.sensor.x,
                    y: sb.sensor.y + budget + 1,
                },
                Point {
                    x: sb.sensor.x + budget + 1,
                    y: sb.sensor.y,
                },
                Point {
                    x: sb.sensor.x,
                    y: sb.sensor.y - budget - 1,
                },
                Point {
                    x: sb.sensor.x - budget - 1,
                    y: sb.sensor.y,
                },
            )
        })
        .collect();

    let mut intersections = vec![];
    for d1 in &diamonds {
        for d2 in &diamonds {
            if d1 == d2 {
                continue;
            }
            let (d1, r1, u1, l1) = d1;
            let (d2, r2, u2, l2) = d2;
            println!("{:?} {:?}", (u1, r1), (l2, u2));
            println!("fi1: {:?}", find_intersection((u1, r1), (l2, u2)));
            println!("fi2: {:?}", find_intersection2((u1, r1), (l2, u2)));
            if let Some(i1) = find_intersection((u1, r1), (l2, u2)) {
                intersections.push(i1);
            }
            if let Some(i2) = find_intersection((l1, d1), (d2, r2)) {
                intersections.push(i2);
            }
        }
    }

    let res: Vec<&Point> = intersections
        .iter()
        .filter(|i| {
            if i.x < 0 || i.x > max_c || i.y < 0 || i.y > max_c {
                return false;
            }
            for sb in input {
                let budget = (sb.sensor.x - sb.beacon.x).abs() + (sb.sensor.y - sb.beacon.y).abs();
                if (sb.sensor.x - i.x).abs() + (sb.sensor.y - i.y).abs() <= budget {
                    return false;
                }
            }
            return true;
        })
        .collect();

    println!("Res: {res:?}");
    let r = res.get(0).unwrap();
    return r.x * 4000000 + r.y;
}

fn find_intersection(l1: (&Point, &Point), l2: (&Point, &Point)) -> Option<Point> {
    let (Point { x: x1, y: y1 }, Point { x: x2, y: y2 }) = l1;
    let (Point { x: x3, y: y3 }, Point { x: x4, y: y4 }) = l2;

    // x1 = 14
    // x2 = 16
    // x3 = 12
    // x4 = 20
    //
    // y1 = 1
    // y2 = 3
    // y3 = 1
    // y4 = -7
    //
    //    1 1 1 1 2
    //    2 4 6 8 0
    // -7 ........x
    // -6 .......x.
    // -5 ......x..
    // -4 .#...x...
    // -3 ..#.x....
    // -2 ...x.....
    // -1 ..x.#....
    //  0 .x...#...
    //  1 x.o...#..
    //  2 ...o...#.
    //  3 ....o...#
    //  4 .........#
    //  5 .........
    //
    // narrow down intersection area
    let x_start = max(*x1, *x3); //14
    let x_end = min(*x2, *x4); // 16
    let y_start = max(*y1, *y4); // 1
    let y_end = min(*y2, *y3); // 1

    if !((x_start, y_start) == (*x1, *y1) || (x_start, y_end) == (*x3, *y3))
        || x_start > x_end
        || y_start > y_end
    {
        // segments don't have intersection
        return None;
    }

    let delta_x = x_end - x_start;
    let delta_y = y_end - y_start;

    if delta_x % 2 != 0 || delta_y % 2 != 0 {
        // intersection happens in fractional coordinates, not in the grid
        return None;
    }
    if delta_x <= delta_y {
        Some(Point {
            x: x_start + delta_x / 2,
            y: y_start + delta_x / 2,
        })
    } else {
        Some(Point {
            x: x_start + delta_y / 2,
            y: y_start + delta_y / 2,
        })
    }
}

fn find_intersection2(l1: (&Point, &Point), l2: (&Point, &Point)) -> Option<Point> {
    let (Point { x: x1, y: y1 }, Point { x: x2, y: y2 }) = l1;
    let (Point { x: x3, y: y3 }, Point { x: x4, y: y4 }) = l2;

    let px = ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4))
        / ((x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4));
    let py = ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4))
        / ((x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4));

    if px >= *x1
        && px <= *x2
        && py >= *y1
        && py <= *y2
        && px >= *x3
        && px <= *x4
        && py >= *y4
        && py <= *y4
    {
        return Some(Point { x: px, y: py });
    } else {
        return None;
    }
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
        assert_eq!(56000011, solve_part_b(&input, 20));
    }
}
