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

fn solve_part_b(input: &Vec<PosAndVel>) -> i64 {
    for gap in 2..1000000 {
        if gap % 10000 == 0 {
            println!("{gap}")
        }
        for i in 0..input.len() {
            for j in 0..input.len() {
                if j == i {
                    continue;
                }
                let pv1 = input[i];
                let pv2 = input[j];

                let pos_a = (
                    pv1.px as i64 + 1 * pv1.vx as i64,
                    pv1.py as i64 + 1 * pv1.vy as i64,
                    pv1.pz as i64 + 1 * pv1.vz as i64,
                );
                let pos_b = (
                    pv2.px as i64 + gap * pv2.vx as i64,
                    pv2.py as i64 + gap * pv2.vy as i64,
                    pv2.pz as i64 + gap * pv2.vz as i64,
                );
                if (pos_b.0 - pos_a.0) % (gap - 1) != 0
                    || (pos_b.1 - pos_a.1) % (gap - 1) != 0
                    || (pos_b.2 - pos_a.2) % (gap - 1) != 0
                {
                    continue;
                }
                let deltas = (
                    (pos_b.0 - pos_a.0) / (gap - 1),
                    (pos_b.1 - pos_a.1) / (gap - 1),
                    (pos_b.2 - pos_a.2) / (gap - 1),
                );
                let p0 = (pos_a.0 - deltas.0, pos_a.1 - deltas.1, pos_a.2 - deltas.2);
                let mut all_valid = true;
                // if gap == 3 && i == 4 && j == 1 {
                //     println!("{pos_a:?}  {pos_b:?}  {p0:?}  {deltas:?}")
                // }
                for k in 0..input.len() {
                    if k == i || k == j {
                        continue;
                    }
                    let pv3 = input[k];
                    // println!("{gap} {i} {j} {k} {pv3:?}");
                    // check for obvius inconsistencies
                    // if (pv3.px as i64 < pos_b.0 && pv3.vx as i64 < 0)
                    //     || (pv3.px as i64 > pos_b.0 && pv3.vx as i64 > 0)
                    //     || (pv3.py as i64 < pos_b.1 && pv3.vy as i64 < 0)
                    //     || (pv3.py as i64 > pos_b.1 && pv3.vy as i64 > 0)
                    //     || (pv3.pz as i64 < pos_b2 && pv3.vz  as i64< 0)
                    //     || (pv3.pz as i64 > pos_b2 && pv3.vz  as i64> 0)
                    // if (((pv3.px as i64) < pos_b.0) == ((pv3.vx as i64) < 0))
                    //     || (((pv3.py as i64) < pos_b.1) == ((pv3.vy as i64) < 0))
                    //     || (((pv3.pz as i64) < pos_b.2) == ((pv3.vz as i64) < 0))
                    // {
                    all_valid = false;
                    if (deltas.0 > 0 && (pv3.px as i64) < pos_b.0 && (pv3.px as i64) < 0)
                        || (deltas.0 < 0 && (pv3.px as i64) > pos_b.0 && (pv3.vx as i64) > 0)
                        || (deltas.1 > 0 && (pv3.py as i64) < pos_b.1 && (pv3.vy as i64) < 0)
                        || (deltas.1 < 0 && (pv3.py as i64) > pos_b.1 && (pv3.vy as i64) > 0)
                        || (deltas.2 > 0 && (pv3.pz as i64) < pos_b.2 && (pv3.vz as i64) < 0)
                        || (deltas.2 < 0 && (pv3.pz as i64) > pos_b.2 && (pv3.vz as i64) > 0)
                    // || deltas.0 == (pv3.vx as i64)
                    // || deltas.1 == (pv3.vy as i64)
                    // || deltas.2 == (pv3.vz as i64)
                    {
                        // println!("first excep");
                        break;
                    }

                    // x0 + dx0 * t = x3 + dx3 * t
                    // dx0*t - dx3*t = x3 - x0
                    // t (dx0 - dx3) = x3 - x0
                    // t = (x3 - x0) / (dx0 - dx3)
                    if
                    //deltas.0 == (pv3.vx as i64)
                    // || deltas.1 == (pv3.vy as i64)
                    // || deltas.2 == (pv3.vz as i64)
                    //||
                    ((pv3.px as i64) - p0.0) % (deltas.0 - (pv3.vx as i64)) != 0 {
                        // println!("deltas excep");
                        break;
                    }
                    let t = ((pv3.px as i64) - p0.0) / (deltas.0 - (pv3.vx as i64));

                    if t == 0 {
                        println!("{pv3:?}; {p0:?};  {deltas:?}");
                    }

                    // println!("{gap} {i} {j} {k} {t}");
                    if t < 0 {
                        break;
                    }

                    if (((pv3.px as i64) + t * (pv3.vx as i64)) != p0.0 + t * deltas.0)
                        || (((pv3.py as i64) + t * (pv3.vy as i64)) != p0.1 + t * deltas.1)
                        || (((pv3.pz as i64) + t * (pv3.vz as i64)) != p0.2 + t * deltas.2)
                    {
                        println!("escape");
                        break;
                    }

                    all_valid = true;
                }
                if all_valid {
                    return p0.0 + p0.1 + p0.2;
                }
            }
        }
    }
    unreachable!()
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
        assert_eq!(solve_part_b(&input), 47);
    }
}
