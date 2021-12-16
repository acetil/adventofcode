import math
import time

class Heap:
    def __init__ (self, key=lambda x: x):
        self.key = key
        self.element = None
        self.left = None
        self.right = None

    def updateHeap (self):
        if self.left.element != None and self.key(self.left.element) < self.key(self.element):
            temp = self.element
            self.element = self.left.element
            self.left.element = temp
        if self.right.element != None and self.key(self.right.element) < self.key(self.element):
            temp = self.element
            self.element = self.right.element
            self.right.element = temp

    def addElement (self, element):
        if self.element == None:
            self.element = element
            self.left = Heap(self.key)
            self.right = Heap(self.key)
            return
    
        if self.left.getSize() <= self.right.getSize():
            self.left.addElement(element)
        else:
            self.right.addElement(element)
        
        self.updateHeap()
    

    def pop (self):
        if self.element == None:
            return None
        
        val = self.element
        self.element = None

        if self.left.element == None and self.right.element == None:
            return val

        if self.left.element == None and self.right.element != None:
            self.element = self.right.pop()
        elif self.right.element == None and self.left.element != None:
            self.element = self.left.pop()
        elif self.key(self.left.element) < self.key(self.right.element):
            self.element = self.left.pop()
        else:
            self.element = self.right.pop()
        
        return val

    def updateElement (self, old, new):
        if self.element == None:
            return
        
        if self.element == old:
            self.element = new
        else:
            if self.left != None:
                self.left.updateElement(old, new)
            if self.right != None:
                self.right.updateElement(old, new)
            
        self.updateHeap()

    def getSize (self):
        if not self.element:
            return 0
        if not self.left and not self.right:
            return 1
        
        return 1 + max(self.left.getSize(), self.right.getSize())

    def isEmpty (self):
        return self.element == None

class PriorityQueue:
    def __init__ (self):
        self.heap = Heap(lambda x: x[0])
        self.vals = {}
    
    def addElement (self, element, priority):
        if element in self.vals:
            self.updateElement(element, priority)
            return
        
        self.heap.addElement((priority, element))
        self.vals[element] = (priority, element)
    
    def updateElement (self, element, priority):
        self.heap.updateElement(self.vals[element], (priority, element))
        self.vals[element] = (priority, element)
    
    def pop (self):
        val = self.heap.pop()[1]
        self.vals.pop(val)
        return val
    
    def isEmpty (self):
        return self.heap.isEmpty()


def heuristic (curr, end):
    return abs(curr[0] - end[0]) + abs(curr[1] - end[1])

def getNeighbours (pos, width, height):
    neighbours = []
    if pos[0] > 0:
        neighbours.append((pos[0] - 1, pos[1]))
    if pos[0] < width - 1:
        neighbours.append((pos[0] + 1, pos[1]))
    if pos[1] > 0:
        neighbours.append((pos[0], pos[1] - 1))
    if pos[1] < height - 1:
        neighbours.append((pos[0], pos[1] + 1))
    
    return neighbours

def findLeastCost (costs, end):
    origins = [[None for j in range(len(i))] for i in costs]

    routeCost = [[math.inf for j in range(len(i))] for i in costs]

    routeCost[0][0] = 0

    heurCost = [[math.inf for j in range(len(i))] for i in costs]
    heurCost[0][0] = routeCost[0][0] + heuristic((0, 0), end)

    queue = PriorityQueue()
    queue.addElement((0, 0), heurCost[0][0])

    while not queue.isEmpty():
        curr = queue.pop()

        if curr == end:
            break

        for i in getNeighbours(curr, len(costs[0]), len(costs)):
            score = routeCost[curr[1]][curr[0]] + costs[i[1]][i[0]]

            if score < routeCost[i[1]][i[0]]:
                origins[i[1]][i[0]] = curr
                routeCost[i[1]][i[0]] = score
                heurCost[i[1]][i[0]] = score + heuristic(i, end)

                queue.addElement(i, heurCost[i[1]][i[0]])
    
    return routeCost[end[1]][end[0]]

def getWrappedCost (cost):
    return (cost - 1) % 9 + 1

f = open("data/day15.txt")

costs = [[int(j) for j in i.strip()] for i in f.readlines() if i != ""]


width = len(costs[0])
height = len(costs)

print(f"Route cost simple: {findLeastCost(costs, (width - 1, height - 1))}")


#print("\n".join("".join(str(getWrappedCost(costs[i % height][j % width] + j // width + i // height)) for j in range(0, width * 5)) for i in range(0, height * 5)))
start = time.time()
print(f"Route cost complex: {findLeastCost([[getWrappedCost(costs[i % height][j % width] + j // width + i // height) for j in range(0, width * 5)] for i in range(0, height * 5)], (width * 5 - 1, height * 5 - 1))}")
end = time.time()

print(f"Time taken: {end - start} s")