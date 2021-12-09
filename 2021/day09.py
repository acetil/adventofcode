class Point:
    def __init__ (self, height):
        self.height = height
        self.sinks = []
        self.sources = []
        self.equal = []
    
    def addNeighbour (self, neighbour):
        if neighbour.height <= self.height:
            self.sinks.append(neighbour)
        else:
            self.sources.append(neighbour)
    
    def isLowPoint (self):
        return len(self.sinks) == 0 and len(self.equal) == 0
    
    def basinSize (self, visited = None):
        if not visited:
            visited = []

        if self.height == 9 or self in visited:
            return 0
        
        visited.append(self)

        return sum(i.basinSize(visited) for i in self.sources) + 1

f = open("data/day09.txt")

l = [i.strip() for i in f.readlines() if i.strip() != ""]

width = len(l[0].strip())
height = len(l)

points = [[Point(int(j)) for j in i] for i in l]

for y, line in enumerate(points):
    for x, point in enumerate(line):
        if x != 0:
            point.addNeighbour(points[y][x - 1])
        if x != width - 1:
            point.addNeighbour(points[y][x + 1])
        if y != 0:
            point.addNeighbour(points[y - 1][x])
        if y != height - 1:
            point.addNeighbour(points[y + 1][x])
            
lowPoints = [j for i in points for j in i if j.isLowPoint()]

print(f"Risk levels: {sum(i.height + 1 for i in lowPoints)}")

basinSizes = sorted([i.basinSize() for i in lowPoints], reverse=True)

print(f"Basin multiple: {basinSizes[0] * basinSizes[1] * basinSizes[2]}")