use std::{io::{BufReader, BufRead}, fs::File, collections::HashSet};

use itertools::Itertools;

fn getStep (stepStr: &str) -> (i32, i32) {
    match stepStr.trim() {
        "L" => (-1, 0),
        "R" => (1, 0),
        "U" => (0, -1),
        "D" => (0, 1),
        _ => panic!("Invalid step: {stepStr}")
    }
}

fn advance (head: (i32, i32), step: (i32, i32)) -> (i32, i32) {
    (head.0 + step.0, head.1 + step.1)
}

fn follow (head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    if (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1 {
        return tail;
    }

    let mut newTail: (i32, i32) = tail;

    if head.0 > tail.0 {
        newTail.0 += 1
    }

    if head.0 < tail.0 {
        newTail.0 -= 1
    }

    if head.1 > tail.1 {
        newTail.1 += 1
    }

    if head.1 < tail.1 {
        newTail.1 -= 1
    }

    return newTail;
}

pub fn simulateStep (step: (i32, i32), knots: &mut Vec<(i32, i32)>) {
    knots[0] = advance(knots[0], step);

    for i in 1..knots.len() {
        knots[i] = follow(knots[i - 1], knots[i]);
    }
}

pub fn simulateKnots (instructions: &Vec<((i32, i32), i32)>, numKnots: usize) -> usize {
    let mut knots: Vec<(i32, i32)> = Vec::new();

    for _ in 0..numKnots {
        knots.push((0, 0));
    }

    let mut posSet: HashSet<(i32, i32)> = HashSet::new();

    for i in instructions {
        for _ in 0..i.1 {
            simulateStep(i.0, &mut knots);
            posSet.insert(*knots.last().unwrap());
        }
    }

    posSet.len()
}

pub fn solve (file: BufReader<File>) {
    let instructions: Vec<((i32, i32), i32)> = file.lines()
        .filter_map(Result::ok)
        .map(|s| {
            let v: (&str, &str) = s.split(" ").collect_tuple().unwrap();

            (getStep(v.0), v.1.parse::<i32>().unwrap())
        })
        .collect();

    println!("Positions visited 1: {}", simulateKnots(&instructions, 2));
    println!("Positions visited 2: {}", simulateKnots(&instructions, 10));
}