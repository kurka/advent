use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

#[derive(Clone, Debug)]
struct DayInput {
    graph: HashMap<String, Vec<String>>,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/11.txt").unwrap());
    println!("Day 11:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> DayInput {
    let graph = input
        .lines()
        .map(|line| {
            let (source_str, target_str) = line.split_once(": ").unwrap();
            (
                source_str.to_string(),
                target_str
                    .split_whitespace()
                    .map(|t| t.to_string())
                    .collect(),
            )
        })
        .collect();
    DayInput { graph }
}

fn solve_part_a(input: &DayInput) -> usize {
    let start = &String::from("you");
    let target = &String::from("out");

    let mut counter: HashMap<&String, usize> = input.graph.keys().map(|k| (k, 0)).collect();
    counter.insert(start, 1);
    counter.insert(target, 0);
    let mut queue: VecDeque<&String> = VecDeque::from([start]);
    let mut visited: HashSet<&String> = HashSet::from([start, target]);

    while queue.len() > 0 {
        let node = queue.pop_front().unwrap();

        let node_count = counter.get(node).unwrap().clone();

        for neighbor in input.graph.get(node).unwrap().iter() {
            counter.entry(neighbor).and_modify(|c| *c += node_count);
            if !visited.contains(neighbor) {
                queue.push_back(neighbor);
                visited.insert(neighbor);
            }
        }
    }
    *counter.get(target).unwrap()
}

fn solve_part_b(input: &DayInput) -> usize {
    let start = &String::from("svr");
    let mid_a = &String::from("dac");
    let mid_b = &String::from("fft");
    let end = &String::from("out");

    let count_mid_a = bfs(input, start, 1, mid_a, end);
    let count_mid_b = bfs(input, start, 1, mid_b, end);
    let count_mid_a_b = bfs(input, mid_a, count_mid_a, mid_b, end);
    let count_mid_b_a = bfs(input, mid_b, count_mid_b, mid_a, end);
    let count_a_end = bfs(input, mid_a, count_mid_b_a, end, end);
    let count_b_end = bfs(input, mid_b, count_mid_a_b, end, end);

    count_a_end + count_b_end
}

fn bfs(
    input: &DayInput,
    start: &String,
    start_count: usize,
    target: &String,
    end: &String,
) -> usize {
    // get the subgraph formed for all nodes reached from start (ignore paths that started outside it)
    let mut nodes_reached_from_start: HashSet<&String> = HashSet::from([start]);
    let mut queue: VecDeque<&String> = VecDeque::from([start]);
    while queue.len() > 0 {
        let node = queue.pop_front().unwrap();
        if node == end {
            continue;
        }
        for neighbor in input.graph.get(node).unwrap().iter() {
            if nodes_reached_from_start.insert(neighbor) {
                queue.push_back(neighbor);
            }
        }
    }
    let mut graph: HashMap<&String, Vec<&String>> = input
        .graph
        .iter()
        .filter_map(|(k, v)| {
            if nodes_reached_from_start.contains(k) {
                Some((
                    k,
                    v.iter()
                        .filter(|vv| nodes_reached_from_start.contains(vv))
                        .collect(),
                ))
            } else {
                None
            }
        })
        .collect();
    graph.insert(end, vec![]);

    let mut counter: HashMap<&String, usize> = graph.keys().map(|k| (*k, 0)).collect();
    let mut degrees: HashMap<&String, usize> = graph.keys().map(|k| (*k, 0)).collect();
    for nodes in graph.values() {
        for node in nodes {
            degrees.entry(node).and_modify(|degree| *degree += 1);
        }
    }
    counter.insert(start, start_count);
    counter.insert(end, 0);
    let mut queue: VecDeque<&String> = VecDeque::from([start]);

    while queue.len() > 0 {
        let node = queue.pop_front().unwrap();
        if node == target {
            break;
        }

        let node_count = counter.get(node).unwrap_or(&0_usize).clone();

        for neighbor in graph.get(node).unwrap().iter() {
            counter
                .entry(neighbor)
                .and_modify(|c| *c += node_count)
                .or_insert(0);
            degrees.entry(neighbor).and_modify(|degree| *degree -= 1);
            if *degrees.get(neighbor).unwrap() == 0 {
                //&& neighbor != end {
                queue.push_back(neighbor);
            }
        }
    }
    *counter.get(target).unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 5);

        let sample = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        let input = parse_input(sample.to_string());
        assert_eq!(solve_part_b(&input), 2);
    }
}
