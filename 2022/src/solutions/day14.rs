use std::{io::{BufReader, BufRead}, fs::File};

#[derive(Copy, Clone, Debug)]
enum Tile {
    EMPTY,
    ROCK,
    SAND
}

enum PlaceResult {
    SUCCESS,
    FAILURE,
    OOB
}

impl PlaceResult {
    pub fn isSuccess (&self) -> bool {
        match self {
            Self::SUCCESS => true,
            _ => false
        }
    }

    pub fn isFailure (&self) -> bool {
        match self {
            Self::FAILURE => true,
            _ => false
        }
    }

    pub fn isOOB (&self) -> bool {
        match self {
            Self::OOB => true,
            _ => false
        }
    }
}

impl Tile {
    pub fn isRock (&self) -> bool {
        match self {
            Tile::ROCK => true,
            _ => false
        }
    }

    pub fn isSand (&self) -> bool {
        match self {
            Tile::SAND => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone)]
struct Cave {
    tiles: Vec<Vec<Tile>>,
    xOff: isize
}

impl Cave {
    pub fn new () -> Cave {
        Cave { 
            tiles: vec![Vec::new()], 
            xOff: 0
        }
    }

    fn getTile (&mut self, pos: (isize, usize)) -> Tile {
        while pos.0 < self.xOff {
            self.tiles.insert(0, Vec::new());
            self.xOff -= 1;
        }

        while pos.0 - self.xOff >= self.tiles.len() as isize {
            self.tiles.push(Vec::new());
        }
        let tileVec: &mut Vec<Tile> = self.tiles.get_mut((pos.0 - self.xOff) as usize).unwrap();

        while tileVec.len() <= pos.1 {
            tileVec.push(Tile::EMPTY);
        }

        *tileVec.get(pos.1).unwrap()
    }

    fn trySetTile (&mut self, pos: (isize, usize), tile: Tile) -> bool {
        let curr = self.getTile(pos);

        if curr.isRock() {
            return false;
        } else if curr.isSand() && tile.isSand() {
            return false;
        } 

        let tileVec: &mut Vec<Tile> = self.tiles.get_mut((pos.0 - self.xOff) as usize).unwrap();
        tileVec[pos.1] = tile;

        true
    }

    fn trySetTileBounded (&mut self, pos: (isize, usize), tile: Tile, bound: usize) -> PlaceResult {
        if pos.1 >= bound {
            PlaceResult::OOB
        } else {
            if self.trySetTile(pos, tile) {
                PlaceResult::SUCCESS
            } else {
                PlaceResult::FAILURE
            }
        }
    }

    pub fn addFormation (&mut self, formation: ((isize, usize), (isize, usize))) {
        let start = formation.0;
        let delta: (isize, isize) = (formation.1.0 - start.0, formation.1.1 as isize - start.1 as isize);
        let n: isize = std::cmp::max(delta.0.abs(), delta.1.abs());

        let step: (isize, isize) = (delta.0 / (n as isize), delta.1 / n);

        for i in 0..n + 1 {
            self.trySetTile(((start.0 + step.0 * i) as isize, (start.1 as isize + step.1 * i) as usize), Tile::ROCK);
        }
    }

    fn sandStep (&mut self, pos: (isize, usize), bound: usize) -> (PlaceResult, (isize, usize)) {
        let mut result = self.trySetTileBounded((pos.0, pos.1 + 1), Tile::SAND, bound);
        if result.isSuccess() || result.isOOB() {
            assert!(self.trySetTile(pos, Tile::EMPTY));
            return (result, (pos.0, pos.1 + 1));
        } 

        result = self.trySetTileBounded((pos.0 - 1, pos.1 + 1), Tile::SAND, bound);
        if result.isSuccess() || result.isOOB() {
            assert!(self.trySetTile(pos, Tile::EMPTY));
            return (result, (pos.0 - 1, pos.1 + 1));
        }

        result = self.trySetTileBounded((pos.0 + 1, pos.1 + 1), Tile::SAND, bound);
        if result.isSuccess() || result.isOOB() {
            assert!(self.trySetTile(pos, Tile::EMPTY));
        }
        return (result, (pos.0 + 1, pos.1 + 1));
    }

    pub fn addSand (&mut self, bound: usize) -> bool {
        let mut currPos: (isize, usize) = (0, 0);

        if !self.trySetTile(currPos, Tile::SAND) {
            return false
        }

        loop {
            let (result, newPos) = self.sandStep(currPos, bound);
            if result.isOOB() {
                return false;
            } else if result.isFailure() {
                return true;
            } else {
                currPos = newPos;
            }
        }
    }

    pub fn getMaxY (&self) -> usize {
        self.tiles.iter().map(Vec::len).max().unwrap()
    }
}

fn parsePos (pos: &str) -> (isize, usize) {
    let parts: Vec<&str> = pos.trim().split(",").collect();

    (parts[0].parse::<isize>().unwrap() - 500, parts[1].parse().unwrap())
}

fn parseFormations (line: &str) -> Vec<((isize, usize), (isize, usize))> {
    let parts: Vec<(isize, usize)> = line.split("->").map(|s| parsePos(s)).collect();



    (1..parts.len())
        .map(|i| (parts[i - 1], parts[i]))
        .collect()
}

pub fn solve (file: BufReader<File>) {
    let mut cave = Cave::new();
    
    for i in file.lines().filter_map(Result::ok).map(|s| parseFormations(&s)).flatten() {
        cave.addFormation(i);
    }

    let yBound = cave.getMaxY();

    let mut num1: i32 = 0;
    while cave.addSand(yBound) {
        num1 += 1;
    }

    let mut num2: i32 = num1;
    for i in -((yBound + 2 + 1) as isize)..((yBound + 2 + 1 + 1) as isize) {
        assert!(cave.trySetTile((i, yBound + 1), Tile::ROCK));
    } 

    while cave.addSand(yBound + 2) {
        num2 += 1;
    }

    println!("Sand number 1: {}", num1);
    println!("Sand number 2: {}", num2);
}