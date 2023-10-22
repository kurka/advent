use std::time::Instant;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day11_alt;
pub mod day12;
pub mod day13;
pub mod day14;

fn main() {
    let days = [
        day01::solve,
        day02::solve,
        day03::solve,
        day04::solve,
        day05::solve,
        day06::solve,
        day07::solve,
        day08::solve,
        day09::solve,
        day10::solve,
        day11::solve,
        // day11_alt::solve,
        day12::solve,
        day13::solve,
        day14::solve,
    ];

    let time = Instant::now();
    let durations = days.map(|day| {
        let start = time.elapsed();
        day();
        let end = time.elapsed();
        end - start
    });
    println!("\n\nTime analysis:");
    let total_duration: u128 = durations.iter().map(|d| d.as_micros()).sum();

    println!("Total: {:.3}ms", total_duration as f32 / 1000.0);
    for (day, duration) in durations.iter().enumerate() {
        println!(
            "Day {:02}:\t{:>8.3}ms\t{:>5.2}%",
            day + 1,
            (duration.as_micros() as f32) / 1000.0,
            (duration.as_micros() as f32 / total_duration as f32) * 100.0
        )
    }
}
