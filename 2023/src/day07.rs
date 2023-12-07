use std::collections::HashMap;
use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/07.txt").unwrap());
    println!("Day 07:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<(Vec<char>, usize)> {
    input
        .lines()
        .map(|l| {
            let (hand_s, bid_s) = l.split_once(" ").unwrap();
            (hand_s.chars().collect(), bid_s.parse().unwrap())
        })
        .collect()
}

fn solve_part_a(input: &Vec<(Vec<char>, usize)>) -> usize {
    solve_day7(input, false)
}

fn solve_part_b(input: &Vec<(Vec<char>, usize)>) -> usize {
    solve_day7(input, true)
}

fn solve_day7(input: &Vec<(Vec<char>, usize)>, part_b: bool) -> usize {
    let mut hands: Vec<(usize, usize, usize, usize, usize, usize, usize)> = input
        .iter()
        .map(|(hand, bid)| encode_hand(hand, *bid, part_b))
        .collect();
    hands.sort();
    hands.iter().enumerate().map(|(i, h)| (i + 1) * h.6).sum()
}

fn encode_hand(
    hand: &Vec<char>,
    bid: usize,
    part_b: bool,
) -> (usize, usize, usize, usize, usize, usize, usize) {
    // classify hand type by counting repeated cards
    let mut counter: HashMap<char, usize> = HashMap::new();
    for card in hand {
        counter.entry(*card).and_modify(|c| *c += 1).or_insert(1);
    }
    let unique_cards: usize;
    let max_repetitions: usize;
    if !part_b {
        unique_cards = counter.len();
        max_repetitions = counter.into_values().max().unwrap();
    } else {
        let js = counter.remove(&'J').unwrap_or(0);
        unique_cards = counter.len();
        max_repetitions = counter.values().max().unwrap_or(&0) + js;
    }
    let hand_points = match (unique_cards, max_repetitions) {
        (_, 5) => 7, // 5 of a kind
        (2, 4) => 6, // 4 of a kind
        (2, 3) => 5, // full house
        (3, 3) => 4, // 3 of a kind
        (3, 2) => 3, // 2 pair
        (4, 2) => 2, // 1 pair
        (5, 1) => 1, // high card
        _ => unreachable!(),
    };

    // encode cards in base 13
    let cards_encoded: Vec<usize> = hand
        .iter()
        .map(|c| match c {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => {
                if !part_b {
                    11
                } else {
                    1
                }
            }
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => unreachable!(),
        })
        .collect();

    (
        hand_points,
        cards_encoded[0],
        cards_encoded[1],
        cards_encoded[2],
        cards_encoded[3],
        cards_encoded[4],
        bid,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 6440);
        assert_eq!(solve_part_b(&input), 5905);
    }
}
