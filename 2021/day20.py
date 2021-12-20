class Cell:
    def __init__ (self, state: bool, next: bool = None):
        self.state = state
        self.next = state if next is None else next
    def updateNext (self, pos, map: "Map", rule):
        val = 0
        for y in range(-1, 2):
            for x in range(-1, 2):
                val = val << 1 | bool(map.getState(pos[0] + x, pos[1] + y))
        
        self.next = rule[val]

    def swapStates (self):
        self.state = self.next

    def __bool__ (self):
        return self.state
    
class Map:
    def __init__ (self, initialMap):
        self.states = [[Cell(j) for j in i] for i in initialMap]

        self.topleft = (0, 0)
        self.bottomright = (len(self.states[0]), len(self.states))

        self.currDefault = False
        self.nextDefault = False

        self.addCol(True)
        self.addCol(False)
        self.addRow(True)
        self.addRow(False)

    def addCol (self, left):
        for i in self.states:
            i.insert(0 if left else len(i), Cell(self.currDefault, self.nextDefault))

        if left:
            self.topleft = (self.topleft[0] - 1, self.topleft[1])
        else:
            self.bottomright = (self.bottomright[0] + 1, self.bottomright[1])
    
    def addRow (self, top):
        self.states.insert(0 if top else len(self.states), [Cell(self.currDefault, self.nextDefault) for _ in self.states[0]])
    
        if top:
            self.topleft = (self.topleft[0], self.topleft[1] - 1)
        else:
            self.bottomright = (self.bottomright[0], self.bottomright[1] + 1)
    
    def getState (self, x, y):
        while x < self.topleft[0]:
            self.addCol(True)
        while x >= self.bottomright[0]:
            self.addCol(False)
        while y < self.topleft[1]:
            self.addRow(True)
        while y >= self.bottomright[1]:
            self.addRow(False)

        return self.states[y - self.topleft[1]][x - self.topleft[0]]
    
    def updateCells (self, rule):
        self.nextDefault = rule[-1] if self.currDefault else rule[0]

        fromPos = self.topleft
        toPos = self.bottomright
        for y in range(fromPos[1], toPos[1]):
            for x in range(fromPos[0], toPos[0]):
                self.getState(x, y).updateNext((x, y), self, rule)
        
        for i in self.states:
            for j in i:
                j.swapStates()
        
        self.currDefault = self.nextDefault

    def getLit (self):
        return sum(sum(j.state for j in i) for i in self.states)
    
    def __str__ (self):
        return "\n".join("".join("#" if j else "." for j in i) for i in self.states)
    
    def __repr__ (self):
        return str(self)

f = open("data/day20.txt")

rule = [i == "#" for i in f.readline().strip()]

f.readline()

map = Map([[j == "#" for j in i.strip()] for i in f.readlines() if i.strip() != ""])

for i in range(0, 50):
    map.updateCells(rule)

    if i == 1:
        print(f"Lit pixels after 2: {map.getLit()}")

print(f"Lit pixels: {map.getLit()}")
    
    
