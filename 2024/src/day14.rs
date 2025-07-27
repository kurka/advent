use regex::Regex;
use std::fs;

#[derive(Clone, Debug)]
struct RobotPos {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/14.txt").unwrap());
    println!("Day 14:");
    println!("{}", solve_part_a(&input, 101, 103));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<RobotPos> {
    let re_robot = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let (_, [px, py, vx, vy]) = re_robot.captures(line).unwrap().extract();
            RobotPos {
                px: px.parse().unwrap(),
                py: py.parse().unwrap(),
                vx: vx.parse().unwrap(),
                vy: vy.parse().unwrap(),
            }
        })
        .collect()
}

fn solve_part_a(input: &Vec<RobotPos>, grid_x: i32, grid_y: i32) -> usize {
    let seconds = 100;
    let mut robots = input.clone();

    let mut grid: Vec<Vec<usize>> = vec![vec![0; grid_x as usize]; grid_y as usize];
    for robot in &robots {
        grid[robot.py as usize][robot.px as usize] += 1;
    }
    for _ in 0..seconds {
        for robot in &mut robots {
            grid[robot.py as usize][robot.px as usize] -= 1;
            robot.px = (robot.px + robot.vx).rem_euclid(grid_x);
            robot.py = (robot.py + robot.vy).rem_euclid(grid_y);
            grid[robot.py as usize][robot.px as usize] += 1;
        }
    }

    // compute quadrants score
    let mut quadrants = [0, 0, 0, 0];
    for y in 0..grid_y {
        for x in 0..grid_x {
            if x < grid_x / 2 && y < grid_y / 2 {
                quadrants[0] += grid[y as usize][x as usize];
            }
            if x > grid_x / 2 && y < grid_y / 2 {
                quadrants[1] += grid[y as usize][x as usize];
            }
            if x < grid_x / 2 && y > grid_y / 2 {
                quadrants[2] += grid[y as usize][x as usize];
            }
            if x > grid_x / 2 && y > grid_y / 2 {
                quadrants[3] += grid[y as usize][x as usize];
            }
        }
    }
    quadrants.iter().product()
}

fn solve_part_b(input: &Vec<RobotPos>) -> usize {
    let grid_x: i32 = 101;
    let grid_y: i32 = 103;
    let robots = input.clone();
    let seconds = 10000;

    let mut grid: Vec<Vec<usize>> = vec![vec![0; grid_x as usize]; grid_y as usize];
    for robot in &robots {
        grid[robot.py as usize][robot.px as usize] += 1;
    }

    // draw robots
    for time in 0..seconds {
        for rob in &robots {
            grid[(rob.py + time * rob.vy).rem_euclid(grid_y) as usize]
                [(rob.px + time * rob.vx).rem_euclid(grid_x) as usize] += 1;
        }

        let mut has_diag = false;
        // look for settings that has a diagonal with 10 bots in a row
        let diag_size = 10;
        for i in 0..(grid_y as usize) - diag_size {
            for j in 2..(grid_x as usize) - diag_size {
                if (0..diag_size).all(|inc| grid[i + inc][j + inc] > 0) {
                    has_diag = true;
                    break;
                }
            }
        }
        if has_diag {
            return time as usize;
            // println!("Round {}", time);

            // for row in &grid {
            //     let visible_row: String = String::from_iter(row.iter().map(|c| {
            //         if *c == 0 {
            //             '.'
            //         } else if *c == 1 {
            //             '#'
            //         } else {
            //             '*'
            //         }
            //     }));
            //     println!("{visible_row:?}");
            // }
        }
        for rob in &robots {
            grid[(rob.py + time * rob.vy).rem_euclid(grid_y) as usize]
                [(rob.px + time * rob.vx).rem_euclid(grid_x) as usize] -= 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input, 11, 7), 12);
        // assert_eq!(solve_part_b(&input), 1337);
    }
}
