use std::{collections::VecDeque, fs};

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/input10.in").unwrap());
    println!("Day 10:");
    println!("{}", solve_part_a(&input));
    print_grid(&solve_part_b(&input))
}

fn parse_input(input: String) -> Vec<Instruction> {
    input
        .lines()
        // .map(|line| line.bytes().map(|b| (b - b'0') as i32).collect())
        .map(|line| {
            if line.starts_with("addx") {
                let (_, number) = line.split_at(5);
                Instruction::Addx(number.parse().unwrap())
            } else {
                Instruction::Noop
            }
        })
        .collect()
}

fn solve_part_a(instructions: &Vec<Instruction>) -> i32 {
    let mut marks: VecDeque<_> = [20, 60, 100, 140, 180, 220].into();
    let mut register = 1;
    let mut cycle = 1;
    let mut result = 0;

    for instruction in instructions {
        if marks.len() == 0 {
            break;
        }
        match instruction {
            Instruction::Addx(x) => {
                register += x;
                cycle += 2
            }
            Instruction::Noop => cycle += 1,
        }

        if cycle == marks[0] || cycle + 1 == marks[0] {
            let position = marks.pop_front().unwrap();
            result += position * register;
        }
    }
    result
}

fn solve_part_b(instructions: &Vec<Instruction>) -> Vec<Vec<char>> {
    let (crt_v, crt_h) = (6, 40);
    let mut crt: Vec<Vec<char>> = vec![vec!['.'; 40]; 6];
    let mut sprite_pos: i32 = 1;
    let mut cycle: (i32, i32) = (0, 0);
    for instruction in instructions {
        let (steps, sprite_increase) = match instruction {
            Instruction::Addx(x) => (2, *x),
            Instruction::Noop => (1, 0),
        };
        for _ in 0..steps {
            if (cycle.1 - sprite_pos).abs() <= 1 {
                crt[cycle.0 as usize][cycle.1 as usize] = '#';
            } else {
                crt[cycle.0 as usize][cycle.1 as usize] = '.'
            }

            cycle.0 = if cycle.1 < (crt_h - 1) {
                cycle.0
            } else {
                (cycle.0 + 1) % crt_v
            };
            cycle.1 = (cycle.1 + 1) % crt_h;
        }
        sprite_pos += sprite_increase
    }
    crt
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        let r = String::from_iter(row);
        println!("{r}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

        let input = parse_input(sample.to_string());

        assert_eq!(13140, solve_part_a(&input));

        let part2_ans: Vec<Vec<char>> = vec![
            "##..##..##..##..##..##..##..##..##..##..".chars().collect(),
            "###...###...###...###...###...###...###.".chars().collect(),
            "####....####....####....####....####....".chars().collect(),
            "#####.....#####.....#####.....#####.....".chars().collect(),
            "######......######......######......####".chars().collect(),
            "#######.......#######.......#######.....".chars().collect(),
        ];
        assert_eq!(part2_ans, solve_part_b(&input));
    }
}
