use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/input04.in").unwrap());
    println!("Day 4:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input))
}

fn parse_input(input: String) -> Vec<((i32, i32), (i32, i32))> {
    input
        .lines()
        .map(|line| {
            let (pair1, pair2) = line.split_once(",").unwrap();
            let (lim1_1, lim1_2) = pair1.split_once("-").unwrap();
            let (lim2_1, lim2_2) = pair2.split_once("-").unwrap();
            (
                (lim1_1.parse().unwrap(), lim1_2.parse().unwrap()),
                (lim2_1.parse().unwrap(), lim2_2.parse().unwrap()),
            )
        })
        .collect()
}

fn solve_part_a(pairs: &Vec<((i32, i32), (i32, i32))>) -> usize {
    pairs
        .iter()
        .filter(|((p11, p12), (p21, p22))| (p11 >= p21 && p12 <= p22) || (p21 >= p11 && p22 <= p12))
        .count()
}

fn solve_part_b(pairs: &Vec<((i32, i32), (i32, i32))>) -> usize {
    pairs
        .iter()
        .filter(|((p11, p12), (p21, p22))| (p11 <= p21 && p12 >= p21) || (p21 <= p11 && p22 >= p11))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        let input = parse_input(sample.to_string());

        assert_eq!(2, solve_part_a(&input));
        assert_eq!(4, solve_part_b(&input));
    }
}
