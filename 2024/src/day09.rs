use std::fs;

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/09.txt").unwrap());
    println!("Day 09:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<char> {
    input.trim().chars().collect()
}

fn solve_part_a(input: &Vec<char>) -> i64 {
    let mut disk: Vec<i64> = vec![];

    for (i, c) in input.into_iter().enumerate() {
        for _ in 0..c.to_digit(10).unwrap() {
            disk.push(if i % 2 == 0 { i as i64 / 2 } else { -1 })
        }
    }
    // println!("{disk:?}");

    let mut left = 0;
    let mut right = disk.len() - 1;

    while left < right {
        if disk[left] != -1 {
            left += 1;
            continue;
        } else {
            disk[left] = disk[right];
            left += 1;
            right -= 1;
            while disk[right] == -1 {
                right -= 1
            }
        }
    }
    // println!("{disk:?}, {left} {right}");

    (0..=left).map(|i| (i as i64) * disk[i]).sum()
}

fn solve_part_b(input: &Vec<char>) -> u64 {
    let mut disk: Vec<(u32, i32)> = vec![];

    for (i, c) in input.into_iter().enumerate() {
        disk.push((
            c.to_digit(10).unwrap(),
            if i % 2 == 0 { i as i32 / 2 } else { -1 },
        ));
    }
    // println!("{disk:?}");

    let mut right = disk.len() - 1;
    while right > 0 {
        if disk[right].1 == -1 {
            right -= 1;
            continue;
        }
        let (right_count, right_value) = disk[right];
        for left in 0..right {
            let (candidate_count, candidate_value) = disk[left];
            if candidate_value == -1 && candidate_count >= right_count {
                // do the swap
                // insert right into left
                disk.insert(left, (right_count, right_value));
                // update empty space
                if candidate_count - right_count > 0 {
                    disk[left + 1].0 -= right_count;
                    right += 1;
                } else {
                    disk.remove(left + 1);
                }
                // remove right from disk
                disk[right].1 = -1;
                if right > 0 && disk[right - 1].1 == -1 {
                    // join right with its left
                    disk[right - 1].0 = disk[right - 1].0 + disk[right].0;
                    disk.remove(right);
                    right -= 1;
                }
                if right < disk.len() - 1 && disk[right + 1].1 == -1 {
                    // join right with its right
                    disk[right].0 = disk[right].0 + disk[right + 1].0;
                    disk.remove(right + 1);
                }
                break;
            }
        }

        right -= 1;
    }

    // println!("{disk:?}, {right}");
    let mut checksum = 0_u64;
    let mut id = 0_u64;
    for (count, val) in disk {
        for _ in 0..count {
            if val != -1 {
                checksum += id * val as u64;
            }
            id += 1;
        }
    }
    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "2333133121414131402";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 1928);
        assert_eq!(solve_part_b(&input), 2858);
    }
}
