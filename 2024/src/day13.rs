use regex::Regex;
use std::fs;

#[derive(Clone, Debug)]
struct ClawGame {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize),
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/13.txt").unwrap());
    println!("Day 13:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<ClawGame> {
    let re_button = Regex::new(r"Button [A|B]: X\+(\d+), Y\+(\d+)").unwrap();
    let re_prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    input
        .split("\n\n")
        .map(|game| {
            let mut game_parts = game.lines();
            let (_, [but_a_x, but_a_y]) = re_button
                .captures(game_parts.next().unwrap())
                .unwrap()
                .extract();
            let (_, [but_b_x, but_b_y]) = re_button
                .captures(game_parts.next().unwrap())
                .unwrap()
                .extract();
            let (_, [prize_x, prize_y]) = re_prize
                .captures(game_parts.next().unwrap())
                .unwrap()
                .extract();
            ClawGame {
                button_a: (but_a_x.parse().unwrap(), but_a_y.parse().unwrap()),
                button_b: (but_b_x.parse().unwrap(), but_b_y.parse().unwrap()),
                prize: (prize_x.parse().unwrap(), prize_y.parse().unwrap()),
            }
        })
        .collect()
}

fn solve_part_a(input: &Vec<ClawGame>) -> usize {
    solve_day13(input, false)
}

fn solve_part_b(input: &Vec<ClawGame>) -> usize {
    solve_day13(input, true)
}

fn solve_day13(input: &Vec<ClawGame>, part_b: bool) -> usize {
    input
        .iter()
        .map(|game| {
            let prize_x = game.prize.0 as f64 + if part_b { 10000000000000.0 } else { 0.0 };
            let prize_y = game.prize.1 as f64 + if part_b { 10000000000000.0 } else { 0.0 };

            // solving the equations system for a and b, we find a trivial answer:
            // a*ax + b*bx = px
            // a*ay + b*by = py
            //
            // b = (px - a*ax)/bx
            // a*ay + ((px - a*ax)/bx)by = py
            // a*ay - a*ax*by/bx + px*by/bx = py
            // a*(ay - ax*by/bx) = py - px*by/bx
            // a = (py - px*by/bx)/(ay - ax*by/bx)
            let a = (prize_y - ((prize_x * game.button_b.1 as f64) / game.button_b.0 as f64))
                / (game.button_a.1 as f64
                    - ((game.button_a.0 * game.button_b.1) as f64 / game.button_b.0 as f64));
            let b = (prize_x - a * game.button_a.0 as f64) / game.button_b.0 as f64;
            let a = a.round() as usize;
            let b = b.round() as usize;
            if game.button_a.0 * a + game.button_b.0 * b == (prize_x as usize)
                && game.button_a.1 * a + game.button_b.1 * b == (prize_y as usize)
            {
                3 * a + b
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 480);
    }
}
