use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/07.txt").unwrap());
    println!("Day 07:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (res, eq) = line.split_once(": ").unwrap();
            let res = res.parse().unwrap();
            let eq = eq.split(" ").map(|num| num.parse().unwrap()).collect();
            (res, eq)
        })
        .collect()
}

fn solve_part_a(input: &Vec<(usize, Vec<usize>)>) -> usize {
    solve_day7(input, false)
}

fn solve_part_b(input: &Vec<(usize, Vec<usize>)>) -> usize {
    solve_day7(input, true)
}

fn solve_day7(input: &Vec<(usize, Vec<usize>)>, part_b: bool) -> usize {
    input
        .into_iter()
        .map(|(eq, res)| {
            // println!("{:?}", get_permutations(res, res.len() - 1, part_b));
            if get_permutations(eq, res, res.len() - 1, part_b)
                .iter()
                .any(|perm| {
                    // println!("{perm} {eq} {}", perm == eq);
                    perm == eq
                })
            {
                *eq
            } else {
                0
            }
        })
        .sum()
}

fn get_permutations(eq: &usize, res: &Vec<usize>, pos: usize, part_b: bool) -> Vec<usize> {
    if pos == 0 {
        return vec![res[pos]];
    }

    let head = res[pos];
    let tails = get_permutations(eq, res, pos - 1, part_b);

    if !part_b {
        tails
            .into_iter()
            .flat_map(|tail| [head + tail, head * tail])
            .filter(|x| x <= eq)
            .collect()
    } else {
        tails
            .into_iter()
            .flat_map(|tail| {
                [
                    head + tail,
                    head * tail,
                    10_usize.pow(head.ilog10() + 1) * tail + head,
                ]
            })
            .filter(|x| x <= eq)
            // 10.0_f64.powf((t.log10() + 1.0).floor()) * st + t,
            .collect()
    }

    // return tails.into_iter().map(|tail| head + tail).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 3749);
        assert_eq!(solve_part_b(&input), 11387);
    }
}
