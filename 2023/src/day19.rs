use regex::Regex;
use std::collections::HashMap;
use std::{cmp::Ordering, fs};

// #[derive(Clone, Debug)]
// struct Workflow {
//     rules: Vec<Rule>,
// }

// #[derive(Clone, Debug)]
// struct Rule {
//     part: String,
//     op: Ordering,
//     amount: usize,
//     dest: Rulead::NextRule(String),
// }

#[derive(Clone, Debug)]
enum Rulead {
    // Rule(Rule),
    Rule {
        part: String,
        op: Ordering,
        amount: usize,
        dest: Box<Rulead>,
    },
    // Decision(Decision),
    NextRule(String),
}

// #[derive(Clone, Debug)]
// enum Decision {
//     A,
//     R,
// }

type Workflow = HashMap<String, Vec<Rulead>>;
type Rating = HashMap<String, usize>;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/19.txt").unwrap());
    println!("Day 19:");
    println!("{}", solve_part_a(&input));
    // println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> (Workflow, Vec<Rating>) {
    let (workflows_s, ratings_s) = input.split_once("\n\n").unwrap();

    let re_workflow = Regex::new(r"(\w+)\{(.*)\}").unwrap();
    let re_rule = Regex::new(r"(?<part>\w+)(?<op>[<>])(?<amount>\d+):(?<dest>\w+)").unwrap();
    let workflows: Workflow = workflows_s
        .lines()
        .map(|line| {
            let (_, [wname, rules_s]) = re_workflow.captures(line).unwrap().extract();
            let rules: Vec<Rulead> = rules_s
                .split(',')
                .map(|mayberule| match re_rule.captures(mayberule) {
                    Some(caps) => Rulead::Rule {
                        part: caps["part"].to_string(),
                        op: match &caps["op"] {
                            ">" => Ordering::Greater,
                            "<" => Ordering::Less,
                            _ => unreachable!(),
                        },
                        amount: caps["amount"].parse().unwrap(),
                        dest: Box::new(Rulead::NextRule(caps["dest"].to_string())),
                    },
                    None => Rulead::NextRule(mayberule.to_string().clone()),
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
                ("x".to_string(), x.parse().unwrap()),
                ("m".to_string(), m.parse().unwrap()),
                ("a".to_string(), a.parse().unwrap()),
                ("s".to_string(), s.parse().unwrap()),
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

            // while let Rule::NextRule(ruleset) = cur_ruleset {
            loop {
                // consume ruleset until we find a decision or a next rule
                let mut rule: &Rulead = cur_ruleset.next().unwrap();
                loop {
                    match rule {
                        Rulead::NextRule(ruleset_name) => match ruleset_name.as_str() {
                            "A" => return rating["x"] + rating["m"] + rating["a"] + rating["s"],
                            "R" => return 0,
                            x => {
                                cur_ruleset = workflow[x].iter();
                                break;
                            }
                        },
                        Rulead::Rule {
                            part,
                            op,
                            amount,
                            dest,
                        } => {
                            if rating[part].cmp(amount) == *op {
                                // let new_rule = **dest; //Rulead::NextRule((*dest).clone()).clone();
                                // rule = &new_rule;
                                rule = &(**dest);
                            } else {
                                rule = cur_ruleset.next().unwrap();
                            }
                        }
                    }
                }
                // match cur_ruleset {
                //     Rule::NextRule(ruleset_name) => cur_ruleset = workflow[&ruleset_name],
                //     _ => todo!(),
                // }
            }
        })
        .sum()
}

fn solve_part_b(_input: &(Workflow, Vec<Rating>)) -> usize {
    todo!()
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
        // assert_eq!(solve_part_b(&input), 1337);
    }
}
