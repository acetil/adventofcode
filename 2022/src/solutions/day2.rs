use std::{io::{BufReader, BufRead}, fs::File};
use itertools::Itertools;

fn getRPSResult (choice: i32, oppChoice: i32) -> i32 {
    if choice == oppChoice {
        3
    } else if (choice - 1 + 3) % 3 == oppChoice % 3 {
        6
    } else {
        0
    }
}

fn getScore (choice: i32, oppChoice: i32) -> i32 {
    return getRPSResult(choice, oppChoice) + choice;
}

fn getExpectedScore (oppChoice: i32, result: i32) -> i32 {
    let choice = (oppChoice - 1 + (result - 2) + 3) % 3 + 1;

    getScore(choice, oppChoice)
}   

fn getValue (choice: &str) -> i32 {
    match choice {
        "A" => 1,
        "X" => 1,
        "B" => 2,
        "Y" => 2,
        "C" => 3,
        "Z" => 3,
        s => panic!("Invalid choice: {}", s)
    }
}

pub fn solve (file: BufReader<File>) {
    let lines: Vec<(i32, i32)> = file.lines()
        .filter_map(Result::ok)
        .map(|s| s.as_str().trim().to_string())
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().split(" ").map(getValue).take(2).collect_tuple())
        .filter_map(|x| x)
        .collect();

    let totalPart1: i32 = lines.iter()
        .map(|tup| getScore(tup.1, tup.0))
        .sum();
    
    println!("Problem 1 total: {}", totalPart1);

    let totalPart2: i32 = lines.iter()
        .map(|tup| getExpectedScore(tup.0, tup.1))
        .sum();
    
    println!("Problem 2 total: {}", totalPart2);
}