use regex::Regex;
use std::{
    collections::{HashSet, VecDeque},
    fs,
};

#[derive(Clone, Debug)]
struct Machine {
    target: Vec<bool>,
    schemas: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

#[derive(Clone, Debug)]
struct DayInput {
    machines: Vec<Machine>,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/10.txt").unwrap());
    println!("Day 10:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> DayInput {
    let re_machine = Regex::new(r"\[(.*)\] (\(.*\)) \{(.*)\}").unwrap();
    let re_schemas = Regex::new(r"\((\S+)\)").unwrap();

    let machines = input
        .lines()
        .map(|line| {
            let (_, [target_str, schemas_str, joltage_str]) =
                re_machine.captures(line).unwrap().extract();
            let target = target_str
                .chars()
                .map(|c| if c == '#' { true } else { false })
                .collect();

            let schemas = re_schemas
                .captures_iter(schemas_str)
                .map(|caps| {
                    let (_, [toggles_str]) = caps.extract();
                    toggles_str
                        .split(',')
                        .map(|digit| digit.parse().unwrap())
                        .collect()
                })
                .collect();

            let joltage = joltage_str
                .split(',')
                .map(|digit| digit.parse().unwrap())
                .collect();

            Machine {
                target: target,
                schemas: schemas,
                joltage: joltage,
            }
        })
        .collect();
    DayInput { machines: machines }
}

fn solve_part_a(input: &DayInput) -> usize {
    // solve using bfs
    input
        .machines
        .iter()
        .map(|machine| {
            let n_lights = machine.target.len();
            let initial_state = vec![false; n_lights];
            let mut queue = VecDeque::from([(initial_state, 0)]);
            let mut visited: HashSet<Vec<bool>> = HashSet::new();

            while !queue.is_empty() {
                let (node, dist) = queue.pop_front().unwrap();
                if node == machine.target {
                    return dist;
                }

                for schema in &machine.schemas {
                    let neighbor = flip(&node, schema);
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor.clone());
                        queue.push_back((neighbor, dist + 1));
                    }
                }
            }
            unreachable!()
        })
        .sum()
}

fn flip(state: &Vec<bool>, instruction: &Vec<usize>) -> Vec<bool> {
    let new_state = state
        .clone()
        .iter()
        .enumerate()
        .map(|(i, s)| if instruction.contains(&i) { !*s } else { *s })
        .collect();

    new_state
}

fn solve_part_b(input: &DayInput) -> usize {
    // solve using bfs
    input
        .machines
        .iter()
        .map(|machine| {
            println!("Machine: {:?} {:?}", machine.target, machine.joltage);
            let n_lights = machine.target.len();
            let initial_state = vec![0; n_lights];
            let mut queue = VecDeque::from([(initial_state, 0)]);
            let mut visited: HashSet<Vec<usize>> = HashSet::new();

            while !queue.is_empty() {
                let (counters, dist) = queue.pop_front().unwrap();
                // println!("Trying {state:?}  {counters:?}");
                if counters == machine.joltage {
                    return dist;
                }

                for schema in &machine.schemas {
                    let neighbor = flip3(&counters, schema);
                    if visited.contains(&neighbor)
                        || neighbor
                            .iter()
                            .zip(&machine.joltage)
                            .any(|(nc, jc)| nc > jc)
                    {
                        continue;
                    }
                    visited.insert(neighbor.clone());
                    queue.push_back((neighbor, dist + 1));
                }
            }
            unreachable!()
        })
        .sum()
}

fn flip2(
    state: &Vec<bool>,
    counters: &Vec<usize>,
    instruction: &Vec<usize>,
) -> (Vec<bool>, Vec<usize>) {
    let new_state = state
        // .clone()
        .iter()
        .zip(counters)
        .enumerate()
        .map(|(i, (s, c))| {
            if instruction.contains(&i) {
                (!*s, *c + 1)
            } else {
                (*s, *c)
            }
        })
        .unzip();

    new_state
}
fn flip3(counters: &Vec<usize>, instruction: &Vec<usize>) -> Vec<usize> {
    let new_state = counters
        // .clone()
        .iter()
        .enumerate()
        .map(|(i, c)| if instruction.contains(&i) { *c + 1 } else { *c })
        .collect();

    new_state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 7);
        assert_eq!(solve_part_b(&input), 33);
    }
}
