use regex::Regex;
use std::cmp::max;
use std::collections::HashSet;
use std::fs;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy, Debug)]
struct Range {
    start: i64,
    end: i64,
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
    // store the ranges of invalid areas for each sensor
    let mut ranges: Vec<Range> = input
        .iter()
        .filter_map(|sb| {
            let budget = (sb.sensor.x - sb.beacon.x).abs() + (sb.sensor.y - sb.beacon.y).abs();
            let vdist = (sb.sensor.y - target_row).abs();

            let dx = budget - vdist;

            if dx >= 0 {
                Some(Range {
                    start: sb.sensor.x - dx,
                    end: sb.sensor.x + dx,
                })
            } else {
                None
            }
        })
        .collect();
    ranges.sort_by_key(|r| r.start);

    // merge ranges
    let merged_ranges = ranges
        .iter()
        .fold(vec![], |mut merged_ranges: Vec<Range>, cur_range| {
            let last_range_maybe = merged_ranges.pop();
            if last_range_maybe.is_none() {
                merged_ranges.push(*cur_range)
            } else {
                let last_range = last_range_maybe.unwrap();
                if last_range.end >= cur_range.start {
                    merged_ranges.push(Range {
                        start: last_range.start,
                        end: max(last_range.end, cur_range.end),
                    });
                } else {
                    merged_ranges.push(last_range);
                    merged_ranges.push(*cur_range);
                }
            }
            merged_ranges
        });

    // count number of busy cells
    let busy_row_count = merged_ranges.iter().fold(0, |range_sum, range| {
        range_sum + range.end - range.start + 1
    });

    // get beacons in row, to discount it from the total sum
    let beacons_in_row_count: HashSet<i64> = input
        .iter()
        .filter(|sb| sb.beacon.y == target_row)
        .map(|sb| sb.beacon.x)
        .collect();

    (busy_row_count as usize) - beacons_in_row_count.len()
}

fn solve_part_b(input: &Vec<SensorBeacon>, max_c: i64) -> i64 {
    // We know that there is just one position in the grid where the beacon can
    // be. This fact simplifies a lot the solution.
    //
    //
    // given it's a single beacon, it should lie within the diamond formed by a
    // sensor perimeter+1. Otherwise, if it was in perimeter+n, it would be
    // possible to add beacons to perimeter+n-1, ... perimeter+1. So, we get the
    // 4 diamond lines for each sensor, and store it as a tuple up, right, down, left
    let diamonds: Vec<(Point, Point, Point, Point)> = input
        .iter()
        .map(|sb| {
            let budget = (sb.sensor.x - sb.beacon.x).abs() + (sb.sensor.y - sb.beacon.y).abs();
            (
                Point {
                    // down
                    x: sb.sensor.x,
                    y: sb.sensor.y + budget + 1,
                },
                Point {
                    // right
                    x: sb.sensor.x + budget + 1,
                    y: sb.sensor.y,
                },
                Point {
                    // up
                    x: sb.sensor.x,
                    y: sb.sensor.y - budget - 1,
                },
                Point {
                    // left
                    x: sb.sensor.x - budget - 1,
                    y: sb.sensor.y,
                },
            )
        })
        .collect();

    // apart from being at a sensor's diamond, the beacon should also stay in
    // the intersection between 2 diamond+1 perimeter (otherwise, more than one
    // position would be possible).
    //
    // Find the (potentially) 4 intersections between 2 diamonds and store it all
    let mut intersections = HashSet::new();
    for d1 in &diamonds {
        for d2 in &diamonds {
            if d1 == d2 {
                continue;
            }
            let (d1, r1, u1, l1) = d1;
            let (d2, r2, u2, l2) = d2;
            if let Some(i1) = find_intersection((u1, r1), (l2, u2)) {
                intersections.insert(i1);
            }
            if let Some(i2) = find_intersection((l1, d1), (d2, r2)) {
                intersections.insert(i2);
            }
        }
    }

    // filter the intersections by eliminating those who lie outside the allowed
    // area, and those that are covered by other sensors.
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

    let r = res.get(0).unwrap();
    return r.x * 4000000 + r.y;
}

fn find_intersection(l1: (&Point, &Point), l2: (&Point, &Point)) -> Option<Point> {
    let (Point { x: x1, y: y1 }, Point { x: x2, y: y2 }) = l1;
    let (Point { x: x3, y: y3 }, Point { x: x4, y: y4 }) = l2;

    let drift = (*y3 - *y1 - (*x1 - *x3).abs()) / 2;
    let new_x = max(*x1, *x3) + drift;
    let new_y = if *x1 >= *x3 { *y1 + drift } else { *y3 - drift };

    if new_x < *x1
        || new_x > *x2
        || new_x < *x3
        || new_x > *x4
        || new_y < *y1
        || new_y > *y2
        || new_y > *y3
        || new_y < *y4
        || 2 * drift != *y3 - *y1 - (*x1 - *x3).abs()
    {
        return None;
    }
    return Some(Point { x: new_x, y: new_y });
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
