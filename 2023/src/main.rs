use std::time::Instant;

pub mod day01;
pub mod day02;

fn main() {
    let days = [day01::solve, day02::solve];

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
