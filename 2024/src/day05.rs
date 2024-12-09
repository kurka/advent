use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Clone, Debug)]
struct DayOutput {
    page_orderings: HashMap<usize, HashSet<usize>>,
    page_productions: Vec<Vec<usize>>,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/05.txt").unwrap());
    println!("Day 05:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> DayOutput {
    let (page_orderings_str, page_productions_str) = input.split_once("\n\n").unwrap();

    let mut page_orderings = HashMap::new();
    page_orderings_str.lines().for_each(|line| {
        let (first, second) = line.split_once("|").unwrap();
        let first = first.parse().unwrap();
        let second = second.parse().unwrap();
        page_orderings
            .entry(first)
            .and_modify(|adjs: &mut HashSet<usize>| {
                adjs.insert(second);
            })
            .or_insert(HashSet::from([second]));
    });

    let page_productions: Vec<Vec<usize>> = page_productions_str
        .lines()
        .map(|line| line.split(",").map(|el| el.parse().unwrap()).collect())
        .collect();

    DayOutput {
        page_orderings,
        page_productions,
    }
}

fn solve_part_a(input: &DayOutput) -> usize {
    solve_day05(input, true)
}

fn solve_part_b(input: &DayOutput) -> usize {
    solve_day05(input, false)
}

fn solve_day05(input: &DayOutput, consider_valids: bool) -> usize {
    let mut middles = 0;
    for prods in &input.page_productions {
        let mut visited: Vec<usize> = vec![];
        let mut fixed: Vec<usize> = vec![];
        let mut valid = true;
        for prod in prods {
            let count_misplaces = visited
                .iter()
                .filter(|vis| {
                    input.page_orderings.contains_key(prod)
                        && input.page_orderings[prod].contains(*vis)
                })
                .count();
            fixed.insert(fixed.len() - count_misplaces, *prod);
            visited.push(*prod);
            if count_misplaces > 0 {
                valid = false;
            }
        }

        middles += if valid == consider_valids {
            fixed[prods.len() / 2]
        } else {
            0
        };
    }

    middles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 143);
        assert_eq!(solve_part_b(&input), 123);
    }
}
