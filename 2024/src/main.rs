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
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
// pub mod day17;
// pub mod day18;
// pub mod day19;
// pub mod day20;
// pub mod day21;
// pub mod day22;
// pub mod day23;
// pub mod day24;
// pub mod day25;

fn main() {
    let days = [
        day01::solve,
        day02::solve,
        day03::solve,
        day04::solve,
        day05::solve,
        // day06::solve,
        // day07::solve,
        // day08::solve,
        // day09::solve,
        day10::solve,
        day11::solve,
        day12::solve,
        day13::solve,
        day14::solve,
        day15::solve,
        day16::solve,
        // day17::solve,
        // day18::solve,
        // day19::solve,
        // day20::solve,
        // day21::solve,
        // day22::solve,
        // day23::solve,
        // day24::solve,
        // day25::solve,
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
