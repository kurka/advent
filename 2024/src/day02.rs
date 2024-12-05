use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/02.txt").unwrap());
    println!("Day 01:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.split(" ").map(|num| num.parse().unwrap()).collect())
        .collect()
}

fn solve_part_a(input: &Vec<Vec<i32>>) -> usize {
    solve_day2(input, false)
}

fn solve_part_b(input: &Vec<Vec<i32>>) -> usize {
    solve_day2(input, true)
}
fn solve_day2(input: &Vec<Vec<i32>>, can_forgive: bool) -> usize {
    input
        .iter()
        .map(|seq| {
            if check_valid(seq, 1, 0, seq[1] < seq[0], can_forgive) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn check_valid(
    seq: &Vec<i32>,
    i: usize,
    prev_i: usize,
    decreasing: bool,
    can_forgive: bool,
) -> bool {
    if i >= seq.len() {
        return true;
    }

    let diff = seq[i] - seq[prev_i];
    if diff.abs() > 3 || diff == 0 || (diff < 0) != decreasing {
        if can_forgive {
            match i {
                1 => {
                    return check_valid(seq, i + 1, i - 1, seq[i + 1] < seq[i - 1], false) // skip i
                        || check_valid(seq, i + 1, i, seq[i + 1] < seq[i], false);
                    // skip i-1
                }
                2 => {
                    return check_valid(seq, i + 1, i-1, decreasing, false) // skip i
                        || check_valid(seq, i, i-2, decreasing, false) // skip i-1
                        || check_valid(seq, i, i-1, seq[i] < seq[i-1], false); // skip i-2
                }
                _ => {
                    return check_valid(seq, i + 1, i-1, decreasing, false) // skip i
                        || check_valid(seq, i, i-2, decreasing, false); // skip i-1
                }
            }
        } else {
            return false;
        }
    } else {
        return check_valid(seq, i + 1, i, decreasing, can_forgive);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 2);
        assert_eq!(solve_part_b(&input), 4);
    }
}
