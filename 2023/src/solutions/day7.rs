use std::collections::HashMap;

use itertools::Itertools;

struct Hand {
    bid: i64,
    cards: [char; 5],
    counts: [u8; 6]
}

#[derive(PartialEq, Eq)]
struct ScoredHand {
    bid: i64,
    rank: u32,
    score_cards: [u8; 5]
}

impl Hand {
    pub fn new (line: &str) -> Self {
        let mut split = line.trim().split(" ");

        let cards: [char; 5] = split.next().unwrap().chars()
            .collect::<Vec<char>>()
            .try_into().unwrap();

        let bid: i64 = split.next().unwrap()
            .trim()
            .parse().unwrap();
        let mut counts: HashMap<char, u8> = HashMap::new();

        for i in cards {
            counts.entry(i).and_modify(|n| *n += 1).or_insert(1);
        }


        let mut num_counts: [u8; 6] = [0; 6];
        for (_, v) in counts.iter() {
            num_counts[*v as usize] += 1
        }

        Hand { 
            bid, 
            cards,
            counts: num_counts
        }
    }

    pub fn score_p1 (&self) -> ScoredHand {
        static CARDS_P1: &str = "23456789TJQKA";

        let score_cards = self.cards.map(|c| CARDS_P1.find(c).unwrap()).map(|u| u as u8);

        ScoredHand { 
            bid: self.bid, 
            rank: rank_counts(&self.counts), 
            score_cards: score_cards
        }
    }

    pub fn score_p2 (&self) -> ScoredHand {
        static CARDS_P2: &str = "J23456789TQKA";
        let score_cards = self.cards.map(|c| CARDS_P2.find(c).unwrap()).map(|u| u as u8);
        
        let mut new_counts = self.counts.clone();

        let num_jokers = score_cards.iter()
            .filter(|&i| *i == 0)
            .count();

        //dbg!(num_jokers);

        if num_jokers > 0 && num_jokers < 5 {
            new_counts[num_jokers as usize] -= 1;

            for i in (1..5).rev() {
                if new_counts[i] != 0 {
                    new_counts[i] -= 1;
                    new_counts[i + num_jokers as usize] += 1;
                    break;
                }
            }
        }
        //dbg!(&new_counts);

        ScoredHand {
            bid: self.bid, 
            rank: rank_counts(&new_counts), 
            score_cards 
        }
    }
}

impl Ord for ScoredHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank.cmp(&other.rank).then(self.score_cards.cmp(&other.score_cards))
    }
}

impl PartialOrd for ScoredHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn rank_counts (counts: &[u8;6]) -> u32 {
    if counts[5] == 1 {
        6
    } else if counts[4] == 1 {
        5
    } else if counts[3] == 1 && counts[2] == 1 {
        4
    } else if counts[3] == 1 {
        3
    } else if counts[2] == 2 {
        2
    } else if counts[2] == 1 {
        1
    } else {
        0
    }
}

pub fn solve (input: &str) {
    let hands: Vec<Hand> = input.lines()
        .map(Hand::new)
        .collect();

    let part1: i64 = hands.iter()
        .map(Hand::score_p1)
        .sorted()
        .enumerate()
        .map(|(i, h)| h.bid * (i + 1) as i64)
        .sum();
    println!("Part 1: {part1}");

    let part2: i64 = hands.iter()
        .map(Hand::score_p2)
        .sorted()
        .enumerate()
        .map(|(i, h)| h.bid * (i + 1) as i64)
        .sum();
    println!("Part 2: {part2}");
}