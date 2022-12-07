use std::{io::{BufReader, BufRead}, fs::File, cell::{RefCell}};

enum DirTreeCommand {
    CdUp,
    Cd(String),
    NewDir(String),
    NewFile(i64)
}

struct Directory {
    name: String,
    filesSize: i64,
    totalSize: RefCell<i64>,
    childDirs: Vec<Directory>
}

impl DirTreeCommand {
    pub fn parse (line: &str) -> Option<DirTreeCommand> {
        let splitLine: Vec<&str> = line.split(" ").collect();

        match *splitLine.get(0)? {
            "$" => {
                if *splitLine.get(1)? == "cd" {
                    if *splitLine.get(2)? == ".." {
                        Some(Self::CdUp)
                    } else {
                        Some(Self::Cd(splitLine.get(2)?.to_string()))
                    }
                } else {
                    None
                }
            },
            "dir" => Some(DirTreeCommand::NewDir(splitLine.get(1)?.to_string())),
            _ => Some(DirTreeCommand::NewFile(splitLine.get(0)?.parse().ok()?))
        }
    }
}

impl Directory {
    pub fn new (name: String) -> Directory {
        Directory { 
            name: name, 
            filesSize: 0, 
            totalSize: RefCell::new(-1), 
            childDirs: Vec::new() 
        }
    }

    pub fn addChildren<T> (self: &mut Directory, commands: &mut T) where T: Iterator<Item = DirTreeCommand> {
        let mut curr: Option<DirTreeCommand> = commands.next();

        while curr.is_some() {
            match curr.unwrap() {
                DirTreeCommand::CdUp => break,
                DirTreeCommand::Cd(dirName) => self.childDirs.iter_mut().filter(|d| d.name == dirName).next().unwrap().addChildren(commands),
                DirTreeCommand::NewFile(size) => self.filesSize += size,
                DirTreeCommand::NewDir(dirName) => self.childDirs.push(Directory::new(dirName))
            }
            
            curr = commands.next();
        }
    }

    pub fn getSize (self: &Directory) -> i64 {
        if *self.totalSize.borrow() == -1 {
            *self.totalSize.borrow_mut() = self.filesSize + self.childDirs.iter().map(Directory::getSize).sum::<i64>();
        }

        *self.totalSize.borrow()
    }

    pub fn totalDirectorySizeUnder (self: &Directory, maxSize: i64) -> i64 {
        let childTotal: i64 = self.childDirs.iter().map(|d| d.totalDirectorySizeUnder(maxSize)).sum();

        if self.getSize() <= maxSize {
            childTotal + self.getSize()
        } else {
            childTotal
        }
    }

    pub fn minimumDeletionSize (self: &Directory, requiredSize: i64) -> Option<i64> {
        let childDeletionSize: Option<i64> = self.childDirs.iter()
            .filter_map(|d| d.minimumDeletionSize(requiredSize))
            .min();

        let selfDeletionSize = if self.getSize() >= requiredSize {
                Some(self.getSize())
            } else {
                None
            };
        
        return childDeletionSize.map(|childSize| {
                selfDeletionSize.map(|selfSize| std::cmp::min(childSize, selfSize))
                    .unwrap_or(childSize)
            })
            .or(selfDeletionSize);
    }
}

pub fn solve (file: BufReader<File>) {
    let mut root = Directory::new("/".to_string());

    root.addChildren(&mut file.lines()
        .filter_map(Result::ok)
        .skip(1)
        .filter_map(|s| DirTreeCommand::parse(s.as_str())));

    println!("Directory size under 100000: {}", root.totalDirectorySizeUnder(100000));
    println!("Minimum deletion: {}", root.minimumDeletionSize(30000000 - (70000000 - root.getSize()))
        .expect("Failed to get minimum deletion!"));
}