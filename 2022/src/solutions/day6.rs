use std::{io::{BufReader, BufRead}, fs::File};

use itertools::Itertools;

fn findFirstMarker (buffer: &String, numDistinct: usize) -> Option<usize> {
    return (numDistinct..buffer.len() + 1)
        .filter(|i| buffer[i - numDistinct..*i].chars().all_unique())
        .nth(0);
}

pub fn solve (mut file: BufReader<File>) {
    let mut buffer: String = String::new();

    file.read_line(&mut buffer).expect("Failed to read datastream buffer!");

    println!("First marker found at {}", findFirstMarker(&buffer, 4).expect("Failed to find datastream marker!"));
    println!("First message found at {}", findFirstMarker(&buffer, 14).expect("Failed to find message marker!"));
}