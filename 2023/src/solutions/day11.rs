pub fn get_sum (galaxies: &Vec<(isize, isize)>, prefix_rows: &Vec<isize>, prefix_cols: &Vec<isize>, mult: isize) -> isize {
    let mut sum = 0;
    let mut it = galaxies.iter();
    while let Some(g1) = it.next() {
        for g2 in it.clone() {
            sum += (g1.0 - g2.0).abs() + (prefix_cols[g1.0 as usize] - prefix_cols[g2.0 as usize]).abs() * (mult - 1);
            sum += (g1.1 - g2.1).abs() + (prefix_rows[g1.1 as usize] - prefix_rows[g2.1 as usize]).abs() * (mult - 1);
        }
    }

    sum
}

pub fn solve (input: &str) {
    let image: Vec<Vec<bool>> = input.lines()
        .map(|l| l.chars()
            .map(|c| c == '#')
            .collect())
        .collect();

    let galaxies: Vec<(isize, isize)> = image.iter()
        .enumerate()
        .map(|(y, v)| v.iter()
            .enumerate()
            .filter(|(_, b)| **b)
            .map(move |(x, _)| (x as isize, y as isize)))
        .flatten()
        .collect();

    let mut prefix_rows: Vec<isize> = Vec::new();
    for v in &image {
        let last = *prefix_rows.last().unwrap_or(&0);
        if v.iter().any(|b| *b) {
            prefix_rows.push(last);
        } else {
            prefix_rows.push(last + 1);
        }
    }

    let mut prefix_cols: Vec<isize> = Vec::new();
    for x in 0..image[0].len() {
        let last = *prefix_cols.last().unwrap_or(&0);

        if (0..image.len()).map(|y| image[y][x]).any(|b| b) {
            prefix_cols.push(last);
        } else {
            prefix_cols.push(last + 1);
        }
    }

    println!("Part 1: {}", get_sum(&galaxies, &prefix_rows, &prefix_cols, 2));
    println!("Part 2: {}", get_sum(&galaxies, &prefix_rows, &prefix_cols, 1e6 as isize));
}