use std::cmp::max;
use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/05.txt").unwrap());
    println!("Day 05:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> (Vec<(usize, usize)>, Vec<usize>) {
    let (ranges_str, ingredients_str) = input.split_once("\n\n").unwrap();

    let ranges = ranges_str
        .lines()
        .map(|line| {
            let (start_str, end_str) = line.split_once("-").unwrap();
            (start_str.parse().unwrap(), end_str.parse().unwrap())
        })
        .collect();

    let ingredients = ingredients_str
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    (ranges, ingredients)
}

fn create_disjoint_ranges(ranges: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    // merge overlapping ranges so we end up with disjoint intervals
    let mut disjoint_ranges = vec![];

    let mut cur_range = ranges[0];
    for range in &ranges[1..] {
        if range.0 > cur_range.1 {
            disjoint_ranges.push(cur_range);
            cur_range = range.clone();
        } else {
            cur_range.1 = max(cur_range.1, range.1);
        }
    }
    disjoint_ranges.push(cur_range);

    disjoint_ranges
}

fn bin_search(
    ranges: &Vec<(usize, usize)>,
    target: usize,
    left: usize,
    right: usize,
) -> Option<usize> {
    if left >= right {
        return None;
    }

    let mid = (left + right) / 2;
    if ranges[mid].0 > target {
        bin_search(ranges, target, left, mid)
    } else if ranges[mid].1 < target {
        bin_search(ranges, target, mid + 1, right)
    } else {
        Some(mid)
    }
}

fn solve_part_a(input: &(Vec<(usize, usize)>, Vec<usize>)) -> usize {
    let (mut ranges, ingredients) = input.clone();
    ranges.sort_by_key(|(start, _)| *start);
    let disjoint_ranges = create_disjoint_ranges(&ranges);

    let mut res = 0;
    for ingredient in ingredients {
        let idx = bin_search(&disjoint_ranges, ingredient, 0, disjoint_ranges.len());
        if idx.is_some() {
            res += 1;
        }
    }
    res
}
fn solve_part_b(input: &(Vec<(usize, usize)>, Vec<usize>)) -> usize {
    let (mut ranges, _) = input.clone();
    ranges.sort_by_key(|(start, _)| *start);
    let disjoint_ranges = create_disjoint_ranges(&ranges);

    let mut res = 0;
    for range in disjoint_ranges {
        res += range.1 - range.0 + 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 3);
        assert_eq!(solve_part_b(&input), 14);
    }
}
