use std::{
    collections::{HashMap, VecDeque},
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

fn find_shortest_path(map: &Vec<Vec<char>>, start_pos: (usize, usize)) -> Result<usize, String> {
    let mut known_locations: HashMap<(usize, usize), usize> = HashMap::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    known_locations.insert(start_pos, 0);
    queue.push_back(start_pos);

    let nrows = map.len();
    let ncols = map[0].len();

    while !queue.is_empty() {
        // go to next unvisited spot in queue
        let (srow, scol) = queue.pop_front().unwrap(); // FUTURE: transform that into A*
        let dist = *known_locations.get(&(srow, scol)).unwrap();
        if map[srow][scol] == 'E' {
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
            let (ineirow, ineicol) = ((srow as i32 + rdiff), (scol as i32 + cdiff));
            if ineirow < 0 || ineirow >= nrows as i32 || ineicol < 0 || ineicol >= ncols as i32 {
                continue;
            }
            let nei_pos = (ineirow as usize, ineicol as usize);
            let nei_char = match map[nei_pos.0][nei_pos.1] {
                'S' => 'a',
                'E' => 'z',
                c => c,
            };
            let maybe_known_dist = known_locations.get(&nei_pos);
            if nei_char > (cur_char as u8 + 1) as char {
                continue;
            }
            if let Some(known_dist) = maybe_known_dist {
                if *known_dist <= dist + 1 {
                    continue;
                }
            }
            known_locations.insert(nei_pos, dist + 1);
            queue.push_back(nei_pos)
        }
    }

    Err("Could not find path to solution!".to_string())
}

fn solve_12(map: &Vec<Vec<char>>, partb: bool) -> usize {
    // find starts using part a or part b criteria
    let mut starts: Vec<(usize, usize)> = Vec::new();
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == 'S' || (partb && map[row][col] == 'a') {
                starts.push((row, col));
            }
        }
    }

    // for each start, find shortes path. Returns minimum path found.
    starts
        .iter()
        .map(|start_pos| find_shortest_path(map, *start_pos))
        .filter_map(|sp| sp.ok())
        .min()
        .unwrap()
}

fn solve_part_a(map: &Vec<Vec<char>>) -> usize {
    solve_12(map, false)
}

fn solve_part_b(map: &Vec<Vec<char>>) -> usize {
    solve_12(map, true)
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
