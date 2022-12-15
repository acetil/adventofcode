use std::{io::{BufReader, BufRead}, fs::File, cell::Cell};

#[derive(Debug)]
#[derive(Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Debug)]
struct DirectedEdge {
    toNode: usize,
    dir: Direction,
    visibleSize: Cell<Option<i32>>
}

#[derive(Debug)]
struct Node {
    size: i32,
    index: usize,

    leftEdge: Option<DirectedEdge>,
    rightEdge: Option<DirectedEdge>,
    upEdge: Option<DirectedEdge>,
    downEdge: Option<DirectedEdge>
}

#[derive(Debug)]
struct NodeGraph {
    nodes: Vec<Node>,
    width: usize,
    height: usize
}

impl DirectedEdge {
    pub fn new (toNode: usize, dir: Direction) -> DirectedEdge {
        DirectedEdge { 
            toNode, 
            dir,
            visibleSize: Cell::new(None) 
        }
    }

    fn calculateMaxVisibleSize (self: &DirectedEdge, graph: &NodeGraph) -> i32 {
        let nextNode = graph.getNode(self.toNode);

        let nextEdge = nextNode.getEdge(self.dir);

        let visibleSize = std::cmp::max(nextEdge.map(|e| e.getMaxVisibleSize(graph)).unwrap_or(-1), nextNode.size);

        self.visibleSize.replace(Some(visibleSize));

        visibleSize
    }

    pub fn getMaxVisibleSize (self: &DirectedEdge, graph: &NodeGraph) -> i32 {
        self.visibleSize.get().unwrap_or_else(|| self.calculateMaxVisibleSize(graph))
    }

    pub fn getViewingDistance (self: &DirectedEdge, graph: &NodeGraph, maxSize: i32) -> usize {
        let nextNode = graph.getNode(self.toNode);

        if nextNode.size >= maxSize {
            1
        } else {
            nextNode.getEdge(self.dir)
                .map(|e| e.getViewingDistance(graph, maxSize))
                .unwrap_or(0) + 1
        }
    }

}

impl Node {
    pub fn new (index: usize, size: i32) -> Node {
        Node { 
            size, 
            index, 
            leftEdge: None, 
            rightEdge: None, 
            upEdge: None, 
            downEdge: None 
        }
    }

    pub fn setEdges (self: &mut Node, leftEdge: Option<DirectedEdge>, rightEdge: Option<DirectedEdge>, upEdge: Option<DirectedEdge>, downEdge: Option<DirectedEdge>) {
        self.leftEdge = leftEdge;
        self.rightEdge = rightEdge;
        self.upEdge = upEdge;
        self.downEdge = downEdge;
    }

    pub fn getEdge (self: &Node, dir: Direction) -> Option<&DirectedEdge> {
        match dir {
            Direction::LEFT => self.leftEdge.as_ref(),
            Direction::RIGHT => self.rightEdge.as_ref(),
            Direction::UP => self.upEdge.as_ref(),
            Direction::DOWN => self.downEdge.as_ref(),
        }
    }

    fn isVisibleDir (self: &Node, graph: &NodeGraph, dir: Direction) -> bool {
        self.getEdge(dir)
            .map(|e| e.getMaxVisibleSize(graph))
            .map(|s| s < self.size)
            .unwrap_or(true)
    }

    fn getViewingDistance (self: &Node, graph: &NodeGraph, dir: Direction) -> usize {
        self.getEdge(dir)
            .map(|e| e.getViewingDistance(graph, self.size))
            .unwrap_or(0)
    }

    pub fn isVisible (self: &Node, graph: &NodeGraph) -> bool {
        self.isVisibleDir(graph, Direction::LEFT) 
            || self.isVisibleDir(graph, Direction::RIGHT) 
            || self.isVisibleDir(graph, Direction::UP) 
            || self.isVisibleDir(graph, Direction::DOWN)
    }

    pub fn getScore (self: &Node, graph: &NodeGraph) -> usize {
        self.getViewingDistance(graph, Direction::LEFT) 
            * self.getViewingDistance(graph, Direction::RIGHT) 
            * self.getViewingDistance(graph, Direction::UP) 
            * self.getViewingDistance(graph, Direction::DOWN)
    }
}

impl NodeGraph {
    pub fn new<T> (lines: T) -> NodeGraph where T: Iterator<Item = String> {
        let mut nodes = Vec::<Node>::new();

        let mut width: usize = 0;
        let mut height: usize = 0;

        for l in lines {
            let mut lineSize: usize = 0;
            for c in l.chars() {
                nodes.push(Node::new(nodes.len(), c.to_digit(10).unwrap() as i32));
                lineSize += 1;
            }

            width = std::cmp::max(width, lineSize);
            height += 1;
        }

        let mut graph = NodeGraph {
            nodes,
            width,
            height
        };

        graph.setupConnections();

        graph
    }


    fn setupConnections (self: &mut NodeGraph) {
        for x in 0..self.width as isize {
            for y in 0..self.height as isize {
                let nodeIndex = self.getIndexFromPos((x, y)).unwrap();
                let leftEdge = self.getIndexFromPos((x - 1, y)).map(|i| DirectedEdge::new(i, Direction::LEFT));
                let rightEdge = self.getIndexFromPos((x + 1, y)).map(|i| DirectedEdge::new(i, Direction::RIGHT));
                let upEdge = self.getIndexFromPos((x, y - 1)).map(|i| DirectedEdge::new(i, Direction::UP));
                let downEdge = self.getIndexFromPos((x, y + 1)).map(|i| DirectedEdge::new(i, Direction::DOWN));

                let node = self.nodes.get_mut(nodeIndex).unwrap();

                node.setEdges(leftEdge, rightEdge, upEdge, downEdge);
            }
        }
    }

    fn getIndexFromPos (self: &NodeGraph, pos: (isize, isize)) -> Option<usize> {
        if pos.0 >= 0 && (pos.0 as usize) < self.width && pos.1 >= 0 && (pos.1 as usize) < self.height {
            Some((pos.1 as usize) * self.width + (pos.0 as usize))
        } else {
            None
        }
    }

    pub fn getNode (self: &NodeGraph, index: usize) -> &Node {
        self.nodes.get(index).unwrap()
    }

    pub fn getNumNodes (self: &NodeGraph) -> usize {
        self.nodes.len()
    }
}

pub fn solve (file: BufReader<File>) {
    let graph = NodeGraph::new(file.lines().filter_map(Result::ok));

    let numVisible = (0..graph.getNumNodes())
        .map(|i| graph.getNode(i))
        .filter(|n| n.isVisible(&graph))
        .count();
    
    println!("Visible trees: {numVisible}");

    let maxScore = (0..graph.getNumNodes())
        .map(|i| graph.getNode(i))
        .map(|n| n.getScore(&graph))
        .max()
        .unwrap();
    
    println!("Max viewing score: {maxScore}");
}