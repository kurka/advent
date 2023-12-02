use regex::Regex;
use std::fs;

#[derive(Clone, Debug)]
struct Game {
    reds: usize,
    greens: usize,
    blues: usize,
}

#[derive(Clone, Debug)]
struct GameRecord {
    id: usize,
    games: Vec<Game>,
}

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/02.txt").unwrap());
    println!("Day 02:");
    println!("{}", solve_part_a(&input));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> Vec<GameRecord> {
    let re_number = Regex::new(r"\d+").unwrap();
    let re_red = Regex::new(r"(\d+) red").unwrap();
    let re_green = Regex::new(r"(\d+) green").unwrap();
    let re_blue = Regex::new(r"(\d+) blue").unwrap();

    fn read_color(re: &Regex, str: &str) -> usize {
        match re.captures(str) {
            Some(m) => m[1].parse().unwrap(),
            None => 0,
        }
    }

    input
        .lines()
        .map(|line| {
            let (title, body) = line.split_once(":").unwrap();
            let games = body
                .split(";")
                .map(|game| Game {
                    reds: read_color(&re_red, game),
                    greens: read_color(&re_green, game),
                    blues: read_color(&re_blue, game),
                })
                .collect();
            GameRecord {
                id: re_number.find(title).unwrap().as_str().parse().unwrap(),
                games: games,
            }
        })
        .collect()
}

fn solve_part_a(input: &Vec<GameRecord>) -> usize {
    let max_game = Game {
        reds: 12,
        greens: 13,
        blues: 14,
    };
    input
        .iter()
        .filter_map(|record| {
            let is_possible = record.games.iter().all(|g| {
                g.reds <= max_game.reds && g.greens <= max_game.greens && g.blues <= max_game.blues
            });
            if is_possible {
                Some(record.id)
            } else {
                None
            }
        })
        .sum()
}

fn solve_part_b(input: &Vec<GameRecord>) -> usize {
    input
        .iter()
        .map(|record| {
            let mut min_game = Game {
                reds: 0,
                greens: 0,
                blues: 0,
            };
            for game in &record.games {
                // If I used numerical indexes instead of names, I could have a loop here and DRY. Too late now!
                if game.reds > min_game.reds {
                    min_game.reds = game.reds;
                }
                if game.greens > min_game.greens {
                    min_game.greens = game.greens;
                }
                if game.blues > min_game.blues {
                    min_game.blues = game.blues;
                }
            }
            min_game.reds * min_game.greens * min_game.blues
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input), 8);
        assert_eq!(solve_part_b(&input), 2286);
    }
}
