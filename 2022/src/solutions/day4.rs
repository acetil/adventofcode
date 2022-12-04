use std::{io::{BufReader, BufRead}, fs::File};

use itertools::Itertools;

fn rangesContain (range1: (i32, i32), range2: (i32, i32)) -> bool {
    return (range1.0 <= range2.0 && range1.1 >= range2.1) || (range2.0 <= range1.0 && range2.1 >= range1.1);
}

fn rangesOverlap (range1: (i32, i32), range2: (i32, i32)) -> bool {
    return (range1.0 <= range2.0 && range1.1 >= range2.0) || (range1.0 >= range2.0 && range1.0 <= range2.1);
}

fn parseLine (line: &str) -> Option<((i32, i32), (i32, i32))> {
    let ranges: Vec<&str> = line.split(",").collect();

    let range1: (i32, i32) = ranges.get(0)?
        .split("-")
        .map(|s| s.parse::<i32>())
        .filter_map(Result::ok)
        .collect_tuple()?;

    let range2: (i32, i32) = ranges.get(1)?
        .split("-")
        .map(|s| s.parse::<i32>())
        .filter_map(Result::ok)
        .collect_tuple()?;

    Some((range1, range2))
}

pub fn solve (file: BufReader<File>) {
    let pairs: Vec<((i32, i32), (i32, i32))> = file.lines()
        .filter_map(Result::ok)
        .map(|s| s.trim().to_string())
        .filter_map(|s| parseLine(s.as_str()))
        .collect();

    let containedPairs = pairs.iter()
        .filter(|(range1, range2)| rangesContain(*range1, *range2))
        .count();

    println!("Contained pairs: {}", containedPairs);

    let overlappingPairs = pairs.iter()
        .filter(|(range1, range2)| rangesOverlap(*range1, *range2))
        .count();

    println!("Overlapping pairs: {}", overlappingPairs);
}