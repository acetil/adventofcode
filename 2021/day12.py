from typing import Dict


class Node:
    def __init__ (self, name):
        self.name = name
        self.maxVisits = -1
        self.start = False
        self.big = False
        self.special = False
        if name == "start" or name == "end":
            self.start = name == "start"
            self.special = True
        elif name.isupper():
            self.big = True

        self.connections = []

    def addConnection (self, other):
        self.connections.append(other)

    def getPaths (self, part1=True, visited=None):
        if not visited:
            visited = []
        
        if self.start:
            return [[self]] if self not in visited else []
        elif self in visited and (part1 or self.special \
            or len([i for i in visited]) != len(set(i for i in visited))):
            return []
        else:
            if not self.big:
                visited.append(self)
            
            return [j + [self] for i in self.connections for j in i.getPaths(part1, visited[:])]


def getNode (nodes, key):
    if key not in nodes:
        nodes[key] = Node(key)
    
    return nodes[key]

f = open("data/day12.txt")

nodes: Dict[str, Node] = {}

for i in f.readlines():
    if i.strip() != "":
        l = i.strip().split("-")
        getNode(nodes, l[0]).addConnection(getNode(nodes, l[1]))
        getNode(nodes, l[1]).addConnection(getNode(nodes, l[0]))

print(f"Num routes part 1: {len(getNode(nodes, 'end').getPaths())}")

print(f"Num results part 2: {len(getNode(nodes, 'end').getPaths(False))}")
