use std::{io::{BufReader, self, BufRead}, fs::File, cmp::min};

struct Card {
    //id: u32,
    win_nums: Vec<u32>,
    card_nums: Vec<u32>
}

impl Card {
    pub fn new (line: &str) -> Self {
        let nums_str = line.split(":").last().unwrap();
        let mut bar_split = nums_str.split(" | ");
        let mut win_nums: Vec<u32> = bar_split.next()
            .unwrap()
            .trim()
            .split(" ")
            .filter(|&s| !s.is_empty())
            .map(str::parse)
            .map(Result::unwrap)
            .collect();
        let mut card_nums: Vec<u32> = bar_split.next()
            .unwrap()
            .trim()
            .split(" ")
            .filter(|&s| !s.is_empty())
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        win_nums.sort();
        card_nums.sort();
        
        Self {
            //id,
            win_nums,
            card_nums
        }
    }

    pub fn matches (&self) -> u32 {
        let mut win_it = self.win_nums.iter().peekable();
        let mut card_it = self.card_nums.iter().peekable();

        let mut matches: u32 = 0;
        while win_it.peek().is_some() && card_it.peek().is_some() {
            if win_it.peek() == card_it.peek() {
                matches += 1;
                win_it.next();
                card_it.next();
            } else if win_it.peek() < card_it.peek() {
                win_it.next();
            } else {
                card_it.next();
            }
        }

        matches
    }
}

pub fn solve (input: &str){
    let cards: Vec<Card> = input.lines()
        .map(Card::new)
        .collect();

    let part1: u32 = cards.iter()
        .map(Card::matches)
        .filter(|i| *i > 0u32)
        .map(|i| 1u32 << (i - 1))
        .sum();
    println!("Part 1: {part1}");

    let mut copies: Vec<u32> = Vec::new();
    for _ in 0..cards.len() {
        copies.push(1);
    }

    for i in 0..cards.len() {
        for j in (i+1)..min(i + 1 + cards[i].matches() as usize, cards.len()) {
            copies[j] += copies[i];
        }
    }

    let part2: u32 = copies.iter().sum();
    println!("Part 2: {part2}");
}