use regex::Regex;
use std::fs;

#[derive(Clone, Debug)]
struct DigPlan {
    dir: char,
    steps: i32,
    color: String,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/18.txt").unwrap());
    println!("Day 18:");
    println!("{}", solve_part_a(&input));
    // println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<DigPlan> {
    let re = Regex::new(r"(\w) (\d+) \((#\w{6})\)").unwrap();
    input
        .lines()
        .map(|line| {
            let (_, [dir_s, steps_s, color]) = re.captures(line).unwrap().extract();
            DigPlan {
                dir: dir_s.chars().next().unwrap(),
                steps: steps_s.parse().unwrap(),
                color: color.to_string(),
            }
        })
        .collect()
}

fn solve_part_a(input: &Vec<DigPlan>) -> usize {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    let mut cur_x = 0;
    let mut cur_y = 0;
    for dp in input {
        match dp.dir {
            'L' => cur_x -= dp.steps,
            'R' => cur_x += dp.steps,
            'U' => cur_y -= dp.steps,
            'D' => cur_y += dp.steps,
            _ => unreachable!(),
        }

        if cur_x < min_x {
            min_x = cur_x
        }
        if cur_x > max_x {
            max_x = cur_x
        }
        if cur_y < min_y {
            min_y = cur_y
        }
        if cur_y > max_y {
            max_y = cur_y
        }
    }
    let mut grid = vec![vec!['.'; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
    let mut cur_x = -min_x;
    let mut cur_y = -min_y;
    let mut prev_dir = input[input.len() - 1].dir;
    println!(
        "grid completed: {min_x}, {max_x};  {min_y}, {max_y} - gx {} gy {}",
        grid[0].len(),
        grid.len()
    );
    // println!("{grid:?}");
    for dp in input {
        println!(
            "Processing {dp:?} ({cur_x}, {cur_y}),  {prev_dir} {}",
            dp.dir
        );

        match (prev_dir, dp.dir) {
            ('D', 'R') => grid[cur_y as usize][cur_x as usize] = 'L',
            ('L', 'U') => grid[cur_y as usize][cur_x as usize] = 'L',
            ('D', 'L') => grid[cur_y as usize][cur_x as usize] = 'J',
            ('R', 'U') => grid[cur_y as usize][cur_x as usize] = 'J',
            ('R', 'D') => grid[cur_y as usize][cur_x as usize] = '7',
            ('U', 'L') => grid[cur_y as usize][cur_x as usize] = '7',
            ('L', 'D') => grid[cur_y as usize][cur_x as usize] = 'F',
            ('U', 'R') => grid[cur_y as usize][cur_x as usize] = 'F',
            _ => unreachable!(),
        }
        prev_dir = dp.dir;

        match dp.dir {
            'L' => {
                (cur_x - dp.steps..cur_x).for_each(|x| grid[cur_y as usize][x as usize] = '-');
                cur_x -= dp.steps
            }
            'R' => {
                (cur_x + 1..=cur_x + dp.steps).for_each(|x| grid[cur_y as usize][x as usize] = '-');
                cur_x += dp.steps
            }
            'U' => {
                (cur_y - dp.steps..cur_y).for_each(|y| grid[y as usize][cur_x as usize] = '|');
                cur_y -= dp.steps
            }
            'D' => {
                (cur_y + 1..=cur_y + dp.steps).for_each(|y| grid[y as usize][cur_x as usize] = '|');
                cur_y += dp.steps
            }
            _ => unreachable!(),
        }
    }

    // fill end of loop
    match (prev_dir, input[0].dir) {
        ('D', 'R') => grid[cur_y as usize][cur_x as usize] = 'L',
        ('L', 'U') => grid[cur_y as usize][cur_x as usize] = 'L',
        ('D', 'L') => grid[cur_y as usize][cur_x as usize] = 'J',
        ('R', 'U') => grid[cur_y as usize][cur_x as usize] = 'J',
        ('R', 'D') => grid[cur_y as usize][cur_x as usize] = '7',
        ('U', 'L') => grid[cur_y as usize][cur_x as usize] = '7',
        ('L', 'D') => grid[cur_y as usize][cur_x as usize] = 'F',
        ('U', 'R') => grid[cur_y as usize][cur_x as usize] = 'F',
        _ => unreachable!(),
    }
    // println!("{grid:?}");
    for l in &grid {
        println!("{:?}", l.iter().take(600).collect::<String>());
    }

    let mut enclosed = 0;
    for i in 0..grid.len() {
        let mut is_inside = false;
        for j in 0..grid[i].len() {
            let symbol = grid[i][j];
            if symbol == '|' || symbol == 'L' || symbol == 'J' {
                is_inside = !is_inside;
            }
            if symbol != '.' {
                enclosed += 1;
            } else if is_inside {
                enclosed += 1;
                grid[i][j] = '#';
            } else {
                grid[i][j] = ' ';
            }
        }
    }

    // grid[267][159] = '*';
    for l in &grid {
        println!("{:?}", l.iter().take(600).collect::<String>());
    }

    enclosed
}

fn _solve_part_b(_input: &Vec<DigPlan>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 62);
        // assert_eq!(solve_part_b(&input), 1337);
    }
}
