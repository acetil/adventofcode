use std::{cell::RefCell, rc::Rc, collections::HashMap};

use regex::Regex;

struct Gear {
    adj: Vec<usize>
}

pub fn solve (input: &str) {
    let lines: Vec<&str> = input.lines()
        .collect();

    let y_size = lines.len();
    let x_size = lines[0].len();

    let mut special_positions: Vec<((usize, usize), Option<Rc<RefCell<Gear>>>)> = Vec::new();
    let mut numbers: Vec<(usize, (usize, usize), usize)> = Vec::new();
    let mut gears: Vec<Rc<RefCell<Gear>>> = Vec::new();

    let num_regex = Regex::new(r"[0-9]+").unwrap();

    let mut y = 0;
    for l in &lines {
        let mut x: usize = 0;
        for c in l.chars() {
            if !c.is_ascii_digit() && c != '.' {
                if c == '*' {
                    let gear = Rc::new(RefCell::new(Gear{
                        adj: Vec::new()
                    }));
                    gears.push(gear.clone());
                    special_positions.push(((x, y), Some(gear)));
                } else {
                    special_positions.push(((x, y), None));
                }
            }

            x += 1;
        }

        for i in num_regex.find_iter(l) {
            numbers.push((i.as_str().parse().unwrap(), (i.start(), i.end()), y));
        }

        y += 1;
    }

    let mut valid_parts: HashMap<(usize, usize), Option<Rc<RefCell<Gear>>>> = HashMap::new();
    for ((x, y), opt) in special_positions {
        if y > 0 {
            if x > 0 {
                valid_parts.insert((x - 1, y - 1), opt.clone());
            }

            valid_parts.insert((x, y - 1), opt.clone());

            if x + 1 < x_size {
                valid_parts.insert((x + 1, y - 1), opt.clone());
            }
        }

        if x > 0 {
            valid_parts.insert((x - 1, y), opt.clone());
        }

        valid_parts.insert((x, y), opt.clone());

        if x + 1 < x_size {
            valid_parts.insert((x + 1, y), opt.clone());
        }

        if y + 1 < y_size {
            if x > 0 {
                valid_parts.insert((x - 1, y + 1), opt.clone());
            }

            valid_parts.insert((x, y + 1), opt.clone());

            if x + 1 < x_size {
                valid_parts.insert((x + 1, y + 1), opt.clone());
            }
        }
    }

    let mut part_sum: usize = 0;

    for (n, (start, end), y) in numbers {
        for i in start..end {
            if valid_parts.contains_key(&(i, y)) {
                part_sum += n;

                if let Some(gear) = valid_parts.get(&(i, y)).unwrap() {
                    gear.borrow_mut().adj.push(n);
                }
                break;
            }
        }
    }

    println!("Part 1: {part_sum}");

    let gear_sum: usize = gears.iter()
        .map(|rc| rc.borrow())
        .filter(|gear| gear.adj.len() == 2)
        .map(|gear| gear.adj.iter()
            .fold(1, |a, b| a * b))
        .sum();


    println!("Part 2: {gear_sum}");
}