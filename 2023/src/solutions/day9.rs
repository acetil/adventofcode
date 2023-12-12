fn seq_next (seq: &Vec<i64>) -> i64 {
    if seq.len() == 1 {
        return *seq.first().unwrap();
    }

    let mut diff_seq: Vec<i64> = Vec::new();
    let mut last = seq.first().unwrap();
    let mut num_zeroes = 0;

    for i in seq.iter().skip(1) {
        diff_seq.push(*i - last);
        if *i - last == 0 {
            num_zeroes += 1;
        }

        last = i;
    }

    if num_zeroes == diff_seq.len() {
        *seq.last().unwrap()
    } else {
        *seq.last().unwrap() + seq_next(&diff_seq)
    }
}

fn seq_prev (seq: &Vec<i64>) -> i64 {
    if seq.len() == 1 {
        return *seq.first().unwrap();
    }

    let mut diff_seq: Vec<i64> = Vec::new();
    let mut last = seq.first().unwrap();
    let mut num_zeroes = 0;

    for i in seq.iter().skip(1) {
        diff_seq.push(*i - last);
        if *i - last == 0 {
            num_zeroes += 1;
        }

        last = i;
    }

    if num_zeroes == diff_seq.len() {
        *seq.first().unwrap()
    } else {
        *seq.first().unwrap() - seq_prev(&diff_seq)
    }
}

pub fn solve (input: &str) {
    let lines: Vec<Vec<i64>> = input.lines()
        .map(|l| l.trim()
            .split(" ")
            .map(str::parse)
            .map(Result::unwrap)
            .collect())
        .collect();
    
    let part1: i64 = lines.iter()
        .map(seq_next)
        .sum();
    println!("Part 1: {part1}");

    let part2: i64 = lines.iter()
        .map(seq_prev)
        .sum();
    println!("Part 2: {part2}");
}