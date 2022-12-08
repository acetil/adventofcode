use std::{io::{BufReader, BufRead}, fs::File, cell::{Cell}};

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Debug)]
struct Tree {
    size: i32,
    index: usize,
    
    leftNode: Option<usize>,
    rightNode: Option<usize>,
    upNode: Option<usize>,
    downNode: Option<usize>,
    
    leftVisible: Cell<Option<(i32, i32)>>,
    rightVisible: Cell<Option<(i32, i32)>>,
    upVisible: Cell<Option<(i32, i32)>>,
    downVisible: Cell<Option<(i32, i32)>>
}

struct Graph {
    trees: Vec<Box<Tree>>,
    width: isize,
    height: isize,
}
impl Tree {
    pub fn new (size: i32, index: usize) -> Tree {
        Tree { 
            size, 
            index,
            leftNode: None, 
            rightNode: None, 
            upNode: None, 
            downNode: None, 
            leftVisible: Cell::new(None), 
            rightVisible: Cell::new(None), 
            upVisible: Cell::new(None), 
            downVisible: Cell::new(None) 
        }
    }

    pub fn addAdjacent (self: &mut Tree, leftNode: Option<usize>, rightNode: Option<usize>, upNode: Option<usize>, downNode: Option<usize>) {
        self.leftNode = leftNode;
        self.rightNode = rightNode;
        self.upNode = upNode;
        self.downNode = downNode;
    }

    fn getNext (oldScore: (i32, i32), newScore: (i32, i32)) -> (i32, i32) {
        if oldScore.0 > newScore.0 {
            (oldScore.0, oldScore.1 + 1)
        } else {
            (newScore.0, newScore.1 + 1)
        }
    }

    fn calculateVisibleSize (self: &Tree, graph: &Graph, direction: Direction, node: &Option<usize>, visible: &Cell<Option<(i32, i32)>>) -> (i32, i32) {
        let val = node.and_then(|n| graph.getTree(n)).map(|t| Self::getNext(t.getVisibleSizeDir(graph, direction), (t.getSize(), 0))).unwrap_or((-1, 0));

        visible.replace(Some(val));

        val
    }

    /*fn calculateVisibleDir (self: &Tree, graph: &Graph, direction: Direction, node: &Option<usize>, visible: &Cell<Option<i32>>) -> bool {
        visible.get()
            .unwrap_or_else(|| {
                let result: bool = node.and_then(|t| graph.getTree(t))
                    .map(|t| t.isVisibleDir(graph, direction) && t.size < self.size)
                    .unwrap_or(true);

                visible.replace(Some(result));

                result
            })
        
        visible.replace(val)
    }*/

    pub fn getVisibleSizeDir (self: &Tree, graph: &Graph, direction: Direction) -> (i32, i32) {
        match direction {
            Direction::LEFT => self.leftVisible.get().unwrap_or_else(|| self.calculateVisibleSize(graph, direction, &self.leftNode, &self.leftVisible)),
            Direction::RIGHT => self.rightVisible.get().unwrap_or_else(|| self.calculateVisibleSize(graph, direction, &self.rightNode, &self.rightVisible)),
            Direction::UP => self.upVisible.get().unwrap_or_else(|| self.calculateVisibleSize(graph, direction, &self.upNode, &self.upVisible)),
            Direction::DOWN => self.downVisible.get().unwrap_or_else(|| self.calculateVisibleSize(graph, direction, &self.downNode, &self.downVisible)),
        }
    }

    pub fn isVisibleDir (self: &Tree, graph: &Graph, direction: Direction) -> bool {
        self.getVisibleSizeDir(graph, direction).0 < self.size
    }

    pub fn isVisible (self: &Tree, graph: &Graph) -> bool {
        self.isVisibleDir(graph, Direction::LEFT) || self.isVisibleDir(graph, Direction::RIGHT) 
            || self.isVisibleDir(graph, Direction::UP) || self.isVisibleDir(graph, Direction::DOWN)
    }

    pub fn getScoreDir (self: &Tree, graph: &Graph, direction: Direction) -> i32 {
        dbg!((self.index, &direction));
        let x = self.getVisibleSizeDir(graph, direction);
        dbg!(x);
        x.1
    }

    pub fn calculateScore (self: &Tree, graph: &Graph) -> i32 {
        return self.getScoreDir(graph, Direction::LEFT) * self.getScoreDir(graph, Direction::RIGHT) 
                * self.getScoreDir(graph, Direction::UP) * self.getScoreDir(graph, Direction::DOWN)
    }

    pub fn getIndex (self: &Tree) -> usize {
        return self.index
    }

    pub fn getSize (self: &Tree) -> i32 {
        return self.size
    }
}

impl Graph {
    pub fn new<T> (lines: T) -> Graph where T: Iterator<Item = String> {
        let mut graph: Graph = Graph { 
            trees: Vec::new(),
            width: 0,
            height: 0 
        };

        for l in lines {
            let mut currXSize: isize = 0;
            for c in l.chars() {
                graph.trees.push(Box::new(Tree::new(c.to_digit(10).unwrap().try_into().unwrap(), graph.trees.len())));

                currXSize += 1;
            }

            if graph.width == 0 {
                graph.width = currXSize;
            }


            graph.height += 1;
        }

        for y in 0..graph.height {
            for x in 0..graph.width {
                let index = graph.getTreeIndex(x, y).unwrap();
                let leftNode = graph.getTreeIndex(x - 1, y);
                let rightNode = graph.getTreeIndex(x + 1, y);
                let upNode = graph.getTreeIndex(x, y - 1);
                let downNode = graph.getTreeIndex(x, y + 1);

                let tree: &mut Tree = graph.trees.get_mut(index).unwrap();

                tree.addAdjacent(leftNode, rightNode, upNode, downNode);
            }
        }

        //dbg!(&graph.trees);

        return graph
    }

    pub fn getTree<'a> (self: &'a Graph, index: usize) -> Option<&'a Tree> {
        self.trees.get(index).map(|b| b.as_ref())
    }

    pub fn getTreeIndex (self: &Graph, x: isize, y: isize) -> Option<usize> {
        if 0 <= x && x < self.width && 0 <= y && y < self.height {
            self.trees.get((y * self.width + x) as usize).map(|t| t.getIndex())
        } else {
            None
        }
    }

    pub fn getNumTrees (self: &Graph) -> usize {
        self.trees.len()
    }
}



pub fn solve (file: BufReader<File>) {
    let graph = Graph::new(file.lines().filter_map(Result::ok));

    let numVisible = (0..graph.getNumTrees()).filter_map(|i| graph.getTree(i)).filter(|t| t.isVisible(&graph)).count();

    println!("Number of trees visible: {}", numVisible);

    let maxScore = (0..graph.getNumTrees()).filter_map(|i| graph.getTree(i)).map(|t| (t.getIndex(), t.calculateScore(&graph))).map(|p| {
        dbg!(&p);
        p.1
    }).max().unwrap();

    println!("Maximum score: {}", maxScore);
}