use regex::Regex;
use std::{
    collections::{HashMap, HashSet, VecDeque},
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

fn solve_part_b(input: &DayInput) -> i32 {
    // solve using bfs
    input
        .machines
        .iter()
        .enumerate()
        .map(|(machine_i, machine)| {
            println!(
                "Machine {}: {:?} {:?}\n{:?}",
                machine_i, machine.target, machine.joltage, machine.schemas
            );
            let n_lights = machine.target.len();
            let n_buttons = machine.schemas.len();

            // sort lights by the number of buttons that control it
            let mut sorted_lights: Vec<usize> = (0..n_lights).collect();
            sorted_lights.sort_by_key(|light_i| {
                (
                    machine
                        .schemas
                        .iter()
                        .filter(|button| button.contains(&light_i))
                        .count(),
                    machine.joltage[*light_i],
                )
            });

            let sorted_lights_map: HashMap<usize, usize> =
                sorted_lights.into_iter().enumerate().collect();

            let mut sorted_buttons = machine.schemas.clone();
            sorted_buttons.sort_by_key(|button| {
                let mut mapped_buttons = button
                    .into_iter()
                    .map(|b| *sorted_lights_map.get(b).unwrap())
                    .collect::<Vec<usize>>();
                mapped_buttons.sort();
                mapped_buttons
            });

            println!("Sorted buttons: {sorted_lights_map:?}\n{sorted_buttons:?}");

            // create system Ax = c
            // where:
            // l = number of lights
            // b = number of buttons
            // A -> Matrix correlating lights and buttons
            // |A| = l*b
            // A_ij = 1 if light `i` is affected by button `j`
            // x -> Vector with number of button presses
            // |x| = b*1
            // c -> vector with counts of lights activations (joltage), given
            // |c| = l*1

            let light_to_buttons: Vec<Vec<bool>> = (0..n_lights)
                .map(|light_i| {
                    sorted_buttons
                        .iter()
                        .map(|button_j| button_j.contains(&light_i))
                        .collect()
                })
                .collect();
            let mut state = Vec::with_capacity(n_buttons);
            constrained_search(
                0,
                &mut state,
                n_buttons,
                &machine.joltage,
                &light_to_buttons,
            ).unwrap()

            // Faster alternative, but with a bug somewhere that I couldn't find:

            // let mut state = vec![0; n_buttons];
            // let mut resolutions = vec![false; n_buttons];
            // let mut ranges: Vec<(i32, i32)> = sorted_buttons
            //     .iter()
            //     .map(|button_lights| {
            //         (
            //             0,
            //             button_lights
            //                 .iter()
            //                 .map(|light_i| machine.joltage[*light_i])
            //                 .min()
            //                 .unwrap() as i32,
            //         )
            //     })
            //     .collect();
            // constrained_search_inorder(
            //     &mut state,
            //     &mut resolutions,
            //     &mut ranges,
            //     n_buttons,
            //     n_lights,
            //     &machine.joltage,
            //     &light_to_buttons,
            // )
            // .unwrap()
        })
        .sum()
}

fn constrained_search(
    button: usize,
    state: &mut Vec<i32>,
    n_buttons: usize,
    target_joltage: &Vec<usize>,
    light_to_buttons: &Vec<Vec<bool>>,
) -> Option<i32> {
    if button == n_buttons {
        let total = state.iter().sum();
        println!("Found solution {total} {state:?}");
        return Some(total);
    }

    // find min and max possible values for this button
    let joltage_left_per_light: Vec<(i32, bool)> = light_to_buttons
        .iter()
        .enumerate()
        .filter_map(|(light_i, button_row)| {
            if !button_row[button] {
                return None
            }
            let joltage_already_used: i32 = state
                .iter()
                .zip(button_row)
                .map(|(count, active)| *count * (*active as i32))
                .sum();
            let is_last_constraint_for_light = !button_row[(button + 1)..].iter().any(|x| *x);
            Some((
                target_joltage[light_i] as i32 - joltage_already_used,
                is_last_constraint_for_light,
            ))
        })
        .collect();
    // max_val is equal to the minimum joltage left for a given light
    let max_val = *joltage_left_per_light
        .iter()
        .map(|(joltage_left, _)| joltage_left)
        .min()
        .unwrap();
    if max_val < 0 {
        return None;
    }

    // min_val is given by the maximum value where the joltage is the last left
    let min_val = joltage_left_per_light
        .iter()
        .filter_map(|(joltage_left, is_last)| if *is_last { Some(*joltage_left) } else { None })
        .max()
        .unwrap_or(0);

    if min_val > max_val {
        return None;
    }

    let mut best_result = None;
    for count in min_val..=max_val {
        state.push(count);
        let maybe_res = constrained_search(
            button + 1,
            state,
            n_buttons,
            target_joltage,
            light_to_buttons,
        );
        state.pop();
        if let Some(res) = maybe_res && best_result.is_none_or(|best_result| best_result > res) {
            best_result = maybe_res;
         }
     }

     best_result
 }


fn constrained_search_inorder(
    state: &mut Vec<i32>,
    resolutions: &mut Vec<bool>,
    ranges: &mut Vec<(i32, i32)>,
    n_buttons: usize,
    n_lights: usize,
    target_joltage: &Vec<usize>,
    light_to_buttons: &Vec<Vec<bool>>,
) -> Option<i32> {
    if resolutions.iter().all(|x| *x) {
        let total = state.iter().sum();
        println!("Found solution {total} {state:?}");
        return Some(total);
    }

    // pick the button with minimum range available
    let (button, (min_val, max_val)) = ranges
        .iter()
        .enumerate()
        .filter(|(b, _)| !resolutions[*b])
        .min_by_key(|(_, (min_val, max_val))| max_val - min_val)
        .unwrap();
    resolutions[button] = true;
    // println!("S {button} {resolutions:?} {ranges:?} {state:?}");

    let mut best_result = None;
    for count in *min_val..=*max_val {
        state[button] = count;
        // println!("count: {count}");

        // update ranges
        let joltage_left_per_light: Vec<i32> = light_to_buttons
            .iter()
            .enumerate()
            .map(|(light_i, button_row)| {
                let joltage_already_used: i32 = (0..n_buttons)
                    .map(|b| {
                        if button_row[b] && resolutions[b] {
                            state[b]
                        } else {
                            0
                        }
                    })
                    .sum();
                target_joltage[light_i] as i32 - joltage_already_used
            })
            .collect();
        let mut created_contradiction = false;
        for other_button in 0..n_buttons {
            // only update ranges of buttons that were not resolved.
            if resolutions[other_button]
            // || !(0..n_lights).any(|light_i| {
            //     light_to_buttons[light_i][button] && light_to_buttons[light_i][other_button]
            // })
            {
                continue;
            }
            let other_max_val = (0..n_lights)
                .filter_map(|light_i| {
                    if light_to_buttons[light_i][other_button] {
                        Some(joltage_left_per_light[light_i])
                    } else {
                        None
                    }
                })
                .min()
                .unwrap();
            let other_min_val = (0..n_lights)
                .filter_map(|light_i| {
                    if light_to_buttons[light_i][other_button]
                        // count the number of unresolved buttons in row
                        && light_to_buttons[light_i]
                            .iter()
                            .enumerate()
                            .filter(|(button_id, button_active)| {
                                **button_active && !resolutions[*button_id]
                            })
                            .count()
                            == 1
                    {
                        Some(joltage_left_per_light[light_i])
                    } else {
                        None
                    }
                })
                .max()
                .unwrap_or(0);
            if other_max_val < 0 || other_min_val > other_max_val {
                // skip this count, as it generates a contradiction elsewhere
                created_contradiction = true;
                break;
            }
            ranges[other_button] = (other_min_val, other_max_val);
        }
        if created_contradiction {
            continue;
        }

        let maybe_res = constrained_search_inorder(
            state,
            resolutions,
            ranges,
            n_buttons,
            n_lights,
            target_joltage,
            light_to_buttons,
        );
        // println!("Back from recursion: res == {maybe_res:?}");
        if maybe_res.is_some_and(|res| best_result.is_none_or(|best_result| best_result > res)) {
            best_result = maybe_res;
        }
    }

    resolutions[button] = false;
    best_result
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
