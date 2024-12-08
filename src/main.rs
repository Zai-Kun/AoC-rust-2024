mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;

use std::time::Instant;

fn main() {
    let start = Instant::now();

    day_1::part_1::run().unwrap();
    day_1::part_2::run().unwrap();

    day_2::part_1::run().unwrap();
    day_2::part_2::run().unwrap();

    day_3::part_1::run().unwrap();
    day_3::part_2::run().unwrap();

    day_4::part_1::run().unwrap();
    day_4::part_2::run().unwrap();

    day_5::part_1_n_2::run().unwrap();

    day_6::part_1_n_2::run().unwrap();

    day_7::part_1::run().unwrap();
    day_7::part_2::run().unwrap();

    day_8::part_1::run().unwrap();
    day_8::part_2::run().unwrap();

    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
