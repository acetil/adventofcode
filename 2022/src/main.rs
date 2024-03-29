#![allow(non_snake_case)]
mod solutions;

use std::env;

use std::fs::File;
use std::io::BufReader;

use solutions::{day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13, day14, day15};

#[allow(non_snake_case)]

fn executeDay (day: i32, file: File) {
    match day {
         1 =>  day1::solve(BufReader::new(file)),
         2 =>  day2::solve(BufReader::new(file)),
         3 =>  day3::solve(BufReader::new(file)),
         4 =>  day4::solve(BufReader::new(file)),
         5 =>  day5::solve(BufReader::new(file)),
         6 =>  day6::solve(BufReader::new(file)),
         7 =>  day7::solve(BufReader::new(file)),
         8 =>  day8::solve(BufReader::new(file)),
         9 =>  day9::solve(BufReader::new(file)),
        10 => day10::solve(BufReader::new(file)),
        11 => day11::solve(BufReader::new(file)),
        12 => day12::solve(BufReader::new(file)),
        13 => day13::solve(BufReader::new(file)),
        14 => day14::solve(BufReader::new(file)),
        15 => day15::solve(BufReader::new(file)),
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
