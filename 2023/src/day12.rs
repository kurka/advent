use std::collections::HashMap;
use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/12.txt").unwrap());
    println!("Day 12:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<(Vec<char>, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (record, numbers_str) = line.split_once(" ").unwrap();
            (
                record.chars().collect(),
                numbers_str.split(',').map(|n| n.parse().unwrap()).collect(),
            )
        })
        .collect()
}

fn solve_part_a(input: &Vec<(Vec<char>, Vec<usize>)>) -> usize {
    solve_11(input, 1)
}

fn solve_part_b(input: &Vec<(Vec<char>, Vec<usize>)>) -> usize {
    solve_11(input, 5)
}

fn solve_11(input: &Vec<(Vec<char>, Vec<usize>)>, repetitions: usize) -> usize {
    input
        .iter()
        .map(|(record, springs_groups)| {
            // let rec = [record].repeat(repetitions).join(&'?');
            let mut rec = record.clone();
            for _ in 0..repetitions - 1 {
                rec.push('?');
                rec.extend(record.clone());
            }
            // println!("{rec:?}");
            let springs = springs_groups.repeat(repetitions);
            let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
            let solutions = place_springs(&rec, &springs, 0, 0, &mut cache);
            if solutions == 0 {
                println!("Warning: empty solution for {record:?} {springs_groups:?}");
            }
            solutions
        })
        .sum()
}

fn place_springs(
    record: &Vec<char>,
    groups: &Vec<usize>,
    start_r: usize,
    start_g: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(v) = cache.get(&(start_r, start_g)) {
        return *v;
    }
    // println!("{start_r} {start_g} Analyzing {record:?} and {groups:?}");
    if start_g >= groups.len() {
        if start_r >= record.len() {
            return 1;
        } else if record[start_r..].iter().any(|r| *r == '#') {
            return 0;
        } else {
            return 1;
        }
    }

    let target_size = groups[start_g];

    let mut solutions = 0;
    for i in start_r..record.len() {
        // check if explicit #s were already consumed
        if (start_r..i).any(|pi| record[pi] == '#') {
            break;
        }
        if i + target_size <= record.len()
            && (0..target_size).all(|j| record[i + j] != '.')
            && (i + target_size == record.len() || record[i + target_size] != '#')
        {
            // println!("({}) Found pos {i} to {target_size}", groups.len());
            let mut next_r = i + target_size + 1;
            while next_r < record.len() && record[next_r] == '.' {
                next_r += 1;
            }
            let new_solutions = place_springs(&record, &groups, next_r, start_g + 1, cache);
            // println!("({} (Back)) solutions: {new_solutions}", groups.len());
            // if new_solutions == 0 {
            //     break;
            // }
            solutions += new_solutions;
        }
    }
    cache.insert((start_r, start_g), solutions);
    solutions
}

// fn place_springs_orig(record: &[char], groups: &[usize]) -> usize {
//     // println!("Analyzing {record:?} and {groups:?}");
//     if groups.len() == 0 {
//         if record.iter().any(|r| *r == '#') {
//             return 0;
//         } else {
//             return 1;
//         }
//     }

//     let target_size = groups[0];

//     let mut solutions = 0;
//     for i in 0..record.len() {
//         // check if explicit #s were already consumed
//         if (0..i).any(|pi| record[pi] == '#') {
//             break;
//         }
//         if i + target_size <= record.len()
//             && (0..target_size).all(|j| record[i + j] != '.')
//             && (i + target_size == record.len() || record[i + target_size] != '#')
//         {
//             let rest_records = if record.len() > i + target_size + 1 {
//                 &record[i + target_size + 1..]
//             } else {
//                 &[]
//             };
//             // println!("({}) Found pos {i} to {target_size}", groups.len());
//             let new_solutions = place_springs_orig(rest_records, &groups[1..]);
//             // println!("({} (Back)) solutions: {new_solutions}", groups.len());
//             // if new_solutions == 0 {
//             //     break;
//             // }
//             solutions += new_solutions;
//         }
//     }
//     solutions
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 21);
        assert_eq!(solve_part_b(&input), 525152);
        let sample = "\
??????.#??.. 2,2
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 5);
    }
}
