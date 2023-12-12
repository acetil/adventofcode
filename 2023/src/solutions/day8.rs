use std::{io::{BufReader, self, BufRead}, fs::File, collections::HashMap};

use itertools::Itertools;
use regex::Regex;

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right
}

impl Direction {
    pub fn new (c: char) -> Option<Self> {
        match c {
            'L' => Some(Self::Left),
            'R' => Some(Self::Right),
            _ => None
        }
    }
}

struct Node {
    label: String,
    left: usize,
    right: usize
}

impl Node {
    pub fn new (label: String, left: usize, right: usize) -> Self {
        Self { 
            label, 
            left, 
            right 
        }
    }

    pub fn next (&self, dir: Direction) -> usize {
        match dir {
            Direction::Left => self.left,
            Direction::Right => self.right
        }
    }
}

struct Graph {
    nodes: Vec<Node>,
    node_labels: HashMap<String, usize>
}

impl Graph {
    pub fn new (lines: Vec<&str>) -> Self {
        let regex = Regex::new(r"([0-9A-Z]{3}) = \(([0-9A-Z]{3}), ([0-9A-Z]{3})\)").unwrap();

        let node_tups: Vec<(&str, &str, &str)> = lines.iter()
            .map(|s| regex.captures(s).unwrap())
            .filter_map(|c| c.iter()
                .skip(1)
                .filter_map(|i| i.map(|m| m.as_str()))
                .collect_tuple())
            .collect_vec();

        let mut node_labels: HashMap<String, usize> = HashMap::new();
        for (i, (l, _, _)) in node_tups.iter().enumerate() {
            node_labels.insert(l.to_string(), i);
        }

        let nodes = node_tups.iter()
            .map(|(label, l, r)| Node::new(label.to_string(), node_labels[*l], node_labels[*r]))
            .collect_vec();

        Self { 
            nodes, 
            node_labels
        }
    }

    pub fn get_id (&self, label: &str) -> Option<usize> {
        self.node_labels.get(label)
            .copied()
    }

    pub fn next (&self, id: usize, dir: Direction) -> usize {
        self.nodes[id].next(dir)
    }

    pub fn end_steps (&self, id: usize, instructions: &Vec<Direction>) -> usize {
        let mut len = 0usize;
        let mut pos = 0usize;
        let mut curr = id;
        loop {
            if &self.nodes[curr].label[2..] == "Z" {
                return len;
            }

            curr = self.next(curr, instructions[pos]);
            pos = (pos + 1) % instructions.len();
            len += 1;
        }
    }

    pub fn solve_p2 (&self, instructions: &Vec<Direction>) -> usize {
        self.nodes.iter()
            .enumerate()
            .filter(|(_, n)| &n.label[2..] == "A")
            .map(|(i, _)| self.end_steps(i, instructions))
            .reduce(lcm)
            .unwrap()
    }
}

fn gcd (a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm (a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn solve (input: &str) {
    let mut lines = input.lines();

    let instructions = lines.next().unwrap()
        .trim()
        .chars()
        .filter_map(Direction::new)
        .collect_vec();

    let graph = Graph::new(lines.skip(1).collect());

    let mut curr = graph.get_id("AAA").unwrap();
    let target = graph.get_id("ZZZ").unwrap();

    let mut pos = 0usize;
    let mut len = 0usize;

    while curr != target {
        curr = graph.next(curr, instructions[pos]);
        pos = (pos + 1) % instructions.len();
        len += 1;
    }

    println!("Part 1: {len}");
    println!("Part 2: {}", graph.solve_p2(&instructions));
}