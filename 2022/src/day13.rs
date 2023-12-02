use std::{cmp::Ordering, fs, slice::Iter};

#[derive(PartialEq)]
enum Ternary {
    True,
    False,
    Unknown,
}

#[derive(Debug, PartialEq)]
enum IntOrVec {
    Int(i32),
    Pkg(Vec<IntOrVec>),
}

type Packet = Vec<IntOrVec>;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/input13.in").unwrap());
    println!("Day 13:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<(Packet, Packet)> {
    input
        .lines()
        .collect::<Vec<&str>>() // arg! couldn't find a better way to use chunks below! :(
        .chunks(3)
        .map(|chunk| {
            (
                parse_packet(&(chunk[0].chars().collect()), 0).0,
                parse_packet(&(chunk[1].chars().collect()), 0).0,
            )
        })
        .collect()
}

fn parse_packet(input: &Vec<char>, start: usize) -> (Packet, usize) {
    let mut v = Vec::new();

    let mut idx = start;
    let mut cur_num = Vec::new();
    while idx < input.len() - 1 {
        idx += 1;
        match input[idx] {
            ',' | ']' => {
                if cur_num.len() > 0 {
                    let num: String = cur_num.iter().collect();
                    v.push(IntOrVec::Int(num.parse().unwrap()));
                    cur_num = Vec::new();
                }
                if input[idx] == ']' {
                    break;
                }
            }
            '[' => {
                let (sub_packet, new_idx) = parse_packet(input, idx);
                idx = new_idx;
                v.push(IntOrVec::Pkg(sub_packet))
            }
            num_char => cur_num.push(num_char),
        }
    }
    (v, idx)
}

fn right_order(mut packet_a: Iter<IntOrVec>, mut packet_b: Iter<IntOrVec>) -> Ternary {
    let head_a = packet_a.next();
    let head_b = packet_b.next();

    match (head_a, head_b) {
        (Some(IntOrVec::Int(a)), Some(IntOrVec::Int(b))) => {
            if a < b {
                Ternary::True
            } else if a > b {
                Ternary::False
            } else {
                right_order(packet_a, packet_b)
            }
        }
        (Some(IntOrVec::Pkg(a)), Some(IntOrVec::Pkg(b))) => {
            let sub_result = right_order(a.iter(), b.iter());
            if sub_result == Ternary::Unknown {
                right_order(packet_a, packet_b)
            } else {
                sub_result
            }
        }
        (Some(IntOrVec::Int(a)), Some(IntOrVec::Pkg(b))) => {
            let sub_result = right_order([IntOrVec::Int(*a)].iter(), b.iter());
            if sub_result == Ternary::Unknown {
                right_order(packet_a, packet_b)
            } else {
                sub_result
            }
        }
        (Some(IntOrVec::Pkg(a)), Some(IntOrVec::Int(b))) => {
            let sub_result = right_order(a.iter(), [IntOrVec::Int(*b)].iter());
            if sub_result == Ternary::Unknown {
                right_order(packet_a, packet_b)
            } else {
                sub_result
            }
        }
        (None, Some(_)) => Ternary::True,
        (Some(_), None) => Ternary::False,
        (None, None) => Ternary::Unknown,
    }
}

fn solve_part_a(packets: &Vec<(Packet, Packet)>) -> usize {
    packets
        .iter()
        .enumerate()
        .filter(|(_, (pa, pb))| right_order(pa.iter(), pb.iter()) != Ternary::False)
        .map(|(idx, _)| idx + 1)
        .sum()
}

fn right_order_cmp(pa: &&Packet, pb: &&Packet) -> Ordering {
    match right_order(pa.iter(), pb.iter()) {
        Ternary::False => Ordering::Greater,
        Ternary::Unknown => Ordering::Equal,
        Ternary::True => Ordering::Less,
    }
}

fn solve_part_b(packets: &Vec<(Packet, Packet)>) -> usize {
    let d1 = &vec![IntOrVec::Pkg(vec![IntOrVec::Int(2)])];
    let d2 = &vec![IntOrVec::Pkg(vec![IntOrVec::Int(6)])];

    let mut ordered: Vec<&Packet> = packets
        .iter()
        .map(|ab| [&ab.0, &ab.1])
        .flatten()
        .collect::<Vec<&Packet>>();

    ordered.append(&mut vec![&d1, &d2]);
    ordered.sort_by(right_order_cmp);
    let a = ordered
        .binary_search_by(|x| right_order_cmp(x, &d1))
        .unwrap();
    let b = ordered
        .binary_search_by(|x| right_order_cmp(x, &d2))
        .unwrap();
    (a + 1) * (b + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        let input = parse_input(sample.to_string());

        assert_eq!(13, solve_part_a(&input));
        assert_eq!(140, solve_part_b(&input));
    }
}
