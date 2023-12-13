type Pattern = Vec<Vec<char>>;

fn test_vertical (pattern: &Pattern, i: usize) -> bool {
    for l in pattern {
        let mut j: usize = 0;
        while i - j > 0 && i + j < l.len() {
            if l[i - j - 1] != l[i + j] {
                return false;
            }

            j += 1;
        }
    }

    true
}

fn test_horizontal (pattern: &Pattern, i: usize) -> bool {
    for x in 0..pattern[0].len() {
        let mut j: usize = 0;
        while i - j > 0 && i + j < pattern.len() {
            if pattern[i - j - 1][x] != pattern[i + j][x]  {
                return false;
            }
            j += 1;
        }
    }

    true
}

fn vertical_score (pattern: &Pattern, ignored: usize) -> Option<usize> {
    for i in 1..pattern[0].len() {
        if i != ignored && test_vertical(pattern, i) {
            return Some(i);
        }
    }

    None
}

fn horizontal_score (pattern: &Pattern, ignored: usize) -> Option<usize> {
    for i in 1..pattern.len() {
        if i * 100 != ignored && test_horizontal(pattern, i) {
            return Some(i * 100);
        }
    }

    None
}

fn fixed_score (pattern: &Pattern, old_score: usize) -> usize {
    let mut new_pattern: Pattern = pattern.clone();
    for y in 0..new_pattern.len() {
        for x in 0..new_pattern[y].len() {
            new_pattern[y][x] = if new_pattern[y][x] == '#' {
                '.'
            } else {
                '#'
            };

            if let Some(score) = horizontal_score(&new_pattern, old_score).or_else(|| vertical_score(&new_pattern, old_score)) {
                return score;
            }
        
            new_pattern[y][x] = pattern[y][x]; 
        }
    }

    dbg!(pattern);
    panic!("No fixed score found!")
}

pub fn solve (input: &str) {
    let patterns: Vec<Pattern> = input.split("\n\n")
        .map(|s| s.lines().map(|s| s.chars().collect()).collect())
        .collect();

    let scores: Vec<usize> = patterns.iter()
        .map(|p| horizontal_score(p, 0).or_else(|| vertical_score(p, 0)))
        .map(Option::unwrap)
        .collect();

    let part1: usize = scores.iter().sum();
    println!("Part 1: {part1}");

    let part2: usize = patterns.iter()
        .zip(scores.iter())
        .map(|(p, score)| fixed_score(p, *score))
        .sum();
    println!("Part 2: {part2}");
}