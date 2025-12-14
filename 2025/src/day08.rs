use std::{cmp::Reverse, collections::BinaryHeap, fs};

#[derive(Clone, Debug)]
struct DayInput {
    boxes: Vec<(i64, i64, i64)>,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/08.txt").unwrap());
    println!("Day 08:");
    println!("{}", solve_part_a(&input, 1000));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> DayInput {
    DayInput {
        boxes: input
            .lines()
            .map(|line| {
                let nums: Vec<i64> = line.split(',').map(|n| n.parse().unwrap()).collect();
                (nums[0], nums[1], nums[2])
            })
            .collect(),
    }
}

fn solve_part_a(input: &DayInput, top_k: usize) -> usize {
    solve_day_08(input, Some(top_k))
}

fn solve_part_b(input: &DayInput) -> usize {
    solve_day_08(input, None)
}

fn solve_day_08(input: &DayInput, top_k: Option<usize>) -> usize {
    let n_boxes = input.boxes.len();
    let mut all_dists = vec![];
    // create a priority queue with all possible distances (o(n^2))
    for i in 0..n_boxes {
        for j in (i + 1)..n_boxes {
            let box_a = input.boxes[i];
            let box_b = input.boxes[j];
            let dist = (box_a.0 - box_b.0).pow(2)
                + (box_a.1 - box_b.1).pow(2)
                + (box_a.2 - box_b.2).pow(2);

            all_dists.push((Reverse(dist), i, j));
        }
    }
    let mut priority_queue = BinaryHeap::from(all_dists);

    // use union-find algorithm to determine sets and their sizes
    // o(k*log(n^2))
    let mut disjoint_sets: Vec<usize> = (0..n_boxes).collect();
    let mut sets_sizes: Vec<usize> = (0..n_boxes).map(|_| 1).collect();
    // let mut i = 0;
    for i in 1.. {
        // loop {
        let (Reverse(_min_dist), source, target) = priority_queue.pop().unwrap();
        let union_size = merge_set(&mut disjoint_sets, &mut sets_sizes, source, target);
        if union_size == n_boxes {
            return (input.boxes[source].0 * input.boxes[target].0) as usize;
        }

        if top_k.is_some_and(|top_k| top_k == i) {
            break;
        }
        // i += 1;
    }

    // compute size of 3 largest sets, for part_a
    let mut sets_heap = BinaryHeap::from(sets_sizes);
    let mut result = 1;
    for _ in 0..3 {
        result *= sets_heap.pop().unwrap();
    }
    result
}

fn find_set(disjoint_sets: &mut Vec<usize>, set_id: usize) -> usize {
    if disjoint_sets[set_id] != set_id {
        disjoint_sets[set_id] = find_set(disjoint_sets, disjoint_sets[set_id]);
    }
    disjoint_sets[set_id]
}

fn merge_set(
    disjoint_sets: &mut Vec<usize>,
    sets_sizes: &mut Vec<usize>,
    source: usize,
    target: usize,
) -> usize {
    let mut source_set = find_set(disjoint_sets, source);
    let mut target_set = find_set(disjoint_sets, target);
    if source_set != target_set {
        if sets_sizes[source_set] < sets_sizes[target_set] {
            (source_set, target_set) = (target_set, source_set);
        }

        let sets_sizes_sum = sets_sizes[source_set] + sets_sizes[target_set];
        sets_sizes[source_set] = sets_sizes_sum;
        sets_sizes[target_set] = 0;
        disjoint_sets[target_set] = source_set;
        sets_sizes_sum
    } else {
        sets_sizes[source_set]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input, 10), 40);
        assert_eq!(solve_part_b(&input), 25272);
    }
}
