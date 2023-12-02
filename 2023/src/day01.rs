use regex::Regex;
use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/01.txt").unwrap());
    println!("Day 01:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<Vec<String>> {
    let re = Regex::new(
        r"(?x)
        # literal digit
        \d
        # overlapped digits
        |oneight
        |twone
        |threeight
        |fiveight
        |eightwo
        |eightree
        |nineight
        # spelled out digits
        |one
        |two
        |three
        |four
        |five
        |six
        |seven
        |eight
        |nine
",
    )
    .unwrap();
    input
        .lines()
        .map(|line| {
            re.find_iter(line)
                .map(|n| String::from(n.as_str()))
                .collect()
        })
        .collect()
}

fn solve_part_a(input: &Vec<Vec<String>>) -> usize {
    solve_01(input, false)
}

fn solve_part_b(input: &Vec<Vec<String>>) -> usize {
    solve_01(input, true)
}

fn solve_01(input: &Vec<Vec<String>>, part_b: bool) -> usize {
    fn parse_spelled_num(num: &str) -> Vec<usize> {
        match num {
            "one" => vec![1],
            "two" => vec![2],
            "three" => vec![3],
            "four" => vec![4],
            "five" => vec![5],
            "six" => vec![6],
            "seven" => vec![7],
            "eight" => vec![8],
            "nine" => vec![9],
            "oneight" => vec![1, 8],
            "twone" => vec![2, 1],
            "threeight" => vec![3, 8],
            "fiveight" => vec![5, 8],
            "eightwo" => vec![8, 2],
            "eightree" => vec![8, 3],
            "nineight" => vec![9, 8],
            _ => panic!(),
        }
    }

    input
        .iter()
        .map(|num_str_vec| {
            let nums: Vec<usize> = num_str_vec
                .iter()
                .filter_map(|num_str| match num_str.as_str().parse() {
                    Ok(num_value) => Some(vec![num_value]),
                    Err(..) => {
                        if part_b {
                            Some(parse_spelled_num(num_str))
                        } else {
                            None
                        }
                    }
                })
                .flatten()
                .collect();
            nums[0] * 10 + nums[nums.len() - 1]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchetre
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 142);

        let sample = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        let input = parse_input(sample.to_string());
        assert_eq!(solve_part_b(&input), 281);
    }
}
