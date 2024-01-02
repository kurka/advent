use regex::Regex;
use std::cmp::{max, min};
use std::fs;

#[derive(Clone, Debug)]
struct DigPlan {
    dir: char,
    steps: i64,
    color: String,
}
struct DigPlanSimple {
    dir: char,
    steps: i64,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/18.txt").unwrap());
    println!("Day 18:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<DigPlan> {
    let re = Regex::new(r"(\w) (\d+) \(#(\w{6})\)").unwrap();
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

fn solve_part_a(input: &Vec<DigPlan>) -> i64 {
    // alternative, less efficient solution for part_a:
    // _solve_part_a_grid(input) as i64
    solve_18(input, true)
}

fn solve_part_b(input: &Vec<DigPlan>) -> i64 {
    solve_18(input, false)
}

fn solve_18(input: &Vec<DigPlan>, part_a: bool) -> i64 {
    let mut cur_x = 0;
    let mut cur_y = 0;
    let mut prev_x = 0;
    let mut prev_y = 0;

    let mut coords: Vec<_> = input
        .iter()
        .map(|dp| {
            if part_a {
                DigPlanSimple {
                    dir: dp.dir,
                    steps: dp.steps,
                }
            } else {
                let (dist_str, dir_str) = dp.color.split_at(5);
                DigPlanSimple {
                    dir: match dir_str {
                        "0" => 'R',
                        "1" => 'D',
                        "2" => 'L',
                        "3" => 'U',
                        x => unreachable!("found: {x}"),
                    },
                    steps: i64::from_str_radix(dist_str, 16).unwrap(),
                }
            }
        })
        .map(|dp| {
            prev_x = cur_x;
            prev_y = cur_y;
            match dp.dir {
                'L' => cur_x -= dp.steps,
                'R' => cur_x += dp.steps,
                'U' => cur_y -= dp.steps,
                'D' => cur_y += dp.steps,
                _ => unreachable!(),
            }
            let ((nx1, ny1), (nx2, ny2)) = if cur_x >= prev_x {
                ((prev_x, prev_y), (cur_x, cur_y))
            } else {
                ((cur_x, cur_y), (prev_x, prev_y))
            };
            ((nx1, ny1), (nx2, ny2), dp.dir)
        })
        .collect();

    let min_seg = coords
        .iter()
        .filter(|(_, _, d)| *d == 'R' || *d == 'L')
        .min_by_key(|((_, y), _, _)| y)
        .unwrap();
    let start_idx = coords.iter().position(|c| c == min_seg).unwrap();
    // println!("Min seg: {min_seg:?}; min seg idx: {start_idx:?}");
    // assert top is right, otherwise invert everything?
    assert!(min_seg.2 == 'R');

    // bring start to beggining of queue
    coords.rotate_left(start_idx);
    // add vertical before horizontal
    coords.rotate_right(1);

    let mut rights = vec![];
    let mut lefts = vec![];
    let mut total = 0;
    let mut prev_seg = coords[coords.len() - 1];
    for seg_chunk in coords.chunks_exact(2) {
        let (vert_seg, hor_seg) = if let [vert_seg, hor_seg] = seg_chunk {
            (vert_seg, hor_seg)
        } else {
            unreachable!()
        };
        let &(_p1v, _p2v, dv) = vert_seg;
        let &(p1h, p2h, dh) = hor_seg;
        let (p1p, p2p, dp) = prev_seg;

        match dh {
            'R' => {
                match dp {
                    'L' => {
                        // --<---
                        // |
                        // |
                        // -->---
                        if dv == 'D' {
                            total += p1h.1 - p1p.1 - 1;
                        }
                        // -->---
                        // |
                        // |
                        // --<---
                        rights.push((p1h, p2h));
                    }
                    'R' => {
                        // right to right
                        match dv {
                            'U' => {
                                //      -->---
                                //      |
                                //      |
                                // -->---
                                if rights.len() > 0 {
                                    let last = rights.pop().unwrap();
                                    rights.push((last.0, (last.1 .0 - 1, last.1 .1)));
                                }
                                rights.push((p1h, p2h));
                            }
                            'D' => {
                                // -->---
                                //      |
                                //      |
                                //      -->---
                                rights.push(((p1h.0 + 1, p1h.1), p2h));
                            }
                            _ => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
                }
            }
            'L' => {
                match dp {
                    'R' => {
                        // --<---
                        //      |
                        //      |
                        // -->---
                        if dv == 'U' {
                            total += p2p.1 - p2h.1 - 1;
                        }
                        // -->---
                        //      |
                        //      |
                        // --<---
                        lefts.push((p1h, p2h));
                    }
                    'L' => {
                        // left to left
                        match dv {
                            'D' => {
                                //      --<---
                                //      |
                                //      |
                                // --<---
                                if lefts.len() > 0 {
                                    let last = lefts.pop().unwrap();
                                    lefts.push(((last.0 .0 + 1, last.0 .1), last.1));
                                }
                                lefts.push((p1h, p2h));
                            }
                            'U' => {
                                // --<---
                                //      |
                                //      |
                                //      --<--
                                lefts.push((p1h, (p2h.0 - 1, p2h.1)));
                            }
                            _ => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }

        prev_seg = *hor_seg;
    }
    // fix last seg if needed
    if coords[coords.len() - 1].2 == 'R' && coords[0].2 == 'U' && coords[1].2 == 'R' {
        let last_right = rights.pop().unwrap();
        rights.push((last_right.0, (last_right.1 .0 - 1, last_right.1 .1)));
    }
    if coords[coords.len() - 1].2 == 'L' && coords[0].2 == 'D' && coords[1].2 == 'L' {
        let last_left = lefts.pop().unwrap();
        lefts.push(((last_left.0 .0 + 1, last_left.0 .1), last_left.1));
    }

    // println!("Coords: {coords:?}\nLefts: {lefts:?}\nRights: {rights:?}\ntotal: {total}");

    for right in rights {
        let ((rx1, ry1), (rx2, _ry2)) = right;
        // println!("Trying to find matches for {right:?} {}", rx2 - rx1);
        // find the highest left that aligns with rx1
        let mut segments = vec![(rx1, rx2)];
        // O(n**2) or O(n**3) solution. It must be possible to do something better.
        while let Some((x1, x2)) = segments.pop() {
            let &((lx1, ly1), (lx2, _ly2)) = lefts
                .iter()
                .filter(|((lx1, ly1), (lx2, _ly2))| *ly1 > ry1 && !(*lx2 < x1 || *lx1 > x2))
                .min_by_key(|((_, y), _)| *y)
                .unwrap();
            total += (min(x2, lx2) - max(x1, lx1) + 1) * (ly1 - ry1 + 1);
            if lx1 > x1 {
                segments.push((x1, lx1 - 1))
            }
            if lx2 < x2 {
                segments.push((lx2 + 1, x2))
            }
            // println!("Right: {right:?} Left: ({lx1}-{lx2} {ly1}={_ly2}) Segments: {segments:?}")
        }
    }

    total
}

fn _solve_part_a_grid(input: &Vec<DigPlan>) -> usize {
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
    // println!(
    //     "grid completed: {min_x}, {max_x};  {min_y}, {max_y} - gx {} gy {}",
    //     grid[0].len(),
    //     grid.len()
    // );
    // println!("{grid:?}");
    for dp in input {
        // println!(
        //     "Processing {dp:?} ({cur_x}, {cur_y}),  {prev_dir} {}",
        //     dp.dir
        // );

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
    // for l in &grid {
    //     println!("{:?}", l.iter().take(600).collect::<String>());
    // }

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
    // for l in &grid {
    //     println!("{:?}", l.iter().take(600).collect::<String>());
    // }

    enclosed
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
        assert_eq!(solve_part_b(&input), 952408144115);
    }
}
