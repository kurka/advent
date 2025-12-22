use std::{cmp, fs};

#[derive(Clone, Debug)]
struct DayInput {
    tiles: Vec<(i64, i64)>,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
enum SearchDir {
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/09.txt").unwrap());
    println!("Day 09:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> DayInput {
    DayInput {
        tiles: input
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect(),
    }
}

fn solve_part_a(input: &DayInput) -> i64 {
    let n_tiles = input.tiles.len();

    let mut max_ret = 0;
    for i in 0..n_tiles {
        for j in (i + 1)..n_tiles {
            max_ret = cmp::max(
                max_ret,
                ((input.tiles[i].0 - input.tiles[j].0).abs() + 1)
                    * ((input.tiles[i].1 - input.tiles[j].1).abs() + 1),
            );
        }
    }

    max_ret
}

fn solve_part_b(input: &DayInput) -> i64 {
    let n_tiles = input.tiles.len();
    let horizontal_lines: Vec<(i64, i64, i64)> = (0..n_tiles)
        .filter_map(|i| {
            let source = input.tiles[i];
            let target = input.tiles[(i + 1) % n_tiles];
            if source.1 == target.1 {
                let left = cmp::min(source.0, target.0);
                let right = cmp::max(source.0, target.0);
                Some((source.1, left, right))
            } else {
                None
            }
        })
        .collect();
    let vertical_lines: Vec<(i64, i64, i64)> = (0..n_tiles)
        .filter_map(|i| {
            let source = input.tiles[i];
            let target = input.tiles[(i + 1) % n_tiles];
            if source.0 == target.0 {
                let top = cmp::min(source.1, target.1);
                let bottom = cmp::max(source.1, target.1);
                Some((source.0, top, bottom))
            } else {
                None
            }
        })
        .collect();

    let mut max_rect_area: i64 = 0;
    for i in 0..n_tiles {
        let prev = input.tiles[((i as i32 - 1 + n_tiles as i32) as usize) % n_tiles];
        let cur = input.tiles[i];
        let next = input.tiles[(i + 1) % n_tiles];

        let mut search_spaces = Vec::with_capacity(4);
        match (
            (prev.0 == cur.0),
            (prev.1 == cur.1),
            (cur.0 == next.0),
            (cur.1 == next.1),
        ) {
            (true, _, _, true) if cur.1 > prev.1 && cur.0 > next.0 => {
                // SW
                search_spaces.push(SearchDir::UpLeft); // SW
            }
            (true, _, _, true) if cur.1 < prev.1 && cur.0 > next.0 => {
                // NW
                search_spaces.push(SearchDir::UpLeft);
                search_spaces.push(SearchDir::UpRight);
                search_spaces.push(SearchDir::DownRight);
            } // NW
            (true, _, _, true) if cur.1 > prev.1 && cur.0 < next.0 => {
                // SE
                search_spaces.push(SearchDir::UpLeft);
                search_spaces.push(SearchDir::DownLeft);
                search_spaces.push(SearchDir::DownRight);
            } // SE
            (true, _, _, true) if cur.1 < prev.1 && cur.0 < next.0 => {
                // NE
                search_spaces.push(SearchDir::DownRight);
            } // NE
            (_, true, true, _) if cur.0 > prev.0 && cur.1 > next.1 => {
                // EN
                search_spaces.push(SearchDir::UpRight);
                search_spaces.push(SearchDir::DownLeft);
                search_spaces.push(SearchDir::DownRight);
            } // EN
            (_, true, true, _) if cur.0 < prev.0 && cur.1 > next.1 => {
                // WN
                search_spaces.push(SearchDir::UpRight);
            } // WN
            (_, true, true, _) if cur.0 > prev.0 && cur.1 < next.1 => {
                // ES
                search_spaces.push(SearchDir::DownLeft);
            } // ES
            (_, true, true, _) if cur.0 < prev.0 && cur.1 < next.1 => {
                // WS
                search_spaces.push(SearchDir::UpLeft);
                search_spaces.push(SearchDir::UpRight);
                search_spaces.push(SearchDir::DownLeft);
            } // WS
            _ => unreachable!(),
        };

        for region in search_spaces {
            match region {
                SearchDir::UpLeft => {
                    for (other_x, other_y) in &input.tiles {
                        if !(other_x <= &cur.0 && other_y <= &cur.1) {
                            continue;
                        }

                        // hx0  other_x  cur.0  hx1
                        // hx0  other_x  hx1   cur.0
                        //  other_x hx0  hx1   cur.0
                        //  other_x hx0  cur.0  hx1
                        // hx0  hx1 other_x    cur.0
                        //  other_x    cur.0 hx0  hx1
                        if horizontal_lines.iter().any(|(hy, hx0, hx1)| {
                            hy < &cur.1 && hy > other_y && !(hx1 <= other_x || hx0 >= &cur.0)
                        }) {
                            continue;
                        }
                        if vertical_lines.iter().any(|(vx, vy0, vy1)| {
                            vx < &cur.0 && vx > other_x && !(vy1 <= other_y || vy0 >= &cur.1)
                        }) {
                            continue;
                        }
                        let rect_area = (cur.0 - other_x + 1) * (cur.1 - other_y + 1);
                        // println!("{rect_area} cur: {cur:?} other: ({other_x}, {other_y}) prev: {prev:?}  next: {next:?} {region:?} {rect_area:?}");
                        max_rect_area = cmp::max(max_rect_area, rect_area)
                    }
                }
                SearchDir::DownRight => {
                    for (other_x, other_y) in &input.tiles {
                        if !(other_x >= &cur.0 && other_y >= &cur.1) {
                            continue;
                        }

                        if horizontal_lines.iter().any(|(hy, hx0, hx1)| {
                            hy > &cur.1 && hy < other_y && !(hx1 <= &cur.0 || hx0 >= other_x)
                        }) {
                            continue;
                        }
                        if vertical_lines.iter().any(|(vx, vy0, vy1)| {
                            vx > &cur.0 && vx < other_x && !(vy1 <= &cur.1 || vy0 >= &other_y)
                        }) {
                            continue;
                        }
                        let rect_area = (other_x - cur.0 + 1) * (other_y - cur.1 + 1);
                        // (2,3) -> (9,5)
                        // println!("{rect_area} cur: {cur:?} other: ({other_x}, {other_y}) prev: {prev:?}  next: {next:?} {region:?} {rect_area:?}");
                        max_rect_area = cmp::max(max_rect_area, rect_area)
                    }
                }
                SearchDir::UpRight => {
                    for (other_x, other_y) in &input.tiles {
                        if !(other_x >= &cur.0 && other_y <= &cur.1) {
                            continue;
                        }

                        if horizontal_lines.iter().any(|(hy, hx0, hx1)| {
                            hy < &cur.1 && hy > other_y && !(hx1 <= &cur.0 || hx0 >= other_x)
                        }) {
                            continue;
                        }
                        if vertical_lines.iter().any(|(vx, vy0, vy1)| {
                            vx > &cur.0 && vx < other_x && !(vy1 <= other_y || vy0 >= &cur.1)
                        }) {
                            continue;
                        }
                        let rect_area = (other_x - cur.0 + 1) * (cur.1 - other_y + 1);
                        // println!("{rect_area} cur: {cur:?} other: ({other_x}, {other_y}) prev: {prev:?}  next: {next:?} {region:?} {rect_area:?}");
                        max_rect_area = cmp::max(max_rect_area, rect_area)
                    }
                }
                SearchDir::DownLeft => {
                    for (other_x, other_y) in &input.tiles {
                        if !(other_x <= &cur.0 && other_y >= &cur.1) {
                            continue;
                        }

                        if horizontal_lines.iter().any(|(hy, hx0, hx1)| {
                            hy > &cur.1 && hy < other_y && !(hx1 <= other_x || hx0 >= &cur.0)
                        }) {
                            continue;
                        }
                        if vertical_lines.iter().any(|(vx, vy0, vy1)| {
                            vx < &cur.0 && vx > other_x && !(vy1 <= &cur.1 || vy0 >= other_y)
                        }) {
                            continue;
                        }
                        let rect_area = (cur.0 - other_x + 1) * (other_y - cur.1 + 1);
                        // println!("{rect_area} cur: {cur:?} other: ({other_x}, {other_y}) prev: {prev:?}  next: {next:?} {region:?} {rect_area:?}");
                        max_rect_area = cmp::max(max_rect_area, rect_area)
                    }
                }
            }
        }
    }

    max_rect_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 50);
        assert_eq!(solve_part_b(&input), 24);
    }
}
