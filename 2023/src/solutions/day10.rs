use std::{io::{BufReader, self, BufRead}, fs::File};

use itertools::Itertools;


type Pos = (isize, isize);

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl Direction {
    pub fn reverse (&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }

    pub fn is_down (&self) -> bool {
        matches!(self, Direction::Down)
    }

    pub fn is_up (&self) -> bool {
        matches!(self, Direction::Up)
    }

    pub fn is_left (&self) -> bool {
        matches!(self, Direction::Left)
    }

    pub fn is_right (&self) -> bool {
        matches!(self, Direction::Right)
    }

    pub fn translate (&self, pos: Pos) -> Pos {
        match self {
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0 + 1, pos.1),
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Down => (pos.0, pos.1 + 1),
        }
    }
}

enum CellKind {
    UpDown,
    LeftRight,
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
    None,
    Start
}

impl CellKind {
    pub fn from_char (c: char) -> Self {
        match c {
            'S' => Self::Start,
            '|' => Self::UpDown,
            '-' => Self::LeftRight,
            'L' => Self::UpRight,
            'J' => Self::UpLeft,
            '7' => Self::DownLeft,
            'F' => Self::DownRight,
            _ => Self::None
        }
    }

    pub fn from_dir (dir1: Direction, dir2: Direction) -> Self {
        match (dir1, dir2) {
            (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => Self::UpDown,
            (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => Self::LeftRight,
            (Direction::Up, Direction::Right) | (Direction::Right, Direction::Up) => Self::UpRight,
            (Direction::Up, Direction::Left) | (Direction::Left, Direction::Up) => Self::UpLeft,
            (Direction::Down, Direction::Left) | (Direction::Left, Direction::Down) => Self::DownLeft,
            (Direction::Down, Direction::Right) | (Direction::Right, Direction::Down) => Self::DownRight,
            _ => Self::None
        }
    }

    pub fn accept (&self, dir: Direction) -> Option<Direction> {
        match *self {
            CellKind::UpDown => if dir.is_up() {
                Some(Direction::Down)
            } else if dir.is_down() {
                Some(Direction::Up)
            } else {
                None
            },
            CellKind::LeftRight => if dir.is_left() {
                Some(Direction::Right)
            } else if dir.is_right() {
                Some(Direction::Left)
            } else {
                None
            },
            CellKind::UpRight => if dir.is_up() {
                Some(Direction::Right)
            } else if dir.is_right() {
                Some(Direction::Up)
            } else {
                None
            },
            CellKind::UpLeft => if dir.is_up() {
                Some(Direction::Left)
            } else if dir.is_left() {
                Some(Direction::Up)
            } else {
                None
            },
            CellKind::DownLeft => if dir.is_down() {
                Some(Direction::Left)
            } else if dir.is_left() {
                Some(Direction::Down)
            } else {
                None
            },
            CellKind::DownRight => if dir.is_down() {
                Some(Direction::Right)
            } else if dir.is_right() {
                Some(Direction::Down)
            } else {
                None
            },
            _ => None
        }
    }
    
    pub fn is_corner (&self) -> bool {
        match self {
            Self::UpLeft | Self::UpRight | Self::DownLeft | Self::DownRight => true,
            _ => false
        }
    }
}

struct Cell {
    kind: CellKind,
    visited: bool
}

impl Cell {
    pub fn new (c: char) -> Self {
        Self { 
            kind: CellKind::from_char(c), 
            visited: false 
        }
    }
}

struct Grid {
    cells: Vec<Vec<Cell>>,
    start: Pos
}

impl Grid {
    pub fn new (input: &str) -> Self {
        let cells: Vec<Vec<Cell>> = input.lines()
            .map(|s| s.chars().map(Cell::new).collect())
            .collect();

        let start = cells.iter()
            .enumerate()
            .map(|(y, v)| v.iter()
                .map(|c| &c.kind)
                .enumerate()
                .filter(|(_, k)| matches!(k, CellKind::Start))
                .map(move |(x, _)| (x.clone() as isize, y.clone() as isize))
            )
            .flatten()
            .next()
            .unwrap();

        Self {
            cells,
            start
        }
    }

    pub fn get (&'_ self, pos: Pos) -> Option<&'_ Cell> {
        if pos.0 < 0 || pos.1 < 0 {
            None
        } else {
            self.cells.get(pos.1 as usize)
                .and_then(|v| v.get(pos.0 as usize))
        }
    }

    pub fn get_mut (&'_ mut self, pos: Pos) -> Option<&'_ mut Cell> {
        if pos.0 < 0 || pos.1 < 0 {
            None
        } else {
            self.cells.get_mut(pos.1 as usize)
                .and_then(|v| v.get_mut(pos.0 as usize))
        }
    }

    pub fn width (&self) -> usize {
        self.cells[0].len()
    }

    pub fn height (&self) -> usize {
        self.cells.len()
    }
}

fn find_path_dir (grid: &mut Grid, start_dir: Direction) -> Option<Vec<Pos>> {
    let mut path_vec: Vec<Pos> = Vec::new();
    path_vec.push(grid.start);

    let mut curr = start_dir.translate(grid.start);
    let mut last_dir = start_dir;

    while curr != grid.start {
        path_vec.push(curr);
        last_dir = grid.get(curr)?
            .kind
            .accept(last_dir.reverse())?;
        curr = last_dir.translate(curr);
    }

    grid.get_mut(grid.start)?.kind = CellKind::from_dir(start_dir, last_dir.reverse());

    for &i in &path_vec {
        grid.get_mut(i)?.visited = true;
    }

    Some(path_vec)
}

fn find_path (grid: &mut Grid) -> Vec<Pos> {
    find_path_dir(grid, Direction::Left)
        .or_else(|| find_path_dir(grid, Direction::Right))
        .or_else(|| find_path_dir(grid, Direction::Up))
        .or_else(|| find_path_dir(grid, Direction::Down))
        .unwrap()
}

fn is_enclosed (pos: Pos, corners: &Vec<Pos>) -> bool {
    let real_pos = (pos.0 * 2 + 1, pos.1 *2 + 1);

    (0..corners.len())
        .map(|i| (corners[i], corners[(i + 1) % corners.len()]))
        .filter(|(p1, p2)| p1.1 != p2.1)
        .map(|(p1, p2)| ((p1.0 * 2, p1.1 * 2), (p2.0 * 2, p2.1 * 2)))
        .filter(|(p1, _)| p1.0 > real_pos.0)
        .map(|(p1, p2)| (std::cmp::min(p1.1, p2.1), std::cmp::max(p1.1, p2.1)))
        .filter(|(y1, y2)| *y1 < real_pos.1 && *y2 > real_pos.1)
        .count() % 2 == 1
}

pub fn solve (input: &str) {
    let mut grid = Grid::new(input);

    let path = find_path(&mut grid);

    println!("Part 1: {}", path.len() / 2);

    let corners = path.iter()
        .map(|p| p.clone())
        .filter(|p| grid.get(p.clone()).unwrap().kind.is_corner())
        .collect_vec();

    let mut enclosed: usize = 0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if grid.get((x as isize, y as isize)).unwrap().visited {
                continue;
            }

            if is_enclosed((x as isize, y as isize), &corners) {
                enclosed += 1;
            }
        }
    }
    println!("Part 2: {enclosed}");
}