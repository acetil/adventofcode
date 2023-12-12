#[derive(Clone, Copy)]
enum RecordKind {
    Operational,
    Damaged,
    Unknown
}

impl RecordKind {
    pub fn from_char (c: char) -> Self {
        match c {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            _ => Self::Unknown
        }
    }

    pub fn is_operational (&self) -> bool {
        match self {
            Self::Operational | Self::Unknown => true,
            _ => false
        }
    }

    pub fn is_damaged (&self) -> bool {
        match self {
            Self::Damaged | Self::Unknown => true,
            _ => false
        }
    }
}

fn calc_arrangements (record: &[RecordKind], groups: &[usize], prev: usize) -> usize {
    let g = *groups.first().unwrap_or(&0);
    if record.len() == 0 {
        if groups.len() <= 1 && g == prev {
            return 1;
        } else {
            return 0;
        }
    }

    let kind = record.first().unwrap().clone();
    let mut ans: usize = 0;

    if kind.is_operational() {
        if prev == 0 {
            ans += calc_arrangements(&record[1..], groups, 0);
        } else if prev == g {
            ans += calc_arrangements(&record[1..], &groups[1..], 0);
        }
    }
    if kind.is_damaged() && prev < g {
        ans += calc_arrangements(&record[1..], groups, prev + 1);
    }

    ans
}

type Memo = Vec<Vec<Vec<Option<usize>>>>;
fn calc_arrangements_memo (memo: &mut Memo, record: &[RecordKind], groups: &[usize], prev: usize) -> usize {
    if let Some(ans) = memo[record.len()][groups.len()][prev] {
        return ans;
    }

    let g = *groups.first().unwrap_or(&0);
    if record.len() == 0 {
        if groups.len() <= 1 && g == prev {
            return 1;
        } else {
            return 0;
        }
    }

    let kind = record.first().unwrap().clone();
    let mut ans: usize = 0;

    if kind.is_operational() {
        if prev == 0 {
            ans += calc_arrangements_memo(memo, &record[1..], groups, 0);
        } else if prev == g {
            ans += calc_arrangements_memo(memo, &record[1..], &groups[1..], 0);
        }
    }
    if kind.is_damaged() && prev < g {
        ans += calc_arrangements_memo(memo, &record[1..], groups, prev + 1);
    }

    memo[record.len()][groups.len()][prev] = Some(ans);

    ans
}

struct Row {
    record: Vec<RecordKind>,
    groups: Vec<usize>
}

impl Row {
    pub fn new (line: &str) -> Self {
        let mut split = line.split(" ");

        Self {
            record: split.next().unwrap()
                .chars()
                .map(RecordKind::from_char)
                .collect(),
            groups: split.next().unwrap()
                .split(",")
                .map(str::parse)
                .map(Result::unwrap)
                .collect()
        }
    }

    pub fn get_arrangements (&self) -> usize {
        calc_arrangements(&self.record, &self.groups, 0)
    }

    pub fn get_arrangements_unfold (&self) -> usize {
        let mut memo: Memo = Memo::new();
        let mut record_unfold: Vec<RecordKind> = Vec::new();
        let mut groups_unfold: Vec<usize> = Vec::new();

        for i in 0..5 {
            record_unfold.extend(self.record.iter());
            groups_unfold.extend(self.groups.iter());

            if i < 4 {
                record_unfold.push(RecordKind::Unknown);
            }
        }

        let group_max = *self.groups.iter()
            .max()
            .unwrap();

        for _ in 0..record_unfold.len() + 1 {
            memo.push(Vec::new());
            let curr_r = memo.last_mut().unwrap();
            
            for _ in 0..groups_unfold.len() + 1 {
                curr_r.push(Vec::new());
                let curr_g = curr_r.last_mut().unwrap();

                for _ in 0..group_max + 1 {
                    curr_g.push(None);
                }
            }
        }

        let r = calc_arrangements_memo(&mut memo, &record_unfold, &groups_unfold, 0);

        r
    }
}

pub fn solve (input: &str) {
    let rows: Vec<Row> = input.lines()
        .map(Row::new)
        .collect();

    let part1: usize = rows.iter()
        .map(Row::get_arrangements)
        .sum();
    println!("Part 1: {part1}");

    let part2: usize = rows.iter()
        .map(Row::get_arrangements_unfold)
        .sum();
    println!("Part 2: {part2}");
}