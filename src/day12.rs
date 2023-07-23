use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs,
};

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/input12.in").unwrap());
    println!("Day 12:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn solve_12(map: &Vec<Vec<char>>, partb: bool) -> Result<usize, String> {
    let mut known_locations: HashMap<(usize, usize), usize> = HashMap::new();
    let mut queue: BinaryHeap<(Reverse<usize>, (usize, usize))> = BinaryHeap::new();

    // find end position as start position
    let mut start_pos = (0, 0);
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == 'E' {
                start_pos = (row, col);
            }
        }
    }

    known_locations.insert(start_pos, 0);
    queue.push((Reverse(0), start_pos));

    let nrows = map.len();
    let ncols = map[0].len();

    while let Some((Reverse(dist), (srow, scol))) = queue.pop() {
        // go to next unvisited spot in queue
        if map[srow][scol] == 'S' || (partb && map[srow][scol] == 'a') {
            return Ok(dist);
        }
        // replace special chars
        let cur_char = match map[srow][scol] {
            'S' => 'a',
            'E' => 'z',
            c => c,
        };

        // check neighbors
        for (rdiff, cdiff) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let (isrow, iscol) = (srow as i32, scol as i32);
            let (ineirow, ineicol) = ((isrow + rdiff), (iscol + cdiff));
            if ineirow < 0 || ineirow >= nrows as i32 || ineicol < 0 || ineicol >= ncols as i32 {
                continue;
            }
            let nei_pos = (ineirow as usize, ineicol as usize);
            let nei_char = match map[nei_pos.0][nei_pos.1] {
                'S' => 'a',
                'E' => 'z',
                c => c,
            };
            if nei_char < (cur_char as u8 - 1) as char {
                continue;
            }
            let f = dist + 1;
            let maybe_known_score = known_locations.get(&nei_pos);
            if let Some(known_score) = maybe_known_score {
                if *known_score <= f {
                    continue;
                }
            }
            known_locations.insert(nei_pos, f);
            queue.push((Reverse(f), nei_pos))
        }
    }

    Err("Could not find path to solution!".to_string())
}

fn solve_part_a(map: &Vec<Vec<char>>) -> usize {
    solve_12(map, false).unwrap()
}

fn solve_part_b(map: &Vec<Vec<char>>) -> usize {
    solve_12(map, true).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

        let input = parse_input(sample.to_string());

        assert_eq!(31, solve_part_a(&input));
        assert_eq!(29, solve_part_b(&input));
    }
}
