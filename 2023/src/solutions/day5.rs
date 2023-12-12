use std::{iter::Peekable, cmp::min};

use itertools::Itertools;

struct RangeIt <I: Iterator> {
    iter: Peekable<I>,
    range: (i64, i64)
}

impl <'a, I: Iterator<Item=&'a ((i64, i64), i64)>> RangeIt<I> {
    pub fn new (iter: I, range: (i64, i64)) -> Self {
        Self { 
            iter: iter.peekable(), 
            range 
        }
    }
}

impl <'a, I: Iterator<Item=&'a((i64, i64), i64)>> Iterator for RangeIt<I> {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.range.0 == self.range.1 {
            return None;
        }

        while self.iter.peek().map(|r| r.0.1 <= self.range.0).unwrap_or(false) {
            self.iter.next();
        }

        if let Some(&(m, diff)) = self.iter.peek() {
            if self.range.0 < m.0 {
                let r = (self.range.0, min(self.range.1, m.0));
                self.range.0 = r.1;

                Some(r)
            } else {

                let r = (self.range.0, min(self.range.1, m.1));
                self.range.0 = r.1;
                self.iter.next();

                Some((r.0 + diff, r.1 + diff))
            }
        } else {
            let r = self.range;
            self.range = (r.1, r.1);
            Some(r)
        }
    }
}

fn parse_seeds (input: &str) -> Vec<i64> {
    input.split(":")
        .last().unwrap()
        .trim()
        .split(" ")
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

type Map = Vec<((i64, i64), i64)>;

fn parse_map (map: &str) -> Map {
    let lines = map.lines().skip(1);
    let mut map: Map = Vec::new();

    for l in lines {
        let nums: Vec<i64> = l.trim().split(" ")
            .map(str::parse)
            .map(Result::unwrap)
            .collect();
        map.push(((nums[1], nums[1] + nums[2]), nums[0] - nums[1]));
    }

    map.sort_by_key(|m| m.0);

    map
}

fn simulate <I: Iterator<Item=(i64, i64)>> (maps: &Vec<Map>, seed_ranges: I) -> i64 {
    seed_ranges
        .flat_map(|r| RangeIt::new(maps[0].iter(), r))
        .flat_map(|r| RangeIt::new(maps[1].iter(), r))
        .flat_map(|r| RangeIt::new(maps[2].iter(), r))
        .flat_map(|r| RangeIt::new(maps[3].iter(), r))
        .flat_map(|r| RangeIt::new(maps[4].iter(), r))
        .flat_map(|r| RangeIt::new(maps[5].iter(), r))
        .flat_map(|r| RangeIt::new(maps[6].iter(), r))
        .min()
        .unwrap()
        .0
}

pub fn solve (input: &str) {
    let mut in_iter = input.split("\n\n");

    let seeds: Vec<i64> = parse_seeds(in_iter.next().unwrap());

    let maps: Vec<Map> = in_iter
        .map(parse_map)
        .collect();

    let part1 = simulate(&maps, seeds.iter().map(|i| (*i, i + 1)));
    println!("Part 1: {part1}");

    let part2 = simulate(&maps, seeds.iter()
        .tuples()
        .map(|(a, b)| (*a, *a + *b)));
    println!("Part 2: {part2}");
}