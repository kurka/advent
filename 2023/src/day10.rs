use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/10.txt").unwrap());
    println!("Day 10:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn solve_part_a(input: &Vec<Vec<char>>) -> usize {
    find_path(input).len() / 2
}

fn solve_part_b(input: &Vec<Vec<char>>) -> usize {
    let path = find_path(input);

    let start_pos = path[0];
    let start_edge1 = path[1];

    // make a copy of the map so we can annotate it
    // we are going to replace horizontal paths by '>' and vertical by 'v'.
    // Corners depend: L and J are 'v', F and 7 '>'
    let mut map_copy = input.clone();

    // if start_pos has a path starting from its top, consider it a 'v' piece; otherwise, a '>'.
    map_copy[start_pos.0][start_pos.1] = if start_pos.0 > start_edge1.0 {
        'v'
    } else {
        '>'
    };

    for path_pos in path[1..].iter() {
        map_copy[path_pos.0][path_pos.1] = match map_copy[path_pos.0][path_pos.1] {
            '|' => 'v',
            '-' => '>',
            'L' => 'v',
            'J' => 'v',
            'F' => '>',
            '7' => '>',
            x => {
                println!("Found: {x} {path_pos:?} {start_pos:?}");
                unreachable!("dammit")
            }
        }
    }

    // finally, compute the enclosed cells
    let mut enclosed = 0;
    for i in 0..map_copy.len() {
        let mut is_inside = false;
        for j in 0..map_copy[i].len() {
            let symbol = map_copy[i][j];
            if symbol == 'v' {
                is_inside = !is_inside;
            } else if symbol == '>' {
                continue;
            } else if is_inside {
                enclosed += 1;
            }
        }
    }
    enclosed
}

fn find_path(input: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    // find start position
    let (start_i, start_j): (usize, usize) = input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, cell)| (*cell == 'S').then_some((i, j)))
        })
        .take(1)
        .next()
        .unwrap();

    // find loop start
    let ((mut next_i, mut next_j), mut dir) = [
        (-1, 0, 'U', ['|', '7', 'F']),
        (0, 1, 'R', ['-', '7', 'J']),
        (1, 0, 'D', ['|', 'L', 'J']),
        (0, -1, 'L', ['-', 'L', 'F']),
    ]
    .iter()
    .filter_map(|(ii, jj, ddir, options)| {
        if start_i as i32 + ii < 0
            || start_i as i32 + ii >= input.len() as i32
            || start_j as i32 + jj < 0
            || start_j as i32 + jj >= input[start_i].len() as i32
        {
            return None;
        }

        let new_i = (start_i as i32 + ii) as usize;
        let new_j = (start_j as i32 + jj) as usize;
        for opt in options {
            if input[new_i][new_j] == *opt {
                return Some(((new_i, new_j), *ddir));
            }
        }
        return None;
    })
    .take(1)
    .next()
    .unwrap();

    let mut path = vec![(start_i, start_j)];
    while input[next_i][next_j] != 'S' {
        path.push((next_i, next_j));
        ((next_i, next_j), dir) = match (input[next_i][next_j], dir) {
            ('|', 'U') => ((next_i - 1, next_j), 'U'),
            ('|', 'D') => ((next_i + 1, next_j), 'D'),
            ('-', 'R') => ((next_i, next_j + 1), 'R'),
            ('-', 'L') => ((next_i, next_j - 1), 'L'),
            ('L', 'D') => ((next_i, next_j + 1), 'R'),
            ('L', 'L') => ((next_i - 1, next_j), 'U'),
            ('J', 'D') => ((next_i, next_j - 1), 'L'),
            ('J', 'R') => ((next_i - 1, next_j), 'U'),
            ('7', 'U') => ((next_i, next_j - 1), 'L'),
            ('7', 'R') => ((next_i + 1, next_j), 'D'),
            ('F', 'U') => ((next_i, next_j + 1), 'R'),
            ('F', 'L') => ((next_i + 1, next_j), 'D'),
            _ => unreachable!(),
        };
    }
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
.....
.S-7.
.|.|.
.L-J.
.....
";
        let input = parse_input(sample.to_string());
        assert_eq!(solve_part_a(&input), 4);

        let sample = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";
        let input = parse_input(sample.to_string());
        assert_eq!(solve_part_a(&input), 8);

        let sample = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";
        let input = parse_input(sample.to_string());
        assert_eq!(solve_part_b(&input), 4);

        let sample = "\
        ...........
        .S-------7.
        .|F--7.F7|.
        .||..|.|||.
        .||..L-J||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
        ";

        let input = parse_input(sample.to_string());
        assert_eq!(solve_part_b(&input), 6);

        let sample = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

        let input = parse_input(sample.to_string());
        assert_eq!(solve_part_b(&input), 8);

        let sample = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

        let input = parse_input(sample.to_string());
        assert_eq!(solve_part_b(&input), 10);
    }
}
