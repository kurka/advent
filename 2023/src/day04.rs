use std::collections::HashSet;
use std::fs;

use regex::Regex;

#[derive(Clone, Debug)]
struct ScratchCard {
    winning_nums: HashSet<usize>,
    card_nums: HashSet<usize>,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/04.txt").unwrap());
    println!("Day 04:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<ScratchCard> {
    let re = Regex::new(r"\d+").unwrap();
    input
        .lines()
        .map(|line| {
            let (_, nums) = line.split_once(":").unwrap();
            let (win_ns, card_ns) = nums.split_once(" |").unwrap();

            ScratchCard {
                winning_nums: HashSet::from_iter(
                    re.find_iter(win_ns).map(|n| n.as_str().parse().unwrap()),
                ),
                card_nums: HashSet::from_iter(
                    re.find_iter(card_ns).map(|n| n.as_str().parse().unwrap()),
                ),
            }
        })
        .collect()
}

fn solve_part_a(input: &Vec<ScratchCard>) -> usize {
    input
        .iter()
        .map(|card| {
            let card_hits = card.card_nums.intersection(&card.winning_nums).count();
            if card_hits > 0 {
                2_usize.pow(card_hits as u32 - 1)
            } else {
                0
            }
        })
        .sum()
}

fn solve_part_b(input: &Vec<ScratchCard>) -> usize {
    let mut card_counts = vec![1; input.len()];
    for (card_id, card) in input.into_iter().enumerate() {
        let card_points = card.card_nums.intersection(&card.winning_nums).count();
        for next_card in (card_id + 1)..(card_id + 1 + card_points) {
            card_counts[next_card] += card_counts[card_id]
        }
    }
    card_counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 13);
        assert_eq!(solve_part_b(&input), 30);
    }
}
