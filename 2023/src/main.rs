mod solutions;

use std::{env, io::{self, BufReader, Read}, fs::File};

use solutions::{day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13, day15, day16};

fn solve (num: i32, path: Option<String>) -> io::Result<()> {
    let func = match num {
        1 => day1::solve,
        2 => day2::solve,
        3 => day3::solve,
        4 => day4::solve,
        5 => day5::solve,
        6 => day6::solve,
        7 => day7::solve,
        8 => day8::solve,
        9 => day9::solve,
        10 => day10::solve,
        11 => day11::solve,
        12 => day12::solve,
        13 => day13::solve,
        15 => day15::solve,
        16 => day16::solve,
        _ => {
            println!("Day {num} is not yet implemented!");
            return Ok(())
        }
    };

    let file_path = path.unwrap_or(format!("input/day{}.txt", num));

    let file = File::open(&file_path)
        .expect(&format!("Failed to open file at \"{}\"!", file_path));
    let mut file_str: String = String::new();
    BufReader::new(file).read_to_string(&mut file_str)?;

    func(&file_str);

    Ok(())
}

fn main() -> io::Result<()>{
    let args: Vec<String> = env::args().collect();

    args.get(1)
        .and_then(|s| s.parse::<i32>().ok())
        .map(|i| solve(i, args.get(2).map(String::clone)))
        .unwrap_or_else(|| {
            println!("Usage: cargo run <problem_num> [path]");
            Ok(())
        })
}
