use std::{io::{BufReader, BufRead}, fs::File, collections::HashMap};

use itertools::Itertools;
use regex::Regex;

fn getSetup (setupLines: &Vec<String>) -> Option<HashMap<i32, Vec<String>>> {
    let mut map: HashMap<i32, Vec<String>> = HashMap::new();
    let mut stacks: Vec<i32> = Vec::new();
    
    for i in setupLines.last()?.split(" ").filter(|s| !s.trim().is_empty()) {
        let id: i32 = i.parse().ok()?;
        map.insert(id, Vec::new());
        stacks.push(id);
    }

    for i in setupLines.iter().take(setupLines.len() - 1).rev() {
        let mut index: usize = 0;

        for j in i.chars().skip(1).step_by(4) {
            if !j.is_whitespace() {
                map.get_mut(&stacks[index])?.push(j.to_string());
            }

            index += 1;
        }
    }

    Some(map)
}

fn performInstructionPart1 (stackMap: &mut HashMap<i32, Vec<String>>, amount: i32, from: i32, to: i32) {
    let mut toAdd: Vec<String> = Vec::new();
    for _ in 0..amount {
        toAdd.push(stackMap.get_mut(&from).unwrap().pop().unwrap());
    }

    stackMap.get_mut(&to).unwrap().append(&mut toAdd);
}

fn performInstructionPart2 (stackMap: &mut HashMap<i32, Vec<String>>, amount: i32, from: i32, to: i32) {
    let mut toAdd: Vec<String>;

    {
        let fromVec = stackMap.get_mut(&from).unwrap();
        toAdd = fromVec.split_off(fromVec.len() - amount as usize);
    }

    stackMap.get_mut(&to).unwrap().append(&mut toAdd);
}

pub fn solve (file: BufReader<File>) {
    let lines: Vec<String> = file.lines()
        .filter_map(Result::ok)
        .collect();

    let setupLines: Vec<String> = lines.iter()
        .take_while(|s| !s.is_empty())
        .map(|s| s.clone())
        .collect();

    let stackMap = getSetup(&setupLines).expect("Failed to parse setup!");

    let instPattern = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();


    let mut part1Map = stackMap.clone();
    let mut part2Map = stackMap;

    for i in lines.iter().skip(setupLines.len() + 1) {
        let capture = instPattern.captures(i).expect(format!("Failed to match instruction from \"{i}\"").as_str());

        let amount: i32 = capture.get(1).unwrap().as_str().parse().unwrap();
        let from: i32 = capture.get(2).unwrap().as_str().parse().unwrap();
        let to: i32 = capture.get(3).unwrap().as_str().parse().unwrap();

        performInstructionPart1(&mut part1Map, amount, from, to);
        performInstructionPart2(&mut part2Map, amount, from, to);
    }

    println!("Part 1 result: \"{}\"", part1Map.iter()
        .sorted_by(|a, b| Ord::cmp(a.0, b.0))
        .map(|(_, v)| v)
        .map(|v| v.last().unwrap())
        .fold(String::new(), |mut a, b| {
            a.push_str(b);
            a
        }));

    println!("Part 2 result: \"{}\"", part2Map.iter()
        .sorted_by(|a, b| Ord::cmp(a.0, b.0))
        .map(|(_, v)| v).map(|v| v.last().unwrap())
        .fold(String::new(), |mut a, b| {
            a.push_str(b);
            a
        }));
}