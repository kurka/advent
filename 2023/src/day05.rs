use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;

#[derive(Clone, Debug)]
struct ScratchCard {
    winning_nums: HashSet<usize>,
    card_nums: HashSet<usize>,
}

#[derive(Clone, Debug)]
struct Range {
    start: i64,
    end: i64,
    offset: i64,
}

pub fn solve() {
    let (seeds, maps) = parse_input(fs::read_to_string("inputs/05.txt").unwrap());
    println!("Day 05:");
    println!("{}", solve_part_a(&seeds, &maps));
    println!("{}", solve_part_b(&seeds, &maps));
}

fn parse_input(input: String) -> (Vec<i64>, Vec<Vec<Range>>) {
    let (seeds_and_title, rest) = input.split_once("\n\n").unwrap();
    let seeds: Vec<i64> = seeds_and_title
        .trim_start_matches("seeds: ")
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect();

    // let maps: Vec<HashMap<Range, i64>> = rest.splut("\n\n").map(|map_block| {
    let maps: Vec<Vec<Range>> = rest
        .split("\n\n")
        .map(|map_block| {
            let mut maps: Vec<Range> = map_block
                .lines()
                .skip(1)
                .map(|map_str| {
                    let map_parts: Vec<i64> =
                        map_str.split(" ").map(|s| s.parse().unwrap()).collect();
                    // (dest_start, source_start, range_len)
                    Range {
                        start: map_parts[1],
                        end: map_parts[1] + map_parts[2] - 1,
                        offset: map_parts[0] - map_parts[1],
                    }
                })
                .collect();
            // sort it so we can use binary search later
            maps.sort_by_key(|r| r.start);
            maps
        })
        .collect();
    (seeds, maps)
}

fn solve_part_a(seeds: &Vec<i64>, maps: &Vec<Vec<Range>>) -> i64 {
    seeds
        .iter()
        .map(|seed| {
            maps.iter().fold(*seed, |cur_seed, m| {
                // find the range to which the cur_seed fits
                let search_existing_range =
                    m.binary_search_by(|r| match (cur_seed >= r.start, cur_seed <= r.end) {
                        (false, true) => Ordering::Greater,
                        (true, false) => Ordering::Less,
                        (true, true) => Ordering::Equal,
                        (false, false) => unreachable!(),
                    });
                let offset = if let Ok(range_pos) = search_existing_range {
                    m[range_pos].offset
                } else {
                    // default offset: 0
                    0
                };
                cur_seed + offset
            })
        })
        .min()
        .unwrap()
}

fn solve_part_b(seeds: &Vec<i64>, maps: &Vec<Vec<Range>>) -> i64 {
    seeds
        .chunks(2)
        .flat_map(|seed_info| (seed_info[0]..(seed_info[0] + seed_info[1])))
        .map(|seed| {
            maps.iter().fold(seed, |cur_seed, m| {
                // find the range to which the cur_seed fits
                let search_existing_range =
                    m.binary_search_by(|r| match (cur_seed >= r.start, cur_seed <= r.end) {
                        (false, true) => Ordering::Greater,
                        (true, false) => Ordering::Less,
                        (true, true) => Ordering::Equal,
                        (false, false) => unreachable!(),
                    });
                let offset = if let Ok(range_pos) = search_existing_range {
                    m[range_pos].offset
                } else {
                    // default offset: 0
                    0
                };
                cur_seed + offset
            })
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4

";

        let (seeds, maps) = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&seeds, &maps), 35);
        assert_eq!(solve_part_b(&seeds, &maps), 46);
    }
}
