use std::{collections::HashSet, fs};

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/12.txt").unwrap());
    println!("Day 12:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn solve_part_a(input: &Vec<Vec<char>>) -> usize {
    solve_day12(input, false)
}

fn solve_part_b(input: &Vec<Vec<char>>) -> usize {
    solve_day12(input, true)
}

fn solve_day12(input: &Vec<Vec<char>>, part_b: bool) -> usize {
    let rows = input.len() as i32;
    let cols = input[0].len() as i32;

    // let mut areas: HashMap<char, usize> = HashMap::new();
    // let mut perimeters: HashMap<char, usize> = HashMap::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut res = 0;
    for row in 0..rows {
        let urow = row as usize;
        for col in 0..cols {
            if visited.contains(&(row, col)) {
                continue;
            }
            let ucol = col as usize;
            let letter = input[urow][ucol];
            // discover nodes with bfs
            let mut queue: Vec<(i32, i32)> = vec![(row, col)];
            let mut area = 0;
            let mut visited_sides: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
            let mut perimeter = 0;
            let mut sides = 0;
            visited.insert((row, col));
            while !queue.is_empty() {
                let (qrow, qcol) = queue.pop().expect("queue is never empty at loop");
                area += 1;
                for (nrow, ncol) in [
                    (qrow - 1, qcol),
                    (qrow + 1, qcol),
                    (qrow, qcol - 1),
                    (qrow, qcol + 1),
                ] {
                    if nrow < 0
                        || nrow >= rows
                        || ncol < 0
                        || ncol >= cols
                        || input[nrow as usize][ncol as usize] != letter
                    {
                        perimeter += 1;
                        if part_b && !visited_sides.contains(&((qrow, qcol), (nrow, ncol))) {
                            // println!("new side starting at {:?} {:?}", (qrow, qcol), (nrow, ncol));
                            sides += 1;
                            visited_sides.insert(((qrow, qcol), (nrow, ncol)));
                            if qcol == ncol {
                                // horizontal side
                                // iterate over cols, keep row
                                // go right
                                let mut scol = qcol + 1;
                                while scol < cols
                                    && input[qrow as usize][scol as usize] == letter
                                    && (nrow >= rows
                                        || nrow < 0
                                        || input[nrow as usize][scol as usize] != letter)
                                {
                                    visited_sides.insert(((qrow, scol), (nrow, scol)));
                                    scol += 1;
                                }
                                // go left
                                let mut scol = qcol - 1;
                                while scol >= 0
                                    && input[qrow as usize][scol as usize] == letter
                                    && (nrow >= rows
                                        || nrow < 0
                                        || input[nrow as usize][scol as usize] != letter)
                                {
                                    visited_sides.insert(((qrow, scol), (nrow, scol)));
                                    scol -= 1;
                                }
                            } else {
                                // vertical side
                                // iterate over rows, keep col
                                // go down
                                let mut srow = qrow + 1;
                                while srow < rows
                                    && input[srow as usize][qcol as usize] == letter
                                    && (ncol >= cols
                                        || ncol < 0
                                        || input[srow as usize][ncol as usize] != letter)
                                {
                                    visited_sides.insert(((srow, qcol), (srow, ncol)));
                                    srow += 1;
                                }
                                // go up
                                let mut srow = qrow - 1;
                                while srow >= 0
                                    && input[srow as usize][qcol as usize] == letter
                                    && (ncol >= cols
                                        || ncol < 0
                                        || input[srow as usize][ncol as usize] != letter)
                                {
                                    visited_sides.insert(((srow, qcol), (srow, ncol)));
                                    srow -= 1;
                                }
                            }
                        }
                    } else if !visited.contains(&(nrow, ncol)) {
                        queue.push((nrow, ncol));
                        visited.insert((nrow, ncol));
                    }
                }
            }
            // println!("{letter} {area} {sides} {}", area * sides);
            res += area * if part_b { sides } else { perimeter };
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 1930);
        assert_eq!(solve_part_b(&input), 1206);
    }
}

//  v
// >RRRR<.....
//  RRRRv.....
//  .*RRR<....
//  ..R*......
//  ..^.......
//  ..........
//  VVIIICJJEE
//  MIIIIIJJEE
//  MIIISIJEEE
//  MMMISSJEEE
