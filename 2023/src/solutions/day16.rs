use std::collections::{HashSet, VecDeque};

type Pos = (isize, isize);

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl Direction {
    pub fn translate (&self, pos: Pos) -> Pos {
        match self {
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0 + 1, pos.1),
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Down => (pos.0, pos.1 + 1),
        }
    }
}

enum TileKind {
    Empty,
    ForwardMirror,
    BackwardMirror,
    VerticalSplitter,
    HorizontalSplitter
}

impl TileKind {
    pub fn new (c: char) -> Self {
        match c {
            '/'  => Self::ForwardMirror,
            '\\' => Self::BackwardMirror,
            '|'  => Self::VerticalSplitter,
            '-'  => Self::HorizontalSplitter,
            _    => Self::Empty 
        }
    }
}

struct Tile {
    kind: TileKind,
    energised: bool
}

impl Tile {
    pub fn new (c: char) -> Self {
        Self {
            kind: TileKind::new(c),
            energised: false
        }
    }

    pub fn enter (&mut self, pos: Pos, dir: Direction, queue: &mut VecDeque<(Pos, Direction)>) {
        self.energised = true;

        match self.kind {
            TileKind::Empty => queue.push_back((pos, dir)),
            TileKind::ForwardMirror => {
                let new_dir = match dir {
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                };
                queue.push_back((pos, new_dir));
            },
            TileKind::BackwardMirror => {
                let new_dir = match dir {
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                };
                queue.push_back((pos, new_dir));
            },
            TileKind::VerticalSplitter => {
                match dir {
                    Direction::Left | Direction::Right => {
                        queue.push_back((pos, Direction::Up));
                        queue.push_back((pos, Direction::Down));
                    },
                    _ => queue.push_back((pos, dir))
                }
            },
            TileKind::HorizontalSplitter => match dir {
                Direction::Up | Direction::Down => {
                    queue.push_back((pos, Direction::Left));
                    queue.push_back((pos, Direction::Right));
                },
                _ => queue.push_back((pos, dir))
            },
        }
    }
}

struct Grid {
    tiles: Vec<Vec<Tile>>
}

impl Grid {
    pub fn new (input: &str) -> Self {
        Self {
            tiles: input.trim()
                .lines()
                .map(|l| l.chars().map(Tile::new).collect())
                .collect()
        }
    }

    pub fn get_mut (&mut self, pos: Pos) -> Option<&mut Tile> {
        if pos.0 < 0 || pos.1 < 0 {
            None
        } else {
            self.tiles.get_mut(pos.1 as usize)
                .and_then(|v| v.get_mut(pos.0 as usize))
        }
    }

    pub fn energised_num (&self) -> usize {
        self.tiles.iter()
            .map(|v| v.iter()
                .filter(|t| t.energised)
                .count())
            .sum()
    }
}

pub fn solve (input: &str) {
    let mut grid = Grid::new(input);
    let mut visited: HashSet<(Pos, Direction)> = HashSet::new();

    let mut queue: VecDeque<(Pos, Direction)> = VecDeque::new();
    queue.push_back(((-1, 0), Direction::Right));

    while !queue.is_empty() {
        let pos_dir = queue.pop_front().unwrap();
        if visited.contains(&pos_dir) {
            continue;
        }
        visited.insert(pos_dir.clone());

        let (old_pos, dir) = pos_dir;
        let pos = dir.translate(old_pos);
        
        if let Some(tile) = grid.get_mut(pos) {
            tile.enter(pos, dir, &mut queue);
        }
    }

    println!("Part 1: {}", grid.energised_num());
}