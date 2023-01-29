use std::{cmp, collections::HashSet, fs};

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/input09.in").unwrap());
    println!("Day 9:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input))
}

fn parse_input(input: String) -> Vec<(char, usize)> {
    input
        .lines()
        // .map(|line| line.bytes().map(|b| (b - b'0') as i32).collect())
        .map(|line| {
            let (dir, steps) = line.split_once(" ").unwrap();
            (dir.chars().next().unwrap(), steps.parse().unwrap())
        })
        .collect()
}

fn abs_diff(val1: i32, val2: i32) -> i32 {
    if val1 > val2 {
        1
    } else if val1 < val2 {
        -1
    } else {
        0
    }
}

fn solve_common(instructions: &Vec<(char, usize)>, rope_size: usize) -> usize {
    let mut rope = vec![(0, 0); rope_size];
    let mut visited_pos: HashSet<(i32, i32)> = HashSet::new();
    visited_pos.insert(*rope.last().unwrap());
    for (dir, steps) in instructions {
        for _ in 0..*steps {
            // move head
            match dir {
                'R' => rope[0] = (rope[0].0 + 1, rope[0].1),
                'L' => rope[0] = (rope[0].0 - 1, rope[0].1),
                'U' => rope[0] = (rope[0].0, rope[0].1 + 1),
                'D' => rope[0] = (rope[0].0, rope[0].1 - 1),
                _ => panic!("invalid direction: {dir}"),
            }

            // move tail
            for i in 1..rope_size {
                if cmp::max(
                    (rope[i - 1].0 - rope[i].0).abs(),
                    (rope[i - 1].1 - rope[i].1).abs(),
                ) <= 1
                {
                    continue;
                }
                rope[i].0 += abs_diff(rope[i - 1].0, rope[i].0);
                rope[i].1 += abs_diff(rope[i - 1].1, rope[i].1);
            }

            visited_pos.insert(*rope.last().unwrap());
        }
    }
    visited_pos.len()
}

fn solve_part_a(instructions: &Vec<(char, usize)>) -> usize {
    solve_common(instructions, 2)
}

fn solve_part_b(instructions: &Vec<(char, usize)>) -> usize {
    solve_common(instructions, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

        let input = parse_input(sample.to_string());

        assert_eq!(13, solve_part_a(&input));
        assert_eq!(1, solve_part_b(&input));

        let sample = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

        let input = parse_input(sample.to_string());
        assert_eq!(36, solve_part_b(&input));
    }
}
