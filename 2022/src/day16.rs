use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug)]
struct Valve {
    name: (char, char),
    rate: usize,
    links: Vec<(char, char)>,
    index: usize,
}

#[derive(Clone, Debug)]
struct Path {
    visits: Vec<usize>,
    points: i32,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/input15.in").unwrap());
    println!("Day 16:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> HashMap<usize, Valve> {
    let re = Regex::new(
        r"Valve (?<vname>\w+) has flow rate=(?<srate>\d+); tunnels? leads? to valves? (?<sconns>(\w+,? ?)+)",
    )
    .unwrap();
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (_, [vname, srate, sconns, _]) = re.captures(line).unwrap().extract();
            (
                i,
                Valve {
                    name: (vname.chars().nth(0).unwrap(), vname.chars().nth(1).unwrap()),
                    rate: srate.parse().unwrap(),
                    links: sconns
                        .split(", ")
                        .map(|l| (l.chars().nth(0).unwrap(), l.chars().nth(1).unwrap()))
                        .collect(),
                    index: i,
                },
            )
        })
        .collect()
}

fn solve_part_a(input: &HashMap<usize, Valve>) -> usize {
    // initialize adjacency matrix with infinity for all values
    let mut valves: HashMap<usize, Valve> = input.clone();
    valves.insert(
        input.len(),
        Valve {
            name: ('Z', 'Z'),
            rate: 0,
            links: input.values().map(|v| v.name).collect(),
            index: input.len(),
        },
    );

    let mut dists: Vec<Vec<f32>> = (0..valves.len())
        .map(|_| vec![f32::INFINITY; valves.len()])
        .collect();
    // add distances between neighbors ij (1) and between self (0)
    for (i, v) in &valves {
        dists[*i][*i] = 0.0;
        for j_name in &v.links {
            let mut j = valves.len();
            for (jj, vv) in &valves {
                if vv.name == *j_name {
                    j = *jj;
                }
            }
            dists[*i][j] = 1.0;
        }
    }
    // find shortes paths between all pairs using floyd-warshall algorithm
    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                if dists[i][j] > dists[i][k] + dists[k][j] {
                    dists[i][j] = dists[i][k] + dists[k][j];
                }
            }
        }
    }

    let mut paths: Vec<Vec<Path>> = (0..valves.len())
        .map(|_| {
            vec![
                Path {
                    visits: vec![],
                    points: 0
                };
                valves.len()
            ]
        })
        .collect();

    for i in 0..valves.len() {
        for j in 0..valves.len() {
            let visits = if i == j { vec![i] } else { vec![i, j] };
            let points = compute_points(&visits, &dists, &valves);
            paths[i][j] = Path { visits, points };
        }
    }
    println!("Atencao creuzebeck");
    println!("{paths:?}");

    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                let cur_points = paths[i][j].points;
                let new_path = join_paths(&paths[i][k].visits, &paths[k][j].visits);
                let new_points = compute_points(&new_path, &dists, &valves);

                if i == 3 {
                    println!(
                        "{i}-{k}-{j} Old: {:?} ({:?}) New: {:?} ({:?}) ({:?} + {:?})",
                        paths[i][j].visits,
                        paths[i][j].points,
                        new_path,
                        new_points,
                        paths[i][k].visits,
                        paths[k][j].visits
                    );
                }

                if new_points > cur_points {
                    // println!("{:?} is worse than ...", paths[i][j]);
                    // println!("... {:?}", paths[i][j]);
                    paths[i][j] = Path {
                        visits: new_path,
                        points: new_points,
                    };
                    // } else {

                    // println!(
                    //     "{:p}   {:p} is better than ...",
                    //     &paths[i][j],
                    //     &(paths[i][j].visits)
                    // );
                    // println!("... visits: {:p}, points: {:p}", &new_path, &new_points);
                }
            }
        }
    }

    let aa_index = valves
        .values()
        .filter(|v| v.name == ('A', 'A'))
        .take(1)
        .collect::<Vec<&Valve>>()[0]
        .index;
    let best_sequence = &paths[aa_index][input.len()];

    println!("{:?}", best_sequence.visits);
    let mut answer: Vec<(char, char)> = vec![];
    for idx in best_sequence.visits.iter() {
        let mut ans = &('Z', 'Z');
        for (_k, v) in &valves {
            if v.index == *idx {
                ans = &v.name;
            }
        }
        answer.push(*ans);
    }
    println!("{answer:?}");
    println!("{}", best_sequence.points);

    // let options: Vec<(usize, usize)> = valves
    //     .iter()
    //     .filter_map(|(k, v)| {
    //         if *k == ('D', 'D') || *k == ('B', 'B') || *k == ('J', 'J') {
    //             //if v.rate > 0 {
    //             Some((v.rate, v.index))
    //         } else {
    //             None
    //         }
    //     })
    //     .collect();
    // let best_sequence = find_best_sequence(options, &dists);
    // // println!("{:?}", input);
    todo!();
}

fn compute_points(path: &Vec<usize>, dists: &Vec<Vec<f32>>, valves: &HashMap<usize, Valve>) -> i32 {
    let mut total_points = 0;
    let mut time_left = 30;
    let mut prev_pos: Option<&usize> = None;
    for v in path {
        if let Some(prev) = prev_pos {
            time_left -= dists[*prev][*v] as i32;
        }
        time_left -= 1;
        if time_left > 0 {
            total_points += time_left * valves[&v].rate as i32;
        }
        prev_pos = Some(v);
    }
    total_points
}

fn join_paths(path_a: &Vec<usize>, path_b: &Vec<usize>) -> Vec<usize> {
    let destination = path_b.get(path_b.len() - 1).unwrap();
    let a_minus_destination: Vec<usize> = path_a
        .iter()
        .filter(|p| *p != destination)
        .map(|p| *p)
        .collect();
    let b_minus_a: Vec<usize> = path_b
        .iter()
        .filter(|p| !a_minus_destination.contains(*p))
        .map(|p| *p)
        .collect();
    [a_minus_destination, b_minus_a].concat()
}

fn find_best_sequence(
    options: Vec<(usize, usize)>,
    dists: &Vec<Vec<f32>>,
) -> Vec<(usize, usize, usize)> {
    // println!("{}", options.len());
    if options.len() == 1 {
        return vec![(options[0].1, 1, options[0].0)]; //options.iter().map(|(_, index)| *index).collect();
    } else {
        let mut best_opt: Vec<(usize, usize, usize)> = vec![];
        let mut best_score = 0;
        for opt in 0..options.len() {
            let (left, right) = options.split_at(opt);
            let seq = find_best_sequence([left, &right[1..]].concat(), dists);
            // println!(
            //     "{:?} {:?} {:?} {:?}",
            //     seq,
            //     left,
            //     right,
            //     [left, &right[1..]].concat()
            // );
            let (next_idx, time_so_far, score_so_far) = seq[0];
            let total_time = dists[options[opt].1][next_idx] as usize + 1 + time_so_far;
            let score = options[opt].0 * total_time + score_so_far;
            println!("{:?} {:?}", options[opt], seq);
            if score > best_score {
                best_score = score;
                best_opt = [vec![(options[opt].1, total_time, score)], seq.to_vec()].concat();
                //vec![options[opt]].append(seq);
            }
        }
        println!("{best_opt:?}");
        return best_opt;
    }
}

fn solve_part_b(_input: &HashMap<usize, Valve>) -> i64 {
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
        for (k, v) in &input {
            println!("{k}: {v:?}");
        }

        assert_eq!(26, solve_part_a(&input));
        // assert_eq!(56000011, solve_part_b(&input));
    }
}

/*
 *
 *  D-J-B
 *  20 (A)
 *  20 (I)
 *  20 (J)
 *  20 (J)
 *  20+21 (I)
 *  20+21 (A)
 *  20+21 (B)
 *  20+21 (B)
 *  20+21+13 (C)
 *
 *  180+105+13 = 298
 *
 *  D-B-J
 *  20 (C)
 *  20 (B)
 *  20 (B)
 *  20+13 (A)
 *  20+13 (I)
 *  20+13 (J)
 *  20+13 (J)
 *  20+13+21 (I)
 *  20+13+21 (A)
 *
 *  180+78+42 = 300
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
