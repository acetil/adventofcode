use std::{io::{BufReader, BufRead}, fs::File, collections::{HashSet, HashMap}};

fn getDisjoint (line: &str) -> Option<char> {
    let chars: Vec<char> = line.chars().collect();

    let firstCompartment = chars.get(0..chars.len() / 2)?;
    let secondCompartment = chars.get(chars.len() / 2..)?;

    for c in firstCompartment {
        if secondCompartment.contains(c) {
            return Some(*c);
        }
    }

    None
}

fn getPriority (c: char) -> i32 {
    if 'a' <= c && c <= 'z' {
        ((c as u32) - ('a' as u32) + 1) as i32
    } else {
        ((c as u32) - ('A' as u32) + 27) as i32
    }
}

fn getBadge (group: &[String]) -> Option<char> {
    let sets: Vec<HashSet<char>> = group.iter().map(|s| s.chars().collect::<HashSet<char>>()).collect();

    let mut freqs: HashMap<char, i32> = HashMap::new();

    for i in sets {
        for j in i {
            freqs.entry(j).and_modify(|i| *i += 1).or_insert(1);
        }
    }

    return freqs.iter().filter(|(_, val)| **val == 3).nth(0).map(|(key, _)| key).copied();
}

pub fn solve (file: BufReader<File>) {
    let lines: Vec<String> = file.lines()
        .filter_map(Result::ok)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let disjointSum: i32 = lines.iter()
        .filter_map(|s| getDisjoint(&s))
        .map(getPriority)
        .sum();

    println!("Disjoint sum: {}", disjointSum);

    let badgeSum: i32 = lines.chunks_exact(3)
        .filter_map(getBadge)
        .map(getPriority)
        .sum();

    println!("Badge sum: {}", badgeSum);
}