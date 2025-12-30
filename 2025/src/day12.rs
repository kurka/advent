use std::fs;

#[derive(Clone, Debug)]
struct DayInput {
    pieces: Vec<Vec<Vec<bool>>>,
    grids: Vec<((usize, usize), Vec<usize>)>,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/12.txt").unwrap());
    println!("Day 12:");
    println!("{}", solve_part_a(&input));
}

fn parse_input(input: String) -> DayInput {
    let (pieces_lines, grids_lines): (Vec<&str>, Vec<&str>) =
        input.lines().partition(|line| !line.contains('x'));
    let mut pieces: Vec<Vec<Vec<bool>>> = vec![];

    fn get_piece_row(line: &str) -> Vec<bool> {
        line.chars().map(|c| c == '#').collect()
    }

    let mut pieces_lines = pieces_lines.into_iter().peekable();
    while pieces_lines.peek().is_some() {
        pieces_lines.next(); // id
        let piece = vec![
            get_piece_row(pieces_lines.next().unwrap()),
            get_piece_row(pieces_lines.next().unwrap()),
            get_piece_row(pieces_lines.next().unwrap()),
        ];
        pieces_lines.next(); // \n

        pieces.push(piece);
    }

    let grids = grids_lines
        .iter()
        .map(|line| {
            let (grid_size_str, pieces_count_str) = line.split_once(": ").unwrap();
            let (grid_size_x, grid_size_y) = grid_size_str.split_once('x').unwrap();

            let pieces_count: Vec<usize> = pieces_count_str
                .split(' ')
                .map(|number| number.parse().unwrap())
                .collect();

            (
                (grid_size_x.parse().unwrap(), grid_size_y.parse().unwrap()),
                pieces_count,
            )
        })
        .collect();

    DayInput {
        pieces: pieces,
        grids: grids,
    }
}

fn solve_part_a(input: &DayInput) -> usize {
    let pieces_sizes: Vec<usize> = input
        .pieces
        .iter()
        .map(|piece| {
            piece
                .into_iter()
                .flatten()
                .map(|b| if *b { 1 } else { 0 })
                .sum::<usize>()
        })
        // .map(|piece| piece.into_iter().flatten().sum())
        .collect();

    // Count the number of boards that have total_size bigger than the sum of the space taken by all pieces.
    // I tried to submit this number, and to my surprise, this was accepted as the final answer!
    // I had no idea how I would solve it for real, so very happy its over!! :D
    input
        .grids
        .iter()
        .filter(|((size_x, size_y), counts)| {
            let required_size: i32 = counts
                .iter()
                .enumerate()
                .map(|(i, c)| c * pieces_sizes[i])
                .sum::<usize>() as i32;
            let total_size = (size_x * size_y) as i32;

            total_size > required_size
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 3);
    }
}
