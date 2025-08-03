use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs,
};

#[derive(Eq, Hash, PartialEq, Clone, Debug, Copy, PartialOrd, Ord)]
enum Dir {
    North,
    South,
    East,
    West,
}

type Node = (usize, usize, Dir);
type Weight = usize;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/16.txt").unwrap());
    println!("Day 16:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(
    input: String,
) -> (
    HashMap<Node, Vec<(Node, Weight)>>,
    (usize, usize),
    (usize, usize),
) {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut graph: HashMap<Node, Vec<(Node, Weight)>> = HashMap::new();

    // find start and end positions in grid
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 'S' {
                start_pos = (i, j);
            }
            if grid[i][j] == 'E' {
                end_pos = (i, j);
            }
        }
    }
    grid[start_pos.0][start_pos.1] = '.';
    grid[end_pos.0][end_pos.1] = '.';

    let get_neis = |(i, j): (usize, usize)| {
        [Dir::North, Dir::East, Dir::South, Dir::West]
            .iter()
            .filter_map(|dir| match dir {
                Dir::North => {
                    if i > 0 && grid[i - 1][j] == '.' {
                        Some(Dir::North)
                    } else {
                        None
                    }
                }
                Dir::South => {
                    if i < rows - 1 && grid[i + 1][j] == '.' {
                        Some(Dir::South)
                    } else {
                        None
                    }
                }
                Dir::West => {
                    if j > 0 && grid[i][j - 1] == '.' {
                        Some(Dir::West)
                    } else {
                        None
                    }
                }
                Dir::East => {
                    if j < cols - 1 && grid[i][j + 1] == '.' {
                        Some(Dir::East)
                    } else {
                        None
                    }
                }
            })
            .collect()
    };

    let find_next_node = |start_i: usize, start_j: usize, dir: &Dir| {
        let mut i = start_i;
        let mut j = start_j;
        let mut weight = 0;
        loop {
            match dir {
                Dir::North => i = i - 1,
                Dir::South => i = i + 1,
                Dir::West => j = j - 1,
                Dir::East => j = j + 1,
            };
            weight += 1;

            match dir {
                Dir::North => {
                    if grid[i][j - 1] == '.' || grid[i][j + 1] == '.' || grid[i - 1][j] == '#' {
                        return ((i, j, Dir::South), weight);
                    }
                }
                Dir::South => {
                    if grid[i][j - 1] == '.' || grid[i][j + 1] == '.' || grid[i + 1][j] == '#' {
                        return ((i, j, Dir::North), weight);
                    }
                }
                Dir::West => {
                    if grid[i - 1][j] == '.' || grid[i + 1][j] == '.' || grid[i][j - 1] == '#' {
                        return ((i, j, Dir::East), weight);
                    }
                }
                Dir::East => {
                    if grid[i - 1][j] == '.' || grid[i + 1][j] == '.' || grid[i][j + 1] == '#' {
                        return ((i, j, Dir::West), weight);
                    }
                }
            }
        }
    };

    // go over the grid and build graph
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] != '.' {
                continue;
            }
            let node_sides: Vec<Dir> = get_neis((i, j));
            if node_sides == vec![Dir::North, Dir::South]
                || node_sides == vec![Dir::West, Dir::East]
            {
                continue;
            }
            // add new node in graph
            for side in &node_sides {
                let mut neighbors: Vec<(Node, Weight)> = vec![];
                // connect nodes sides
                for target_side in &node_sides {
                    if side == target_side {
                        continue;
                    }
                    neighbors.push(match (side, target_side) {
                        (Dir::North, Dir::South) => ((i, j, *target_side), 0),
                        (Dir::North, Dir::West) | (Dir::North, Dir::East) => {
                            ((i, j, *target_side), 1000)
                        }
                        (Dir::South, Dir::North) => ((i, j, *target_side), 0),
                        (Dir::South, Dir::West) | (Dir::South, Dir::East) => {
                            ((i, j, *target_side), 1000)
                        }
                        (Dir::West, Dir::East) => ((i, j, *target_side), 0),
                        (Dir::West, Dir::North) | (Dir::West, Dir::South) => {
                            ((i, j, *target_side), 1000)
                        }
                        (Dir::East, Dir::West) => ((i, j, *target_side), 0),
                        (Dir::East, Dir::North) | (Dir::East, Dir::South) => {
                            ((i, j, *target_side), 1000)
                        }
                        _ => panic!(),
                    })
                }
                // add neighbors from other positions
                neighbors.push(find_next_node(i, j, side));
                graph.insert((i, j, *side), neighbors);
            }
        }
    }
    if !graph.contains_key(&(start_pos.0, start_pos.1, Dir::East)) {
        graph.insert(
            (start_pos.0, start_pos.1, Dir::East),
            vec![((start_pos.0, start_pos.1, Dir::North), 1000)],
        );
    }
    (graph, start_pos, end_pos)
}

fn solve_part_a(
    input: &(
        HashMap<Node, Vec<(Node, Weight)>>,
        (usize, usize),
        (usize, usize),
    ),
) -> usize {
    let (graph, start_pos, end_pos) = input;

    // run dijkstra to find shortest path
    let mut queue: BinaryHeap<Reverse<(usize, Node)>> = BinaryHeap::new();
    let mut visited: HashSet<Node> = HashSet::new();
    queue.push(Reverse((0, (start_pos.0, start_pos.1, Dir::East))));

    while queue.len() > 0 {
        let Reverse((dist, node)) = queue.pop().unwrap();
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);

        if node.0 == end_pos.0 && node.1 == end_pos.1 {
            return dist;
        }

        // println!("{:?} {:?} {graph:?}", dist, node);
        for (nei, weight) in graph.get(&node).unwrap() {
            if visited.contains(nei) {
                continue;
            }
            queue.push(Reverse((dist + weight, *nei)));
        }
    }
    unreachable!()
}

fn solve_part_b(
    input: &(
        HashMap<Node, Vec<(Node, Weight)>>,
        (usize, usize),
        (usize, usize),
    ),
) -> usize {
    let (graph, start_pos, end_pos) = input;

    // run dijkstra to find all shortests paths
    let mut queue: BinaryHeap<Reverse<(usize, &Node)>> = BinaryHeap::new();
    let mut visited: HashSet<&Node> = HashSet::new();
    let mut prev: HashMap<&Node, Vec<(&Node, usize)>> = HashMap::new();
    let mut distances: HashMap<&Node, usize> = HashMap::new();

    let start_node = (start_pos.0, start_pos.1, Dir::East);
    let mut end_node = None;
    distances.insert(&start_node, 0);
    queue.push(Reverse((0, &start_node)));

    // let mut count = 0;
    while queue.len() > 0 {
        let Reverse((dist, node)) = queue.pop().unwrap();
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);

        if node.0 == end_pos.0 && node.1 == end_pos.1 {
            end_node = Some(node);
            break;
        }

        for (nei, weight) in graph.get(&node).unwrap() {
            let new_dist = distances.get(node).unwrap() + weight;
            let maybe_old_dist = distances.get(nei);
            if maybe_old_dist.is_none() || new_dist < *maybe_old_dist.unwrap() {
                distances.insert(nei, new_dist);
                prev.insert(nei, vec![(node, *weight)]);
                queue.push(Reverse((dist + weight, nei)));
            } else if new_dist == *maybe_old_dist.unwrap() {
                prev.entry(nei).and_modify(|p| p.push((node, *weight)));
            }
        }
    }

    // rebuild tree by doing a bfs
    let mut size = 0;
    let mut queue: VecDeque<&Node> = VecDeque::from([end_node.unwrap()]);
    let mut visited: HashSet<&Node> = HashSet::new();
    let mut nodes_visited: HashSet<(usize, usize)> = HashSet::new();
    while queue.len() > 0 {
        let node = queue.pop_front().unwrap();

        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);
        if node == &start_node {
            break;
        }

        let prevs = prev.get(node).unwrap();
        for (prev_node, weight) in prevs {
            if node.0 != prev_node.0 || node.1 != prev_node.1 {
                size += weight;
                if nodes_visited.contains(&(node.0, node.1)) {
                    size -= 1;
                }
                nodes_visited.insert((node.0, node.1));
            }
            queue.push_back(prev_node);
        }
    }
    size + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 7036);
        assert_eq!(solve_part_b(&input), 45);

        let sample = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";
        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 11048);
        assert_eq!(solve_part_b(&input), 64);
    }
}
