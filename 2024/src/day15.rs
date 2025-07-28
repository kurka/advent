use std::fs;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/15.txt").unwrap());
    println!("Day 15:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> (Vec<Vec<char>>, Vec<Dir>) {
    let (grid_str, moves_str) = input.split_once("\n\n").unwrap();

    let grid = grid_str
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let moves = moves_str
        .lines()
        .flat_map(|line| {
            line.chars().map(|char| match char {
                '^' => Dir::Up,
                '>' => Dir::Right,
                'v' => Dir::Down,
                '<' => Dir::Left,
                _ => panic!(),
            })
        })
        .collect();
    (grid, moves)
}

fn solve_part_a(input: &(Vec<Vec<char>>, Vec<Dir>)) -> usize {
    let (original_grid, moves) = input;
    let mut grid = original_grid.clone();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut start_pos = (0, 0);
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '@' {
                start_pos = (i, j);
            }
        }
    }
    grid[start_pos.0][start_pos.1] = '.';

    let mut pos = start_pos;
    moves.iter().for_each(|dir| {
        if let Some(free) = find_free(pos, &dir, &grid) {
            pos = match dir {
                Dir::Up => (pos.0 - 1, pos.1),
                Dir::Down => (pos.0 + 1, pos.1),
                Dir::Left => (pos.0, pos.1 - 1),
                Dir::Right => (pos.0, pos.1 + 1),
            };
            grid[free.0][free.1] = 'O';
            grid[pos.0][pos.1] = '.';
        }
    });

    let mut score = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 'O' {
                score += 100 * i + j
            }
        }
    }
    score
}

fn find_free(
    start_pos: (usize, usize),
    dir: &Dir,
    grid: &Vec<Vec<char>>,
) -> Option<(usize, usize)> {
    let mut pos = start_pos;
    loop {
        pos = match dir {
            Dir::Up => (pos.0 - 1, pos.1),
            Dir::Down => (pos.0 + 1, pos.1),
            Dir::Left => (pos.0, pos.1 - 1),
            Dir::Right => (pos.0, pos.1 + 1),
        };

        if grid[pos.0][pos.1] == '.' {
            return Some(pos);
        }
        if grid[pos.0][pos.1] == '#' {
            break;
        }
    }

    None
}

fn solve_part_b(input: &(Vec<Vec<char>>, Vec<Dir>)) -> usize {
    let (original_grid, moves) = input;
    let rows = original_grid.len();
    let cols = original_grid[0].len();

    let mut grid: Vec<Vec<char>> = (0..rows)
        .map(|i| {
            (0..cols)
                .flat_map(|j| match original_grid[i][j] {
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '.' => ['.', '.'],
                    '@' => ['@', '.'],
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    let cols = grid[0].len();

    let mut start_pos = (0, 0);
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '@' {
                start_pos = (i, j);
            }
        }
    }

    // for row in &grid {
    //     println!("{:?}", row);
    // }

    grid[start_pos.0][start_pos.1] = '.';

    let mut pos = start_pos;
    moves.iter().for_each(|dir| {
        if let Some(box_stack) = can_push(pos, &dir, &grid) {
            // println!("Moving {:?} {pos:?}", box_stack);
            pos = match dir {
                Dir::Up => (pos.0 - 1, pos.1),
                Dir::Down => (pos.0 + 1, pos.1),
                Dir::Left => (pos.0, pos.1 - 1),
                Dir::Right => (pos.0, pos.1 + 1),
            };
            push_stack(&box_stack, &dir, &mut grid);
        }
        // println!("Move {:?} {:?}", dir, pos);
        // for row in &grid {
        //     println!("{:?}", row);
        // }
    });

    let mut score = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '[' {
                score += 100 * i + j
            }
        }
    }
    score
}

fn push_stack(box_stack: &Vec<(usize, usize)>, dir: &Dir, grid: &mut Vec<Vec<char>>) {
    for mbox in box_stack.iter() {
        match dir {
            Dir::Up => {
                grid[mbox.0 - 1][mbox.1] = '[';
                grid[mbox.0 - 1][mbox.1 + 1] = ']';
                grid[mbox.0][mbox.1] = '.';
                grid[mbox.0][mbox.1 + 1] = '.';
            }
            Dir::Down => {
                grid[mbox.0 + 1][mbox.1] = '[';
                grid[mbox.0 + 1][mbox.1 + 1] = ']';
                grid[mbox.0][mbox.1] = '.';
                grid[mbox.0][mbox.1 + 1] = '.';
            }
            Dir::Left => {
                grid[mbox.0][mbox.1 - 1] = '[';
                grid[mbox.0][mbox.1] = ']';
                grid[mbox.0][mbox.1 + 1] = '.';
            }
            Dir::Right => {
                grid[mbox.0][mbox.1 + 2] = ']';
                grid[mbox.0][mbox.1 + 1] = '[';
                grid[mbox.0][mbox.1] = '.';
            }
        }
    }
}

fn can_push(pos: (usize, usize), dir: &Dir, grid: &Vec<Vec<char>>) -> Option<Vec<(usize, usize)>> {
    let sides = match dir {
        Dir::Up => {
            if grid[pos.0][pos.1] == '.' {
                vec![(pos.0 - 1, pos.1)]
            } else {
                vec![(pos.0 - 1, pos.1), (pos.0 - 1, pos.1 + 1)]
            }
        }
        Dir::Down => {
            if grid[pos.0][pos.1] == '.' {
                vec![(pos.0 + 1, pos.1)]
            } else {
                vec![(pos.0 + 1, pos.1), (pos.0 + 1, pos.1 + 1)]
            }
        }
        Dir::Left => vec![(pos.0, pos.1 - 1)],
        Dir::Right => vec![(pos.0, pos.1 + 1)],
    };

    if sides.iter().any(|side| grid[side.0][side.1] == '#') {
        return None;
    } else if sides.iter().all(|side| grid[side.0][side.1] == '.') {
        return Some(vec![]);
    } else {
        match dir {
            Dir::Right => {
                if let Some(mut boxes) = can_push((pos.0, pos.1 + 2), dir, grid) {
                    boxes.push((pos.0, pos.1 + 1));
                    return Some(boxes);
                } else {
                    return None;
                }
            }
            Dir::Left => {
                if let Some(mut boxes) = can_push((pos.0, pos.1 - 2), dir, grid) {
                    boxes.push((pos.0, pos.1 - 2));
                    return Some(boxes);
                } else {
                    return None;
                }
            }
            Dir::Down | Dir::Up => {
                let range = if grid[pos.0][pos.1] == '[' {
                    -1..=1
                } else {
                    -1..=0
                };
                let adjacent_boxes = range.filter_map(|pos_side| {
                    if grid[(pos.0 as i32 + (if *dir == Dir::Down { 1 } else { -1 })) as usize]
                        [(pos.1 as i32 + pos_side) as usize]
                        == '['
                    {
                        Some((
                            (pos.0 as i32 + (if *dir == Dir::Down { 1 } else { -1 })) as usize,
                            (pos.1 as i32 + pos_side) as usize,
                        ))
                    } else {
                        None
                    }
                });

                let mut boxes_stack: Vec<(usize, usize)> = vec![];
                for adjacent_box in adjacent_boxes {
                    if let Some(boxes) = can_push(adjacent_box, dir, grid) {
                        let new_boxes: Vec<(usize, usize)> = boxes
                            .into_iter()
                            .filter(|abox| !boxes_stack.contains(abox))
                            .collect();
                        boxes_stack.extend(new_boxes);
                        boxes_stack.push(adjacent_box);
                    } else {
                        return None;
                    }
                }
                return Some(boxes_stack);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 2028);

        let sample = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

        let input = parse_input(sample.to_string());
        assert_eq!(solve_part_a(&input), 10092);
        assert_eq!(solve_part_b(&input), 9021);
    }
}
