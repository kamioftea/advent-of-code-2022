mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;
mod util;

use std::io::{self, Write};
use std::time::Instant;

extern crate core;

extern crate itertools;
#[macro_use]
extern crate text_io;

fn main() {
    print!("Which day? (0 to run all): ");
    io::stdout().flush().unwrap();

    let day: i32 = read!();
    let days: Vec<Box<dyn Fn() -> ()>> = vec![
        Box::new(|| day_1::run()),
        Box::new(|| day_2::run()),
        Box::new(|| day_3::run()),
        Box::new(|| day_4::run()),
        Box::new(|| day_5::run()),
        Box::new(|| day_6::run()),
        Box::new(|| day_7::run()),
        Box::new(|| day_8::run()),
        Box::new(|| day_9::run()),
        Box::new(|| day_10::run()),
        Box::new(|| day_11::run()),
        Box::new(|| day_12::run()),
        Box::new(|| day_13::run()),
        Box::new(|| day_14::run()),
        Box::new(|| day_15::run()),
        Box::new(|| day_16::run()),
        Box::new(|| day_17::run()),
        Box::new(|| day_18::run()),
        Box::new(|| day_19::run()),
        Box::new(|| day_20::run()),
        Box::new(|| day_21::run()),
        Box::new(|| day_22::run()),
        Box::new(|| day_23::run()),
        Box::new(|| day_24::run()),
        Box::new(|| day_25::run()),
    ];

    let start = Instant::now();
    match days.get((day - 1) as usize) {
        Some(solution) => solution(),
        None if day == 0 => days.iter().enumerate().for_each(|(i, solution)| {
            let start = Instant::now();
            println!("==== Day {} ====", i + 1);
            solution();
            println!("-- took {:.2?}", start.elapsed());
        }),
        None => println!("Invalid Day {}", day),
    }

    println!();
    println!("Finished in {:.2?}", start.elapsed());
}
