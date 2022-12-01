use std::{fs::File, io::{BufReader, BufRead}};

pub fn solve (file: BufReader<File>) {
    let mut elves: Vec<i32> = Vec::new();
    elves.push(0);

    for i in file.lines() {
        let line = i.expect("Error reading line!");

        match line.trim().parse::<i32>() {
            Ok(val) => *elves.last_mut().unwrap() += val,
            Err(_) => elves.push(0)
        }
    }

    elves.sort();
    elves.reverse();

    println!("Max calories: {}", elves.get(0).unwrap());
    println!("Sum of max calories: {}", elves.iter().take(3).sum::<i32>());
}