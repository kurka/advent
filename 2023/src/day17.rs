use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct State {
    position: (usize, usize),
    direction: char,
    reps: usize,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/17.txt").unwrap());
    println!("Day 17:");
    println!("{}", solve_part_a(&input).unwrap());
    println!("{}", solve_part_b(&input).unwrap());
}

fn parse_input(input: String) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn solve_part_a(grid: &Vec<Vec<usize>>) -> Option<usize> {
    solve_17(grid, true)
}

fn solve_part_b(grid: &Vec<Vec<usize>>) -> Option<usize> {
    solve_17(grid, false)
}

fn solve_17(grid: &Vec<Vec<usize>>, part_a: bool) -> Option<usize> {
    let n_rows = grid.len();
    let n_cols = grid[0].len();
    let min_value = 1; // grid.iter().flatten().min().unwrap();
    let mut queue: BinaryHeap<(Reverse<usize>, Reverse<usize>, State)> = BinaryHeap::new();
    let mut known_locations: HashMap<State, usize> = HashMap::new();
    let mut parents: HashMap<State, State> = HashMap::new();
    let goal = (n_rows - 1, n_cols - 1);

    // insert start and two neighbors
    let start_right = State {
        position: (0, 0),
        direction: 'R',
        reps: 0,
    };
    let start_down = State {
        position: (0, 0),
        direction: 'D',
        reps: 0,
    };
    let next_right = State {
        position: (0, 1),
        direction: 'R',
        reps: 0,
    };
    let next_down = State {
        position: (1, 0),
        direction: 'D',
        reps: 0,
    };
    known_locations.insert(start_right, 0);
    known_locations.insert(start_down, 0);
    queue.push((
        Reverse(grid[0][1] + (goal.0 + goal.1 - 1) * min_value),
        Reverse(grid[0][1]),
        next_right,
    ));
    queue.push((
        Reverse(grid[1][0] + (n_rows - 2 + n_cols - 1) * min_value),
        Reverse(grid[1][0]),
        next_down,
    ));
    known_locations.insert(next_right, grid[0][1]);
    known_locations.insert(next_down, grid[1][0]);
    parents.insert(next_right, start_right);
    parents.insert(next_down, start_down);

    // let mut visits = 0;
    while let Some((_, path_dist_from_start, node)) = queue.pop() {
        // println!(
        //     "Visiting {_x:?} {path_dist_from_start:?} {node:?} from {:?}",
        //     parents[&node]
        // );
        let dist_from_start = known_locations[&node];
        if path_dist_from_start < Reverse(dist_from_start) {
            // println!("Warning: was about to try a more expensive path");
            continue;
        }
        if node.position == goal {
            // println!("Visits: {visits}");
            // let mut cur_pos = &node;
            // while cur_pos.position != (0, 0) {
            //     println!("{cur_pos:?}");
            //     cur_pos = &parents[cur_pos];
            // }
            return known_locations.get(&node).copied();
        }

        // visits += 1;

        for new_dir in ['R', 'D', 'L', 'U'] {
            if (node.position.0 == 0 && new_dir == 'U')
                || (node.position.0 == n_rows - 1 && new_dir == 'D')
                || (node.position.1 == 0 && new_dir == 'L')
                || (node.position.1 == n_rows - 1 && new_dir == 'R')
                || (node.direction == 'R' && new_dir == 'L')
                || (node.direction == 'L' && new_dir == 'R')
                || (node.direction == 'U' && new_dir == 'D')
                || (node.direction == 'D' && new_dir == 'U')
            {
                continue;
            }
            if part_a && (node.reps == 2 && new_dir == node.direction) {
                continue;
            }
            if !part_a
                && ((node.reps < 3 && new_dir != node.direction)
                    || (node.reps == 9 && new_dir == node.direction))
            {
                continue;
            }

            let nei_pos = match new_dir {
                'R' => (node.position.0, node.position.1 + 1),
                'D' => (node.position.0 + 1, node.position.1),
                'L' => (node.position.0, node.position.1 - 1),
                'U' => (node.position.0 - 1, node.position.1),
                _ => unreachable!(),
            };
            let neighbor = State {
                position: nei_pos,
                direction: new_dir,
                reps: if new_dir == node.direction {
                    node.reps + 1
                } else {
                    0
                },
            };
            let dist_from_start_nei = dist_from_start + grid[nei_pos.0][nei_pos.1];
            let dist_to_target =
                (n_rows - 1 - node.position.0 + n_cols - 1 - node.position.1) * min_value;
            let score = dist_from_start_nei + dist_to_target;
            if known_locations.get(&neighbor) != None
                && known_locations[&neighbor] <= dist_from_start_nei
            {
                // only insert if score is smaller
                continue;
            }
            known_locations.insert(neighbor, dist_from_start_nei);
            parents.insert(neighbor, node);
            queue.push((Reverse(score), Reverse(dist_from_start_nei), neighbor))
        }
        // println!(
        //     "evaluating {node:?} {:?}\n{:?}\n{:?}\n",
        //     queue, known_locations, parents
        // );
        // if queue.len() > 30 {
        //     break;
        // }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input).unwrap(), 102);
        assert_eq!(solve_part_b(&input).unwrap(), 94);
    }
}
