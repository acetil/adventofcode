enum Instruction <'a> {
    Remove(&'a str),
    Add(&'a str, u32)
}

impl <'a> Instruction <'a> {
    pub fn new (s: &'a str) -> Self{
        let mut ch_it = s.char_indices().skip_while(|(_, c)| *c != '-' && *c != '=');
        let sep = ch_it.next().unwrap();

        if sep.1 == '-' {
            Self::Remove(&s[0..sep.0])
        } else {
            Self::Add(&s[0..sep.0], ch_it.next().unwrap().1.to_digit(10).unwrap())
        }
    }
}

fn hash (s: &str) -> u32 {
    let mut hash_val : u32 = 0;

    for c in s.chars() {
        hash_val += c as u32;
        hash_val = (hash_val * 17) % 256;
    }

    hash_val
}

pub fn solve (input: &str) {
    let inputs: Vec<&str> = input.trim().split(",").collect();

    let part1: u32 = inputs.iter()
        .map(|&s| hash(s))
        .sum();
    println!("Part 1: {part1}");

    const V: Vec<(&str, u32)> = Vec::new();
    let mut boxes: [Vec<(&str, u32)>; 256] = [V; 256];
    for i in inputs.iter().map(|&s| Instruction::new(s)) {
        match i {
            Instruction::Remove(label) => boxes[hash(label) as usize].retain(|(l, _)| *l != label),
            Instruction::Add(label, focus) => {
                let mut done: bool = false;
                let box_ref = boxes.get_mut(hash(label) as usize).unwrap();
                for (l, f) in box_ref {
                    if *l == label {
                        *f = focus;
                        done = true;
                        break;
                    }
                }

                let box_ref = boxes.get_mut(hash(label) as usize).unwrap();
                if !done {
                    box_ref.push((label, focus));
                }
                //dbg!(&box_ref);
            }
        }
    }

    //dbg!(&boxes);
    let part2: u32 = boxes.iter()
        .enumerate()
        .map(|(i, v)| v.iter()
            .enumerate()
            .map(move |(j, (_, f))| (i as u32 + 1) * (j + 1) as u32 * f.clone()))
        .flatten()
        .sum();
    println!("Part 2: {part2}");
}