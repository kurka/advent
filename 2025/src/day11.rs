use std::{
    collections::{HashMap, VecDeque},
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
    counter.insert(target, 0);
    let mut queue: VecDeque<&String> = VecDeque::from([start]);

    while queue.len() > 0 {
        let node = queue.pop_front().unwrap();

        // if node == target {
        //     break;
        // }
        for neighbor in input.graph.get(node).unwrap().iter() {
            counter.entry(neighbor).and_modify(|c| *c += 1);
            if neighbor != target {
                queue.push_back(neighbor);
            }
        }
    }

    *counter.get(target).unwrap()
}

//
//         you
//      /      \
//   bbb        ccc
//     \ /  \/       \
//     ddd  eee      ffff
//      |     |     /
//     ggg    |    /
//        \   |   /
//          out
//
//
//

fn solve_part_b(input: &DayInput) -> usize {
    todo!()
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
        assert_eq!(solve_part_b(&input), 1337);
    }
}
