mod solutions;

use std::{env, io::{self, BufReader}, fs::File};

use solutions::{day1, day2, day3, day4, day5};

fn solve (num: i32, path: Option<String>) -> io::Result<()> {
    let func = match num {
        1 => day1::solve,
        2 => day2::solve,
        3 => day3::solve,
        4 => day4::solve,
        5 => day5::solve,
        _ => {
            println!("Day {num} is not yet implemented!");
            return Ok(())
        }
    };

    let file_path = path.unwrap_or(format!("input/day{}.txt", num));

    let file = File::open(&file_path)
        .expect(&format!("Failed to open file at \"{}\"!", file_path));

    func(BufReader::new(file))
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
