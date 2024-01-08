use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::{cmp::Ordering, fs};

#[derive(Clone, Debug)]
enum Rule {
    Rule {
        part: char,
        op: Ordering,
        amount: usize,
        dest: Box<Rule>,
    },
    NextRule(String),
}

#[derive(Clone, Debug)]
enum Node {
    Rule {
        amount: usize,
        op: Ordering,
        part: char,
    },
    A,
    R,
}

#[derive(Clone, Debug)]
struct BinaryTree {
    value: Node,
    left: Option<Box<BinaryTree>>,
    right: Option<Box<BinaryTree>>,
}

type Workflow = HashMap<String, Vec<Rule>>;
type Rating = HashMap<char, usize>;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/19.txt").unwrap());
    println!("Day 19:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> (Workflow, Vec<Rating>) {
    let (workflows_s, ratings_s) = input.split_once("\n\n").unwrap();

    let re_workflow = Regex::new(r"(\w+)\{(.*)\}").unwrap();
    let re_rule = Regex::new(r"(?<part>\w+)(?<op>[<>])(?<amount>\d+):(?<dest>\w+)").unwrap();
    let workflows: Workflow = workflows_s
        .lines()
        .map(|line| {
            let (_, [wname, rules_s]) = re_workflow.captures(line).unwrap().extract();
            let rules: Vec<Rule> = rules_s
                .split(',')
                .map(|mayberule| match re_rule.captures(mayberule) {
                    Some(caps) => Rule::Rule {
                        part: caps["part"].chars().next().unwrap(),
                        op: match &caps["op"] {
                            ">" => Ordering::Greater,
                            "<" => Ordering::Less,
                            _ => unreachable!(),
                        },
                        amount: caps["amount"].parse().unwrap(),
                        dest: Box::new(Rule::NextRule(caps["dest"].to_string())),
                    },
                    None => Rule::NextRule(mayberule.to_string().clone()),
                })
                .collect();
            (wname.to_string(), rules)
        })
        .collect();

    let re_rating = Regex::new(r"\{x=(?<x>\d+),m=(?<m>\d+),a=(?<a>\d+),s=(?<s>\d+)\}").unwrap();
    let ratings: Vec<Rating> = ratings_s
        .lines()
        .map(|line| {
            let (_, [x, m, a, s]) = re_rating.captures(line).unwrap().extract();
            HashMap::from([
                ('x', x.parse().unwrap()),
                ('m', m.parse().unwrap()),
                ('a', a.parse().unwrap()),
                ('s', s.parse().unwrap()),
            ])
        })
        .collect();

    (workflows, ratings)
}

fn solve_part_a(input: &(Workflow, Vec<Rating>)) -> usize {
    let (workflow, ratings) = input;
    ratings
        .iter()
        .map(|rating| {
            let mut cur_ruleset = workflow["in"].iter();

            loop {
                // consume ruleset until we find a decision or a next rule
                let mut rule: &Rule = cur_ruleset.next().unwrap();
                loop {
                    match rule {
                        Rule::NextRule(ruleset_name) => match ruleset_name.as_str() {
                            "A" => {
                                return rating[&'x'] + rating[&'m'] + rating[&'a'] + rating[&'s']
                            }
                            "R" => return 0,
                            x => {
                                cur_ruleset = workflow[x].iter();
                                break;
                            }
                        },
                        Rule::Rule {
                            part,
                            op,
                            amount,
                            dest,
                        } => {
                            if rating[part].cmp(amount) == *op {
                                rule = &(**dest);
                            } else {
                                rule = cur_ruleset.next().unwrap();
                            }
                        }
                    }
                }
            }
        })
        .sum()
}

fn solve_part_b(input: &(Workflow, Vec<Rating>)) -> usize {
    let (workflow, _) = input;

    // unroll workflow to transform it into binary tree
    let root = ruleset2tree(&workflow["in"], workflow);

    // find for each leave of the tree what combination of ranges it allows, and
    // return the possible number of input combinations
    let valid_ranges: HashMap<char, (usize, usize)> = HashMap::from([
        ('x', (1, 4000)),
        ('m', (1, 4000)),
        ('a', (1, 4000)),
        ('s', (1, 4000)),
    ]);
    find_possible_paths(Box::new(root), valid_ranges)
}

fn ruleset2tree(ruleset: &Vec<Rule>, workflow: &Workflow) -> BinaryTree {
    let (head, tail_iter) = (*ruleset).split_first().unwrap();
    let tail: Vec<Rule> = tail_iter.iter().map(|x| x.clone()).collect();
    match head {
        Rule::NextRule(ruleset_name) => rule2tree(ruleset_name, workflow),
        Rule::Rule {
            part,
            op,
            amount,
            dest,
        } => BinaryTree {
            value: Node::Rule {
                amount: *amount,
                op: *op,
                part: *part,
            },
            left: match &(**dest) {
                Rule::NextRule(dest_string) => Some(Box::new(rule2tree(dest_string, workflow))),
                _ => unreachable!(),
            },
            right: Some(Box::new(ruleset2tree(&tail, workflow))),
        },
    }
}

fn rule2tree(rule: &String, workflow: &Workflow) -> BinaryTree {
    match rule.as_str() {
        "A" => BinaryTree {
            value: Node::A,
            left: None,
            right: None,
        },
        "R" => BinaryTree {
            value: Node::R,
            left: None,
            right: None,
        },
        x => ruleset2tree(&workflow[x], workflow),
    }
}

fn find_possible_paths(
    root: Box<BinaryTree>,
    valid_ranges: HashMap<char, (usize, usize)>,
) -> usize {
    match root.value {
        Node::R => 0,
        Node::A => valid_ranges
            .values()
            .map(|(start, end)| (end - start + 1))
            .product(),
        Node::Rule { amount, op, part } => {
            let mut left_ranges = valid_ranges.clone();
            let mut right_ranges = valid_ranges.clone();
            match op {
                Ordering::Less => {
                    left_ranges
                        .entry(part)
                        .and_modify(|(_start, end)| *end = min(*end, amount - 1));
                    right_ranges
                        .entry(part)
                        .and_modify(|(start, _end)| *start = max(*start, amount));
                }
                Ordering::Greater => {
                    left_ranges
                        .entry(part)
                        .and_modify(|(start, _end)| *start = max(*start, amount + 1));
                    right_ranges
                        .entry(part)
                        .and_modify(|(_start, end)| *end = min(*end, amount));
                }
                _ => unreachable!(),
            }
            find_possible_paths(root.left.unwrap(), left_ranges)
                + find_possible_paths(root.right.unwrap(), right_ranges)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 19114);
        assert_eq!(solve_part_b(&input), 167409079868000);
    }
}
