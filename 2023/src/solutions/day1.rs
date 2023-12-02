use std::{io::{BufReader, self, BufRead}, fs::File, collections::{HashMap, HashSet}};

fn curr_digit (line: &str, prefixes: &HashSet<&str>, digits: &HashMap<&str, i32>) -> Option<i32> {
    let mut index: usize = 1;
    while index <= line.len() {
        if !prefixes.contains(&line[0..index]) {
            return None;
        } else if digits.contains_key(&line[0..index]) {
            return digits.get(&line[0..index])
                .copied();
        }
        index += 1;
    }

    None
}

fn do_line (line: &str, prefixes: &HashSet<&str>, digits: &HashMap<&str, i32>) -> i32 {
    let mut it = (0..line.len()).filter_map(|i| curr_digit(&line[i..], prefixes, digits));

    let first = it.next().unwrap_or(0);
    let last = it.last().unwrap_or(first);

    first * 10 + last
}

fn find_sum (lines: &Vec<String>, digits: &HashMap<&str, i32>) -> i32 {
    let prefixes: HashSet<&str> = digits.keys()
        .map(|s| (0..s.len()).map(|i| &s[0..i + 1]))
        .flatten()
        .collect();

    lines.iter()
        .map(|l| do_line(l, &prefixes, digits))
        .sum()
}

fn part1 (lines: &Vec<String>) -> i32 {
    let digits = HashMap::from([
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9)
    ]);

    find_sum(lines, &digits)
}

fn part2 (lines: &Vec<String>) -> i32 {
    let digits = HashMap::from([
        ("0", 0),
        ("1", 1),
        ("one", 1),
        ("2", 2),
        ("two", 2),
        ("3", 3),
        ("three", 3),
        ("4", 4),
        ("four", 4),
        ("5", 5),
        ("five", 5),
        ("6", 6),
        ("six", 6),
        ("7", 7),
        ("seven", 7),
        ("8", 8),
        ("eight", 8),
        ("9", 9),
        ("nine", 9)
    ]);

    find_sum(lines, &digits)
}

pub fn solve (file: BufReader<File>) -> io::Result<()> {
    let lines = file.lines()
        .collect::<io::Result<Vec<String>>>()?;

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
    Ok(())
}