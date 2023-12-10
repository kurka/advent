use num::integer;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub fn solve() {
    let (instructions, network) = parse_input(fs::read_to_string("inputs/08.txt").unwrap());
    println!("Day 08:");
    println!("{}", solve_part_a(&instructions, &network));
    println!("{}", solve_part_b(&instructions, &network));
}

fn parse_input(input: String) -> (Vec<char>, HashMap<String, (String, String)>) {
    let re = Regex::new(r"\((\w\w\w), (\w\w\w)\)").unwrap();
    let (instructions_s, network_s) = input.split_once("\n\n").unwrap();
    let network = network_s
        .lines()
        .map(|line| {
            let (node, leftright) = line.split_once(" = ").unwrap();
            let (_, [left, right]) = re.captures(leftright).unwrap().extract();
            (
                String::from(node),
                (String::from(left), String::from(right)),
            )
        })
        .collect();
    (instructions_s.chars().collect(), network)
}

fn solve_part_a(instructions: &Vec<char>, network: &HashMap<String, (String, String)>) -> usize {
    let mut cur_node = String::from("AAA");
    let mut n_steps: usize = 0;
    for inst in instructions.into_iter().cycle() {
        if cur_node == "ZZZ" {
            break;
        }
        match inst {
            'L' => cur_node = network.get(&cur_node).unwrap().0.clone(), //network[&cur_node].0,
            'R' => cur_node = network.get(&cur_node).unwrap().1.clone(), //network[&cur_node].1,
            _ => unreachable!(),
        }
        n_steps += 1
    }
    n_steps
}

fn solve_part_b(instructions: &Vec<char>, network: &HashMap<String, (String, String)>) -> usize {
    let starts: Vec<&String> = network.keys().filter(|k| k.ends_with("A")).collect();
    let lenghts = starts
        .iter()
        .map(|start_node| {
            let mut cur_node = String::from(*start_node);
            let mut n_steps: usize = 0;
            for inst in instructions.into_iter().cycle() {
                if cur_node.ends_with("Z") {
                    break;
                }
                match inst {
                    'L' => cur_node = network.get(&cur_node).unwrap().0.clone(), //network[&cur_node].0,
                    'R' => cur_node = network.get(&cur_node).unwrap().1.clone(), //network[&cur_node].1,
                    _ => unreachable!(),
                }
                n_steps += 1
            }
            n_steps
        })
        .reduce(|a, b| integer::lcm(a, b))
        .unwrap();
    lenghts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

        let (instructions, network) = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&instructions, &network), 2);

        let sample = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

        let (instructions, network) = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&instructions, &network), 6);

        let sample = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

        let (instructions, network) = parse_input(sample.to_string());

        assert_eq!(solve_part_b(&instructions, &network), 6);

        // assert_eq!(solve_part_b(&instructions, &network), 2);
    }
}
