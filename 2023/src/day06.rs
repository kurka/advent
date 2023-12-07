use regex::Regex;
use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/06.txt").unwrap());
    println!("Day 06:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<(f64, f64)> {
    fn get_nums(line: &str) -> Vec<f64> {
        let re = Regex::new(r"\d+").unwrap();
        re.find_iter(line)
            .map(|d| d.as_str().parse().unwrap())
            .collect()
    }
    let mut lines = input.lines();
    let times = get_nums(lines.next().unwrap());
    let distances = get_nums(lines.next().unwrap());
    times.into_iter().zip(distances.into_iter()).collect()
}

fn solve_part_a(input: &Vec<(f64, f64)>) -> i64 {
    input
        .iter()
        .map(|(t, d)| compute_time_options(*t, *d))
        .product()
}

fn solve_part_b(input: &Vec<(f64, f64)>) -> i64 {
    let (super_t, super_d) = input
        .clone()
        .into_iter()
        .reduce(|(st, sd), (t, d)| {
            // concatenate numbers. This is a bit expensive, it might be easier to just re-read the input
            (
                10.0_f64.powf((t.log10() + 1.0).floor()) * st + t,
                10.0_f64.powf((d.log10() + 1.0).floor()) * sd + d,
            )
        })
        .unwrap();
    compute_time_options(super_t, super_d)
}

fn compute_time_options(t: f64, d: f64) -> i64 {
    // second degree equation:
    // (t-x)*x = d
    // tx - x^2 = d
    // x^2 - tx + d = 0
    // Bhaskara: x = -b +- sqrt(b^2 - 4ac) / 2a
    // x = (t +- sqrt(t^2 - 4d)) / 2
    let delta = (t.powi(2) - 4.0 * d).sqrt();
    let x1 = (((t - delta) / 2.0) + 1.0).floor() as i64;
    let x2 = (((t + delta) / 2.0) - 1.0).ceil() as i64;
    x2 - x1 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
Time:      7  15   30
Distance:  9  40  200
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 288);
        assert_eq!(solve_part_b(&input), 71503);
    }
}
