use std::{io::{BufReader, BufRead}, fs::File};

use regex::Regex;

type Position = (i64, i64);

struct Sensor {
    radius: u64,
    pos: Position,
}

fn sub (pos1: Position, pos2: Position) -> Position {
    (pos1.0 - pos2.0, pos1.1 - pos2.1)
}

impl Sensor {
    pub fn new (sensorPos: Position, beaconPos: Position) -> Sensor {
        let diff = sub(beaconPos, sensorPos);
        let radius = (diff.0.abs() + diff.1.abs()) as u64;

        Self { 
            radius, 
            pos: sensorPos
        }
    }

    pub fn rowVals (&self, height: i64) -> Option<(i64, i64)> {
        let yDisp = (height - self.pos.1).abs();

        if yDisp >= self.radius as i64 {
            None
        } else {
            let maxXDisp = self.radius as i64 - yDisp;
            Some((self.pos.0 - maxXDisp, self.pos.0 + maxXDisp))
        }
    }

    pub fn colVals (&self, xPos: i64) -> Option<(i64, i64)> {
        let xDisp = (xPos - self.pos.0).abs();

        if xDisp >= self.radius as i64 {
            None
        } else {
            let maxYDisp = self.radius as i64 - xDisp;
            Some((self.pos.1 - maxYDisp, self.pos.1 + maxYDisp))
        }
    }
}

fn rangesIntersect (range1: (i64, i64), range2: (i64, i64)) -> bool {
    (range1.0 <= range2.0 && range2.0 <= range1.1) || (range2.0 <= range1.0 && range1.0 <= range2.1)
}

fn rangesMerge (range1: (i64, i64), range2: (i64, i64)) -> (i64, i64) {
    assert!(rangesIntersect(range1, range2));

    (std::cmp::min(range1.0, range2.0), std::cmp::max(range1.1, range2.1))
}

fn clampRanges (ranges: Vec<(i64, i64)>, maxRange: (i64, i64)) -> Vec<(i64, i64)> {
    let r = ranges.iter()
        .map(|r| (std::cmp::max(r.0, maxRange.0), std::cmp::min(r.1, maxRange.1)))
        .collect();
    
    //dbg!(&r);

    r
}

fn addRange (mut ranges: Vec<(i64, i64)>, mut newRange: (i64, i64)) -> Vec<(i64, i64)> {
    let mut index = 0;

    while ranges.len() != 0 && index < ranges.len() && ranges[index].0 <= newRange.1 {
        if rangesIntersect(ranges[index], newRange) {
            let oldRange = ranges.remove(index);
            newRange = rangesMerge(newRange, oldRange);
        } else {
            index += 1;
        }
    }

    ranges.insert(index, newRange);

    //dbg!(&ranges);

    ranges
}

fn getExclusionsRow (sensors: &Vec<Sensor>, row: i64) -> Vec<(i64, i64)> {
    sensors.iter()
        .filter_map(|s| s.rowVals(row))
        .fold(Vec::new(), addRange)
}

fn getExclusionsCol (sensors: &Vec<Sensor>, col: i64) -> Vec<(i64, i64)> {
    sensors.iter()
        .filter_map(|s| s.colVals(col))
        .fold(Vec::new(), addRange)
}

fn rangeSize (ranges: Vec<(i64, i64)>) -> i64 {
    ranges.iter()
        .map(|p| p.1 - p.0 + 1)
        .reduce(|a, b| a + b)
        .unwrap()
}

fn findPoint (sensors: &Vec<Sensor>, xMax: i64, yMax: i64) ->(usize, usize) {
    /*let xPos = (0..xMax)
        .map(|i| getExclusionsCol(sensors, i))
        .map(|r| clampRanges(r, (0, xMax)))
        .map(rangeSize)
        .enumerate()
        .filter(|r| r.1 < xMax)
        .next()
        .unwrap()
        .0;*/

        let xVec: Vec<(usize, i64)> = (0..xMax)
        .map(|i| getExclusionsCol(sensors, i))
        .map(|r| clampRanges(r, (0, xMax)))
        .map(rangeSize)
        .enumerate()
        .collect();
    //dbg!(&xVec);
    let xPos = xVec.iter()
        .filter(|r| r.1 < xMax + 1)
        .next()
        .unwrap()
        .0;

    let yPos = (0..xMax)
        .map(|i| getExclusionsRow(sensors, i))
        .map(|r| clampRanges(r, (0, yMax)))
        .map(rangeSize)
        .enumerate()
        .filter(|r| r.1 < yMax + 1)
        .next()
        .unwrap()
        .0; 
    
    (xPos, yPos)
}

// line regex: ^.*?x=([\d-]+), y=([\d-]+):.*?x=([\d-]+), y=([\d-]+)

pub fn solve (file: BufReader<File>) {
    let re = Regex::new(r"^.*?x=([\d-]+), y=([\d-]+):.*?x=([\d-]+), y=([\d-]+)").unwrap();

    let sensors: Vec<Sensor> = file.lines()
        .filter_map(Result::ok)
        .map(|l| {
            let capture = re.captures(l.as_str()).unwrap();
            ((capture.get(1).unwrap().as_str().parse().unwrap(), capture.get(2).unwrap().as_str().parse().unwrap()), (capture.get(3).unwrap().as_str().parse().unwrap(), capture.get(4).unwrap().as_str().parse().unwrap()))
        })
        .map(|p| Sensor::new(p.0, p.1))
        .collect();

    
    let exclusions = sensors.iter()
        .filter_map(|s| s.rowVals(2000000))
        //.filter_map(|s| s.rowVals(10))
        .fold(Vec::new(), addRange);

    let numExcluded = exclusions.iter()
        .map(|p| p.1 - p.0)
        .reduce(|a, b| a + b)
        .unwrap();
    
    println!("Number excluded: {}", numExcluded);
    
    let beaconPoint = findPoint(&sensors, 4000000, 4000000);
    println!("Point: {:?}", beaconPoint);
    println!("Tuning frequency: {}", beaconPoint.0 *4000000 +  beaconPoint.1);
} 