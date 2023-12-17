use regex::Regex;
use std::fs;

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_length: u8,
}

#[derive(Clone, Debug)]
struct Bucket {
    lenses: Vec<Lens>,
}

impl Bucket {
    fn new() -> Bucket {
        Bucket { lenses: vec![] }
    }

    fn remove(&mut self, label: &String) {
        if let Some(pos) = self.lenses.iter().position(|l| l.label == *label) {
            self.lenses.remove(pos);
        }
    }

    fn insert(&mut self, label: &String, focal_length: u8) {
        if let Some(existing_pos) = self.lenses.iter().position(|l| l.label == *label) {
            self.lenses[existing_pos].focal_length = focal_length
        } else {
            let new_lens = Lens {
                label: label.clone(),
                focal_length,
            };
            self.lenses.push(new_lens);
        }
    }
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/15.txt").unwrap());
    println!("Day 15:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> String {
    input
}

fn solve_part_a(input: &String) -> usize {
    input
        .trim()
        .split(',')
        .map(|inst| hash(inst.as_bytes()))
        .sum()
}

fn solve_part_b(input: &String) -> usize {
    let re = Regex::new(r"(\w+)([=-])(\d?)").unwrap();
    let mut all_boxes: Vec<Bucket> = vec![Bucket::new(); 256];
    input.trim().split(',').for_each(|inst| {
        let (_, [bucket, op, lens]) = re.captures(inst).unwrap().extract();
        let target_box = hash(bucket.as_bytes());
        let bucket = bucket.to_string();
        match op {
            "=" => all_boxes[target_box].insert(&bucket, lens.parse().unwrap()),
            "-" => all_boxes[target_box].remove(&bucket),
            _ => unreachable!(),
        }
    });
    all_boxes
        .iter()
        .enumerate()
        .flat_map(|(i, b)| {
            b.lenses
                .iter()
                .enumerate()
                .map(move |(r, l)| (i + 1) * (r + 1) * l.focal_length as usize)
        })
        .sum()
}

fn hash(input: &[u8]) -> usize {
    input
        .iter()
        .fold(0, |acc, &c| ((acc + (c as usize)) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 1320);
        assert_eq!(solve_part_b(&input), 145);
    }
}
