#![allow(non_snake_case)]
mod solutions;

use std::env;

use std::fs::File;
use std::io::BufReader;

use solutions::{day1, day2, day3, day4, day5, day6};

#[allow(non_snake_case)]

fn executeDay (day: i32, file: File) {
    match day {
        1 => day1::solve(BufReader::new(file)),
        2 => day2::solve(BufReader::new(file)),
        3 => day3::solve(BufReader::new(file)),
        4 => day4::solve(BufReader::new(file)),
        5 => day5::solve(BufReader::new(file)),
        6 => day6::solve(BufReader::new(file)),
        _ => panic!("Unknown day: {}!", day),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: i32 = args.get(1)
        .expect("Usage: cargo run <day num>")
        .parse()
        .expect("Day must be an integer!");

    let file = File::open(format!("input/day{}.txt", day))
        .expect(format!("File input/day{}.txt does not exist!", day).as_str());
    
    println!("Day {}", day);
    executeDay(day, file)
}
