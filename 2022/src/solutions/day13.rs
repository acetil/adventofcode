use std::{io::{BufReader, BufRead}, fs::File};

use itertools::Itertools;

struct CharStream {
    data: Vec<char>,
    pos: usize
}

impl CharStream {
    pub fn new (dataStr: &str) -> CharStream {
        CharStream { 
            data: dataStr.trim().chars().collect(), 
            pos: 0 
        }
    }

    pub fn peek (self: &Self) -> Option<char> {
        self.data.get(self.pos).map(|c| *c)
    }

    pub fn next (self: &mut Self) -> Option<char> {
        let oldPos = self.pos;
        self.pos += 1;

        self.data.get(oldPos).map(|c| *c)
    }

    pub fn require (self: &mut Self, reqC: char) {
        assert!(self.next().map(|c| c == reqC).unwrap_or(false));
    }

    pub fn takeUntil (self: &mut Self, reqC: &str) -> String {
        let mut result: String = String::new();

        let mut cOpt = self.peek();

        while cOpt.is_some() && !reqC.contains(cOpt.unwrap()) {
            result.push(self.next().unwrap());
            cOpt = self.peek();
        }

        result
    }
}

#[derive(Clone, Debug)]
enum PacketValue {
    LIST(Vec<PacketValue>),
    INT(i32)
}

impl PacketValue {
    pub fn parse (mut stream: &mut CharStream) -> PacketValue {
        let next = stream.peek().expect("Unexpected end of packet!");

        if next.is_digit(10) {
            PacketValue::INT(stream.takeUntil(",]").parse().expect("Failed to parse integer!"))
        } else {
            let mut listVec: Vec<PacketValue> = Vec::new();
            stream.require('[');
            
            let mut listNext = stream.peek().expect("Unexpected end of list!");

            while listNext != ']' {
                listVec.push(PacketValue::parse(&mut stream));
                listNext = stream.peek().expect("Unexpected end of list!");
                if listNext == ',' {
                    stream.require(',');
                    listNext = stream.peek().expect("Unexpected end of list!");
                }
            }

            stream.require(']');

            PacketValue::LIST(listVec)
        }
    }

    fn compareInt (self: &Self, intVal: i32) -> i32 {
        match self {
            Self::INT(x) => *x - intVal,
            Self::LIST(l) => -(PacketValue::LIST(vec![PacketValue::INT(intVal)]).compareList(l)),
        }
    }

    fn compareList (self: &Self, listVal: &Vec<PacketValue>) -> i32 {
        match self {
            Self::INT(x) => PacketValue::LIST(vec![PacketValue::INT(*x)]).compareList(listVal),
            Self::LIST(l) => {
                let mut index: usize = 0;
                while index < l.len() && index < listVal.len() {
                    let cmp = l.get(index).unwrap().compare(listVal.get(index).unwrap());
                    if cmp != 0 {
                        return cmp;
                    }
                    index += 1;
                }

                if l.len() != listVal.len() {
                    return l.len() as i32 - listVal.len() as i32;
                }

                0
            }
        }
    }

    pub fn compare (self: &Self, val: &PacketValue) -> i32 {
        match self {
            Self::INT(x) => -val.compareInt(*x),
            Self::LIST(l) => -val.compareList(l),
        }
    }
}

#[derive(Debug)]
struct TagPacket {
    packet: PacketValue,
    tag: bool
}

impl TagPacket {
    pub fn new (packet: PacketValue, tag: bool) -> TagPacket {
        TagPacket { 
            packet, 
            tag 
        }
    }

    pub fn compare (self: &Self, val: &TagPacket) -> i32 {
        self.packet.compare(&val.packet)
    }

    pub fn isTagged (self: &Self) -> bool {
        self.tag
    }
}

impl Ord for TagPacket {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cmpVal = self.compare(other);

        if cmpVal < 0 {
            std::cmp::Ordering::Less
        } else if cmpVal == 0 {
            std::cmp::Ordering::Equal
        } else {
            std::cmp::Ordering::Greater
        }
    }
}

impl Eq for TagPacket {

}

impl PartialOrd for TagPacket {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TagPacket {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

pub fn solve (file: BufReader<File>) {
    let packets: Vec<PacketValue> = file.lines()
        .filter_map(Result::ok)
        .filter(|s| s.trim().len() != 0)
        .map(|l| CharStream::new(&l))
        .map(|mut s| PacketValue::parse(&mut s))
        .collect();
    let packetPairs: Vec<(&PacketValue, &PacketValue)> = packets.iter()
        .tuples()
        .collect();

    let mut indexSum = 0;

    for i in packetPairs.iter().enumerate() {
        if i.1.0.compare(&i.1.1) <= 0 {
            indexSum += i.0 + 1;
        }
    }

    println!("Right order index sum: {}", indexSum);

    let mut taggedPackets: Vec<TagPacket> = packets.iter()
        .map(|p| TagPacket::new(p.clone(), false))
        .collect();

    taggedPackets.push(TagPacket::new(PacketValue::parse(&mut CharStream::new("[[2]]")), true));
    taggedPackets.push(TagPacket::new(PacketValue::parse(&mut CharStream::new("[[6]]")), true));

    let decodeKey = taggedPackets.iter()
        .sorted()
        .enumerate()
        .filter(|e| e.1.isTagged())
        .map(|e| e.0)
        .map(|i| i + 1)
        .reduce(|a, b| a * b)
        .unwrap();

    println!("Decode key: {}", decodeKey);
}