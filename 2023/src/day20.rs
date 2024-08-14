use num::integer;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug)]
enum Module {
    FlipFlop {
        destination: Vec<String>,
        state: bool,
    },
    Conjunction {
        destination: Vec<String>,
        memory: HashMap<String, bool>,
    },
    Broadcast {
        destination: Vec<String>,
    },
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/20.txt").unwrap());
    println!("Day 20:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> HashMap<String, Module> {
    let re = Regex::new(r"([%&]?)(\w+) -> (.*)").unwrap();
    let mut memories: HashMap<String, Vec<String>> = HashMap::new();
    let mut modules: HashMap<String, Module> = input
        .lines()
        .map(|line| {
            let (_, [mtype, mname, nextlist]) = re.captures(line).unwrap().extract();
            let modname = mname.to_string();
            let dests: Vec<String> = nextlist.split(", ").map(|s| s.to_string()).collect();
            for dest in dests.clone() {
                memories.entry(dest).or_default().push(modname.clone());
            }
            let module = match mtype {
                "" => Module::Broadcast { destination: dests },
                "%" => Module::FlipFlop {
                    destination: dests,
                    state: false,
                },
                "&" => Module::Conjunction {
                    destination: dests,
                    memory: HashMap::new(),
                },
                _ => unreachable!(),
            };
            (modname, module)
        })
        .collect();

    for (modname, module) in modules.iter_mut() {
        // write memory components into conjunction objects
        if let Module::Conjunction {
            destination: _,
            memory,
        } = module
        {
            let new_memory: HashMap<String, bool> = memories
                .get(modname)
                .unwrap()
                .iter()
                .map(|name| (name.clone(), false))
                .collect();
            *memory = new_memory;
        }
    }

    for output in memories.keys() {
        if !modules.contains_key(output) {
            modules.insert(
                output.clone(),
                Module::Broadcast {
                    destination: Vec::new(),
                },
            );
        }
    }

    modules
}

fn solve_part_a(input: &HashMap<String, Module>) -> usize {
    let rounds = 1000;
    let mut lows = 0;
    let mut highs = 0;

    let mut modules = input.clone();

    for _round in 0..rounds {
        // print state
        // let round_state: Vec<u8> = modules
        //     .values()
        //     .flat_map(|m| match m {
        //         Module::FlipFlop { destination, state } => vec![if *state { 1 } else { 0 }],
        //         Module::Conjunction {
        //             destination,
        //             memory,
        //         } => memory.values().map(|b| if *b { 1 } else { 0 }).collect(),
        //         Module::Broadcast { destination } => vec![],
        //     })
        //     .collect();
        // println!("{:03} {:?}", round, round_state);

        let mut next: Vec<(String, bool, String)> = match modules.get_key_value("broadcaster") {
            Some((broadcast_key, Module::Broadcast { destination })) => destination
                .iter()
                .map(|d| ((*d).clone(), false, (*broadcast_key).clone()))
                .collect(),
            _ => unreachable!(),
        };
        lows += 1;
        // println!("next: {next:?}");
        while !next.is_empty() {
            let mut next_next: Vec<(String, bool, String)> = Vec::new();
            for (name, pulse, sender) in next {
                // update counts
                match pulse {
                    true => highs += 1,
                    false => lows += 1,
                }

                // propagate signal
                // println!("Searching for {name:?}");
                let next_mod = modules.get_mut(&name).unwrap();
                match next_mod {
                    Module::FlipFlop { destination, state } => match pulse {
                        true => continue,
                        false => {
                            *state = !*state;
                            let ff_next: Vec<(String, bool, String)> = destination
                                .iter()
                                .map(|d| ((*d).clone(), *state, name.clone()))
                                .collect();
                            next_next.extend(ff_next);
                        }
                    },
                    Module::Conjunction {
                        destination,
                        memory,
                    } => {
                        memory.insert(sender.clone(), pulse);
                        let new_state = !memory.values().all(|p| *p);
                        let c_next: Vec<(String, bool, String)> = destination
                            .iter()
                            .map(|d| ((*d).clone(), new_state, name.clone()))
                            .collect();
                        next_next.extend(c_next);
                    }
                    Module::Broadcast { destination } => {
                        let b_next: Vec<(String, bool, String)> = destination
                            .iter()
                            .map(|d| ((*d).clone(), pulse, name.clone()))
                            .collect();
                        next_next.extend(b_next)
                    }
                }
            }
            next = next_next;
        }
    }

    highs * lows
}

fn solve_part_b(_input: &HashMap<String, Module>) -> usize {
    // From the inspection of a graph generated from the instruction, a clear structure became clear:
    // rx is connected to 4 conjunction models, each connected to 12 fliflops in sequence.
    // The 12 flipflops in sequence act as a binary counter with 12 bits, and are reseted every 2**12
    // However, each conjunction is connected to just some of the 12 fliflops, so it *resets* the counter
    // everytime all connected flipflops output 1.
    //
    // In the end, we just need to consider each block of 12 fliflops + conjunction as a binary counter and
    // find the lcm of the binary representation of each

    let conj_a = 0b111100000111;
    let conj_b = 0b111010110001;
    let conj_c = 0b111011010001;
    let conj_d = 0b111100101001;
    [conj_a, conj_b, conj_c, conj_d]
        .into_iter()
        .reduce(|a, b| integer::lcm(a, b))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 32000000);

        let sample = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";
        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 11687500);
    }
}
