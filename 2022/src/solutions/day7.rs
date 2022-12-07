use std::{io::{BufReader, BufRead}, fs::File, rc::Rc, cell::{RefCell, Ref}, borrow::{BorrowMut, Borrow}};

struct AOCFile {
    name: String,
    size: i64
}

enum AOCInode {
    File(Rc<RefCell<AOCFile>>),
    Dir(Rc<RefCell<AOCDirectory>>)
}

struct AOCDirectory {
    name: String,
    children: Vec<AOCInode>,
    size: RefCell<i64>
}

impl AOCFile {
    pub fn getSize (self: &AOCFile) -> i64 {
        self.size
    }

    pub fn new (name: String, size: i64) -> AOCFile {
        AOCFile { 
            name: name, 
            size: size 
        }
    }

    pub fn getName (self: &AOCFile) -> &String {
        &self.name
    }
}

impl AOCDirectory {
    pub fn getSize (self: &AOCDirectory) -> i64 {
        if *self.size.borrow() == -1 {
            *self.size.borrow_mut() = self.children.iter().map(AOCInode::getSize).sum();
        }

        *self.size.borrow()
    }

    pub fn new (name: String) -> AOCDirectory {
        AOCDirectory { 
            name: name, 
            children: Vec::new(),
            size: RefCell::new(-1)
        }
    }

    pub fn addFile (self: &mut AOCDirectory, file: AOCInode) {
        self.children.push(file);
    }

    pub fn getName (self: &AOCDirectory) -> &String {
        &self.name
    }

    pub fn getDir (self: &mut AOCDirectory, fileName: &str) -> Option<Rc<RefCell<AOCDirectory>>>{
        self.children.iter_mut()
            .find(|i| i.getName() == fileName)
            .and_then(|i| match i {
                AOCInode::Dir(d) => Some(d.clone()),
                _ => None
            })
    }

    pub fn getSumDirsBelow (self: &AOCDirectory, n: i64) -> i64 {
        let childDirs = self.children.iter()
            .map(|f| f.getDirsBelow(n))
            .sum();

        if self.getSize() <= n {
            childDirs + self.getSize()
        } else {
            childDirs
        }
    }

    pub fn getSmallestDeletionAmount (self: &AOCDirectory, n: i64) -> Option<i64> {
        let childSmallest: Option<i64> = self.children.iter()
            .filter_map(|i| i.getSmallestDeletionAmount(n))
            .min();

        let deletionSize: Option<i64> = if self.getSize() >= n {
                Some(self.getSize())
            } else {
                None
            };

        return childSmallest.map(|childSize| {
                deletionSize.map(|selfSize| std::cmp::min(childSize, selfSize))
                    .unwrap_or(childSize)
            })
            .or(deletionSize);
    }
}

impl AOCInode {
    pub fn getSize (self: &AOCInode) -> i64 {
        match self {
            AOCInode::File(f) => f.as_ref().borrow().getSize(),
            AOCInode::Dir(d) => d.as_ref().borrow().getSize()
        }
    }

    pub fn getName (self: &AOCInode) -> String {
        match self {
            AOCInode::File(f) => f.as_ref().borrow().getName().to_string(),
            AOCInode::Dir(d) => d.as_ref().borrow().getName().to_string()
        }
    }

    pub fn getDirsBelow (self: &AOCInode, n: i64) -> i64 {
        match self {
            AOCInode::File(_) => 0,
            AOCInode::Dir(d) => d.as_ref().borrow().getSumDirsBelow(n)
        }
    }

    pub fn getSmallestDeletionAmount (self: &AOCInode, n: i64) -> Option<i64> {
        match self {
            AOCInode::File(_) => None,
            AOCInode::Dir(d) => d.as_ref().borrow().getSmallestDeletionAmount(n)
        }
    }
}

pub fn solve (file: BufReader<File>) {
    let lines: Vec<String> = file.lines().filter_map(Result::ok).collect();

    let root: Rc<RefCell<AOCDirectory>> = Rc::new(RefCell::new(AOCDirectory::new("/".to_string())));

    let mut dirStack: Vec<Rc<RefCell<AOCDirectory>>> = Vec::new();
    dirStack.push(root.clone());

    for i in lines.iter().skip(1) {
        //println!("Line: \"{i}\"");
        let lineParts: Vec<&str> = i.split(" ").collect();

        if lineParts[0] == "$" {
            if lineParts[1] == "cd" {
                if lineParts[2] == ".." {
                    dirStack.pop();
                } else {
                    let lastDir = dirStack.last().unwrap();
                    dirStack.push(lastDir.clone().as_ref().borrow_mut().getDir(lineParts[2]).unwrap())
                }
            }

            // Do nothing for ls
        } else {
            let node: AOCInode;
            if lineParts[0] == "dir" {
                node = AOCInode::Dir(Rc::new(RefCell::new(AOCDirectory::new(lineParts[1].to_string()))));
            } else {
                node = AOCInode::File(Rc::new(RefCell::new(AOCFile::new(lineParts[1].to_string(), lineParts[0].parse().expect("Failed to parse file size!")))))
            }

            let lastDir: &Rc<RefCell<AOCDirectory>> = dirStack.last().unwrap();
            lastDir.as_ref().borrow_mut().addFile(node);
        }
    }

    println!("Directory sum: {}", root.as_ref().borrow().getSumDirsBelow(100000));

    let capacity: i64 = 70000000;
    let unusedRequired: i64 = 30000000;

    let deletionAmount: i64 = unusedRequired - (capacity - root.as_ref().borrow().getSize());

    println!("Size to delete: {}", root.as_ref().borrow().getSmallestDeletionAmount(deletionAmount).expect("Could not find directory to delete!"));
}