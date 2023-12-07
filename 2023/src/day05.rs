use std::cmp::Ordering;
use std::cmp::{max, min};
use std::fs;

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

fn parse_input(input: String) -> (Vec<i64>, Vec<Range>) {
    let (seeds_and_title, rest) = input.split_once("\n\n").unwrap();
    let seeds: Vec<i64> = seeds_and_title
        .trim_start_matches("seeds: ")
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect();

    let maps: Vec<Vec<Range>> = rest
        .split("\n\n")
        .map(|map_block| {
            let mut maps: Vec<Range> = map_block
                .lines()
                .skip(1)
                .map(|map_str| {
                    let map_parts: Vec<i64> =
                        map_str.split(" ").map(|s| s.parse().unwrap()).collect();
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

    // create a single map that directly maps seed to location
    let summed_maps: Vec<Range> = maps
        .into_iter()
        .reduce(|ranges_a, ranges_b| {
            // sum ranges_a with ranges_b
            let mut summed_ranges: Vec<Range> = vec![];
            // every range in b can be reached by either original input in a, or a transformed one
            for rb in &ranges_b {
                // case 1: regions in rb without ranges in ra
                let mut intersections: Vec<&Range> = ranges_a
                    .iter()
                    .filter(|ra| !(rb.start > ra.end || rb.end < ra.start))
                    .collect();
                intersections.sort_by_key(|r| r.start);

                let mut new_range_start = rb.start;
                for ra in intersections {
                    if ra.start > new_range_start {
                        summed_ranges.push(Range {
                            start: new_range_start,
                            end: ra.start - 1,
                            offset: rb.offset,
                        });
                    }
                    new_range_start = ra.end + 1;
                }
                if new_range_start <= rb.end {
                    summed_ranges.push(Range {
                        start: new_range_start,
                        end: rb.end,
                        offset: rb.offset,
                    })
                }

                // case 2: regions in rb in regions that were transformed by ra (ra+rb)
                let mut transformed_intersections: Vec<Range> = ranges_a
                    .iter()
                    .map(|ra| Range {
                        start: ra.start + ra.offset,
                        end: ra.end + ra.offset,
                        offset: ra.offset,
                    })
                    .filter(|tra| !(rb.start > tra.end || rb.end < tra.start))
                    .collect();
                transformed_intersections.sort_by_key(|r| r.start);

                for tra in transformed_intersections {
                    summed_ranges.push(Range {
                        start: max(tra.start, rb.start) - tra.offset,
                        end: min(tra.end, rb.end) - tra.offset,
                        offset: tra.offset + rb.offset,
                    })
                }
            }
            // case 3: regions in ra that do not intersect with b
            for ra in &ranges_a {
                let mut intersections: Vec<Range> = ranges_b
                    .iter()
                    .filter(|rb| !(ra.start + ra.offset > rb.end || ra.end + ra.offset < rb.start))
                    .map(|r| r.clone())
                    .collect();
                intersections.sort_by_key(|r| r.start);

                // println!("{intersections:?}");
                let mut new_range_start = ra.start;
                for rb in &intersections {
                    if rb.start - ra.offset > new_range_start {
                        summed_ranges.push(Range {
                            start: new_range_start,
                            end: rb.start - ra.offset - 1,
                            offset: ra.offset,
                        });
                    }
                    new_range_start = rb.end - ra.offset + 1;
                }
                if new_range_start <= ra.end {
                    summed_ranges.push(Range {
                        start: new_range_start,
                        end: ra.end,
                        offset: ra.offset,
                    })
                }
            }
            summed_ranges.sort_by_key(|r| r.start);
            summed_ranges
        })
        .unwrap();

    (seeds, summed_maps)
}

fn solve_part_a(seeds: &Vec<i64>, maps: &Vec<Range>) -> i64 {
    seeds
        .iter()
        .map(|seed| {
            let search_existing_range =
                maps.binary_search_by(|r| match (*seed >= r.start, *seed <= r.end) {
                    (false, true) => Ordering::Greater,
                    (true, false) => Ordering::Less,
                    (true, true) => Ordering::Equal,
                    (false, false) => unreachable!(),
                });
            let offset = if let Ok(range_pos) = search_existing_range {
                maps[range_pos].offset
            } else {
                // default offset: 0
                0
            };
            seed + offset
        })
        .min()
        .unwrap()
}

fn solve_part_b(seeds: &Vec<i64>, maps: &Vec<Range>) -> i64 {
    seeds
        .chunks(2)
        .flat_map(|seed_info| {
            let seed_start = seed_info[0];
            let seed_range_size = seed_info[1];

            let mut locations: Vec<i64> = vec![];
            let mut cur_seed = seed_start;
            let mut remaining_range = seed_range_size;

            while remaining_range > 0 {
                // find the range to which the cur_seed fits
                let existing_range =
                    maps.binary_search_by(|r| match (cur_seed >= r.start, cur_seed <= r.end) {
                        (false, true) => Ordering::Greater,
                        (true, false) => Ordering::Less,
                        (true, true) => Ordering::Equal,
                        (false, false) => unreachable!(),
                    });
                if existing_range.is_ok() {
                    let range_pos = existing_range.unwrap();
                    let cur_range = &maps[range_pos];
                    locations.push(cur_seed + cur_range.offset);
                    let shift = cur_range.end - cur_seed + 1;
                    cur_seed += shift;
                    remaining_range -= shift;
                } else {
                    // there isn't a range where seed fits
                    let range_pos = existing_range.unwrap_err();
                    // push seed as it is
                    locations.push(cur_seed);
                    if range_pos != maps.len() {
                        // move to the next range
                        let shift = maps[range_pos].start - cur_seed;
                        cur_seed += shift;
                        remaining_range -= shift;
                    } else {
                        let shift = remaining_range;
                        cur_seed += shift;
                        remaining_range -= shift;
                    }
                }
            }
            locations
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

        // for r in &maps {
        //     println!("{r:?}")
        // }

        assert_eq!(solve_part_a(&seeds, &maps), 35);
        assert_eq!(solve_part_b(&seeds, &maps), 46);
    }
}
