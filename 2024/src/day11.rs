use std::{collections::HashMap, fs};

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/11.txt").unwrap());
    println!("Day 11:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<usize> {
    input
        .trim()
        .split(" ")
        .map(|c| c.parse().unwrap())
        .collect()
}

fn solve_part_a(input: &Vec<usize>) -> usize {
    solve_day11(input, 25)
}

fn solve_part_b(input: &Vec<usize>) -> usize {
    solve_day11(input, 75)
}

fn count_succ(memo: &mut HashMap<(usize, usize), usize>, stone: usize, steps: usize) -> usize {
    if steps == 0 {
        return 1;
    }
    if let Some(res) = memo.get(&(stone, steps)) {
        return *res;
    }

    let res = if stone == 0 {
        count_succ(memo, 1, steps - 1)
    } else if ((stone.ilog10() + 1) % 2) == 0 {
        let power = (stone.ilog10() + 1) / 2;
        let left = stone / (usize::pow(10, power));
        let right = stone - left * (usize::pow(10, power));
        count_succ(memo, left, steps - 1) + count_succ(memo, right, steps - 1)
    } else {
        count_succ(memo, stone * 2024, steps - 1)
    };
    memo.insert((stone, steps), res);
    res
}

fn solve_day11(input: &Vec<usize>, steps: usize) -> usize {
    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();
    input
        .iter()
        .map(|stone| count_succ(&mut memo, *stone, steps))
        .sum()
    // let mut cur_vec = input.clone();
    // for _ in 0..steps {
    //     let next_vec: Vec<usize> = cur_vec
    //         .iter()
    //         .flat_map(|stone| {
    //             if *stone == 0 {
    //                 [Some(1), None]
    //             } else if ((stone.ilog10() + 1) % 2) == 0 {
    //                 let power = (stone.ilog10() + 1) / 2;
    //                 let left = stone / (usize::pow(10, power));
    //                 [Some(left), Some(stone - left * (usize::pow(10, power)))]
    //             } else {
    //                 [Some(stone * 2024), None]
    //             }
    //         })
    //         .flatten()
    //         .collect();
    //     cur_vec = next_vec;
    //     println!("{cur_vec:?}")
    // }
    // cur_vec.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
125 17
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 55312);
    }
}

// 0 37551 469 63 1 791606 2065 9983586
// 1
// 2024
// 20 24
// 2 0 2 4
// 4048 1 4048
