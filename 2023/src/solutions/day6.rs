use std::{io::{BufReader, self, BufRead}, fs::File};

fn calc_answer (time: i64, dist: i64) -> i64 {
    // n(t - n) = d
    // nt - n^2 = d
    // n^2 - nt + d = 0
    let disc = ((time * time - 4 * (dist + 1)) as f64).sqrt();

    ((time as f64 + disc) / 2.0).floor() as i64 - ((time as f64 - disc) / 2.0).ceil() as i64 + 1
}

fn get_power_10 (n: i64) -> i64 {
    let mut i: i64 = 1;
    while n >= i {
        i *= 10;
    }

    i
}

pub fn solve (mut file: BufReader<File>) -> io::Result<()> {
    let mut line: String = String::new();
    file.read_line(&mut line)?;

    let times: Vec<i64> = line.split(":")
        .last()
        .unwrap()
        .trim()
        .split(" ")
        .filter(|s| !s.trim().is_empty())
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    line.clear();
    file.read_line(&mut line)?;
    let distances: Vec<i64> = line.split(":")
        .last()
        .unwrap()
        .trim()
        .split(" ")
        .filter(|s| !s.trim().is_empty())
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let part1 = times.iter()
        .zip(distances.iter())
        .map(|(&t, &d)| calc_answer(t, d))
        .reduce(|a, b| a * b)
        .unwrap();
    println!("Part 1: {part1}");

    let time = times.iter()
        .fold(0, |a, &b| (a * get_power_10(b) + b));
    let distance = distances.iter()
        .fold(0, |a, &b| (a * get_power_10(b) + b));

    println!("Part 2: {}", calc_answer(time, distance));

    Ok(())
}