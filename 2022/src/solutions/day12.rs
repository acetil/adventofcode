use std::{io::{BufReader, BufRead}, fs::File, collections::{HashMap, VecDeque}};

struct HeightMap {
    elevations: Vec<usize>,
    width: usize,
    height: usize,
    startIndex: usize,
    endIndex: usize
}

impl HeightMap {
    pub fn new<T> (lines: T) -> HeightMap where T: Iterator<Item = String> {
        let mut width: usize = 0;
        let mut height: usize = 0;

        let mut elevations: Vec<usize> = Vec::new();

        let mut startIndex: usize = 0;
        let mut endIndex: usize = 0;

        for l in lines {
            let mut lineX = 0;
            for c in l.chars() {
                let posHeight: usize;
                if c == 'S' {
                    posHeight = 0;
                    startIndex = elevations.len();
                } else if c == 'E' {
                    posHeight = 25;
                    endIndex = elevations.len();
                } else {
                    posHeight = c as usize - 'a' as usize;
                }

                elevations.push(posHeight);
                lineX += 1;
            }

            width = std::cmp::max(lineX, width);
            height += 1;
        }

        HeightMap { 
            elevations, 
            width, 
            height,
            startIndex,
            endIndex
        }
    }

    pub fn findShortestPathLen (self: &Self, start: usize) -> Option<usize> {
        let mut distances: HashMap<usize, usize> = HashMap::new();
        distances.insert(start, 0);
        let mut indexStack: VecDeque<usize> = VecDeque::from(vec![start]);

        while indexStack.len() != 0 {
            let curr = indexStack.pop_front().unwrap();
            
            if curr == self.endIndex {
                break;
            }

            for i in self.getAdjacentIndices(curr) {
                if distances.contains_key(&i) {
                    continue;
                }

                distances.insert(i, distances[&curr] + 1);
                indexStack.push_back(i);
            }
        }

        distances.get(&self.endIndex).map(|i| *i)
    }

    pub fn findShortestPathFromStart (self: &Self) -> Option<usize> {
        self.findShortestPathLen(self.startIndex)
    }

    pub fn findShortestOverallPath (self: &Self) -> usize {
        (0..self.elevations.len()).filter(|i| self.elevations[*i] == 0).filter_map(|i| self.findShortestPathLen(i)).min().unwrap()
    }

    fn getAdjacentIndices (self: &Self, index: usize) -> Vec<usize> {
        let mut indices: Vec<usize> = Vec::new();

        if index % self.width > 0 {
            indices.push(index - 1);
        }

        if index % self.width < self.width - 1 {
            indices.push(index + 1);
        }

        if index / self.width > 0 {
            indices.push(index - self.width);
        }

        if index / self.width < self.height - 1 {
            indices.push(index + self.width);
        }

        indices.iter().filter(|i| self.elevations[**i] as isize - self.elevations[index] as isize <= 1).map(|i| *i).collect()
    }
}

pub fn solve (file: BufReader<File>) {
    let heightMap = HeightMap::new(file.lines().filter_map(Result::ok));

    println!("Shortest path: {}", heightMap.findShortestPathFromStart().unwrap());
    println!("Shortest overall path: {}", heightMap.findShortestOverallPath());
}