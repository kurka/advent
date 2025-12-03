use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/01.txt").unwrap());
    println!("Day 01:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            let (dir, val_str) = line.split_at(1);
            val_str.parse::<i32>().unwrap() * if dir == "L" { -1 } else { 1 }
        })
        .collect()
}

fn solve_part_a(input: &Vec<i32>) -> usize {
    let (_, zeros, _) = solve_day_1(input);
    zeros
}

fn solve_part_b(input: &Vec<i32>) -> usize {
    let (_, _, laps) = solve_day_1(input);
    laps
}

fn solve_day_1(input: &Vec<i32>) -> (i32, usize, usize) {
    let dial_size = 100;
    input
        .iter()
        .fold((50, 0, 0), |(pos, zeros, laps), rotation| {
            let new_pos = pos + rotation;
            (
                new_pos.rem_euclid(dial_size),                         // next pos
                zeros + (new_pos.rem_euclid(dial_size) == 0) as usize, // count zeros
                laps + (new_pos / dial_size).abs() as usize + (new_pos <= 0 && pos != 0) as usize, // count laps
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 3);
        assert_eq!(solve_part_b(&input), 6);
        assert_eq!(solve_part_b(&vec![1000]), 10);
    }
}
