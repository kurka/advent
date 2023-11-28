use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug)]
struct Valve {
    name: String,
    rate: usize,
    links: Vec<String>,
    index: usize,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/input15.in").unwrap());
    println!("Day 16:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> HashMap<String, Valve> {
    let re = Regex::new(
        r"Valve (?<vname>\w+) has flow rate=(?<srate>\d+); tunnels? leads? to valves? (?<sconns>(\w+,? ?)+)",
    )
    .unwrap();
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (_, [vname, srate, sconns, _]) = re.captures(line).unwrap().extract();
            let name = vname.parse().unwrap();
            (
                name,
                Valve {
                    name: name,
                    rate: srate.parse().unwrap(),
                    links: sconns.split(", ").map(|l| l.parse().unwrap()).collect(),
                    index: i,
                },
            )
        })
        .collect()
}

fn solve_part_a(input: &HashMap<String, Valve>) -> usize {
    // initialize adjacency matrix with infinity for all values
    let mut dists: Vec<Vec<f32>> = (0..input.len())
        .map(|_| vec![f32::INFINITY; input.len()])
        .collect();
    // add distances between neighbors ij (1) and between self (0)
    for (_, v) in input {
        let i = v.index;
        dists[i][i] = 0.0;
        for j_name in &v.links {
            let j = input.get(j_name).unwrap().index;
            dists[i][j] = 1.0;
        }
    }
    // find shortes paths between all pairs using floyd-warshall algorithm
    for (_, v_k) in input {
        let k = v_k.index;
        for (_, v_i) in input {
            let i = v_i.index;
            for (_, v_j) in input {
                let j = v_j.index;
                if dists[i][j] > dists[i][k] + dists[k][j] {
                    dists[i][j] = dists[i][k] + dists[k][j];
                }
            }
        }
    }

    let options: Vec<Valve> = input.into_values().collect();
    let best_sequence = find_best_sequence(options, &dists);
    println!("{:?}", best_sequence);

    todo!();
}

fn find_best_sequence(options: Vec<Valve>, dists: &Vec<Vec<f32>>) -> Vec<Valve> {
    if options.len() == 1 {
        return options;
    } else {
        let mut best_opt: Vec<Valve> = vec![];
        let mut best_score = 0.0;
        for opt in 0..options.len() {
            let (left, right) = options.split_at(opt);
            let seq = find_best_sequence([left, right].concat(), dists);
            let score = (options[opt].rate as f32) * dists[options[opt].index][seq[0].index];
            if score > best_score {
                best_score = score;
                best_opt = [vec![options[opt]], seq.to_vec()].concat(); //vec![options[opt]].append(seq);
            }
        }
        return best_opt;
    }
}

fn solve_part_b(_input: &HashMap<String, Valve>) -> i64 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II\
";

        let input = parse_input(sample.to_string());
        println!("{input:?}");

        assert_eq!(26, solve_part_a(&input));
        // assert_eq!(56000011, solve_part_b(&input));
    }
}

/*
 *
 *
 *            A(0)
 *           /  | \
 *     (21)J |  |  B(13)
 *         | |  |  |
 *      (0)I_/  |  C(2)
 *              |  |
 *     (22)H    \__D(20)
 *         |       |
 *      (0)G       E(3)
 *         |       |
 *         \__F(0)_/
 *
 *
 *              0
 *           /  | \
 *        21 |  |  13
 *         | |  |  |
 *         0_/  |  2
 *              |  |
 *        22    \__20
 *         |       |
 *         0       3
 *         |       |
 *         \___0___/
 *
 * t = 28
 *
 *              0
 *           /  | \
 *(24*21) 504|  |  325 (25*13)
 *         | |  |  |
 *         0_/  |  52 (26*2)
 *              |  |
 *(23*22) 506   \__0
 *         |       |
 *         0       48 (26*3)
 *         |       |
 *         \___0___/
 *
 *
 *
 *
 *
 */
