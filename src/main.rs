use std::fs;

fn main() {
    println!("hello world!");
    solve1();
}


fn solve1() {
    let input = parse1(fs::read_to_string("src/input01.in").unwrap());
    println!("Day 1:");
    println!("{}", day1a(&input));
    println!("{}", day1b(&input))
}

fn parse1(input: String) -> Vec<i32> {
    input
        .lines()
        .map(|l| l.parse().unwrap_or_default())
        .collect()
}

fn day1a(calories_list: &Vec<i32>) -> i32 {
    calories_list
        .split(|c| *c == 0)
        .map(|s| (*s).iter().sum())
        .max()
        .unwrap()
}

fn day1b(calories_list: &Vec<i32>) -> i32 {
    let mut calories: Vec<i32> = calories_list
         .split(|c| *c == 0)
         .map(|s| (*s).iter().sum())
         .collect();
    calories.sort();
    calories.reverse();
    calories[..3].iter().sum()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1() {
        let sample = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let input = parse1(sample.to_string());

        assert_eq!(24000, day1a(&input));
        assert_eq!(45000, day1b(&input));
    }
}
