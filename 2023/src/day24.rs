use std::fs;

#[derive(Clone, Debug, Copy)]
struct PosAndVel {
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/24.txt").unwrap());
    println!("Day 24:");
    println!(
        "{}",
        solve_part_a(&input, 200000000000000.0, 400000000000000.0)
    );
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<PosAndVel> {
    input
        .lines()
        .map(|line| {
            let (pos, vel) = line.split_once(" @ ").unwrap();
            let pos: Vec<&str> = pos.split(", ").collect();
            let vel: Vec<&str> = vel.split(", ").collect();
            PosAndVel {
                px: pos[0].trim().parse().unwrap(),
                py: pos[1].trim().parse().unwrap(),
                pz: pos[2].trim().parse().unwrap(),
                vx: vel[0].trim().parse().unwrap(),
                vy: vel[1].trim().parse().unwrap(),
                vz: vel[2].trim().parse().unwrap(),
            }
        })
        .collect()
}

fn solve_part_a(input: &Vec<PosAndVel>, box_min: f64, box_max: f64) -> usize {
    let mut inter = 0;
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            let pv1 = input[i];
            let pv2 = input[j];

            let x1 = pv1.px;
            let x2 = pv1.px + 100.0 * pv1.vx;
            let x3 = pv2.px;
            let x4 = pv2.px + 100.0 * pv2.vx;
            let y1 = pv1.py;
            let y2 = pv1.py + 100.0 * pv1.vy;
            let y3 = pv2.py;
            let y4 = pv2.py + 100.0 * pv2.vy;

            let px = ((((x1 * y2) - (y1 * x2)) * (x3 - x4))
                - ((x1 - x2) * ((x3 * y4) - (y3 * x4))))
                / (((x1 - x2) * (y3 - y4)) - ((y1 - y2) * (x3 - x4)));
            let py = ((((x1 * y2) - (y1 * x2)) * (y3 - y4))
                - ((y1 - y2) * ((x3 * y4) - (y3 * x4))))
                / (((x1 - x2) * (y3 - y4)) - ((y1 - y2) * (x3 - x4)));

            if px.is_finite()
                && px >= box_min
                && px <= box_max
                && ((pv1.vx >= 0.0 && px >= x1) || (pv1.vx < 0.0 && px < x1))
                && ((pv2.vx >= 0.0 && px >= x3) || (pv2.vx < 0.0 && px < x3))
                && py.is_finite()
                && py >= box_min
                && py <= box_max
                && ((pv1.vy >= 0.0 && py >= y1) || (pv1.vy < 0.0 && py < y1))
                && ((pv2.vy >= 0.0 && py >= y3) || (pv2.vy < 0.0 && py < y3))
            {
                inter += 1;
            }
        }
    }

    inter
}

fn solve_part_b(_input: &Vec<PosAndVel>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input, 7.0, 27.0), 2);
        // assert_eq!(solve_part_b(&input), 1337);
    }
}
