class Tile:
    def __init__ (self, idNum, data):
        self.id = idNum
        self.data = data[:]
        self.adjTiles = {"right" : None, "up" : None, "left" : None, "down" : None}
        self.edgeFlip = {"right" : False, "up" : False, "left" : False, "down" : False}
        self.numAdj = 0
        self.explored = False
    def getAdjacentTiles (self, tiles):
        for i in tiles:
            if i == self:
                continue
            for j in self.adjTiles:
                #print(j)
                sEdge = getEdge(self.data, j)
                #print(sEdge)
                for k in i.adjTiles:
                    oEdge = getEdge(i.data, k)
                    if sEdge == oEdge:
                        self.adjTiles[j] = i
                        self.numAdj += 1
                    elif sEdge[::-1] == oEdge:
                        self.adjTiles[j] = i
                        self.edgeFlip[j] = True
                        self.numAdj += 1
    def isCorner (self):
        if self.numAdj != 2:
            return False
        return not (self.adjTiles["left"] and self.adjTiles["right"]) and not (self.adjTiles["up"] and self.adjTiles["down"]) 

    def rotateAnticlockwise (self):
        tmp = self.adjTiles["right"]
        self.adjTiles["right"] = self.adjTiles["down"]
        self.adjTiles["down"] = self.adjTiles["left"]
        self.adjTiles["left"] = self.adjTiles["up"]
        self.adjTiles["up"] = tmp
        tmp = self.edgeFlip["right"]
        self.edgeFlip["right"] = self.edgeFlip["down"]
        self.edgeFlip["down"] = self.edgeFlip["left"]
        self.edgeFlip["left"] = self.edgeFlip["up"]
        self.edgeFlip["up"] = tmp
        self.data = [[j[len(self.data[0]) - i] for j in self.data] for i in range(1, len(self.data[0]) + 1)]
    
    def rotateToTopLeft (self):
        '''rots = 0
        while not self.adjTiles["down"] or not self.adjTiles["right"]:
            self.rotateAnticlockwise()
            rots += 1
        if rots == 1 or rots == 2:
            for i in self.edgeFlip:
                self.edgeFlip[i] = not self.edgeFlip[i]'''
        if self.adjTiles["up"]:
            self.flip(True)
        if self.adjTiles["left"]:
            self.flip(False)

    def flip (self, isX):
        if isX:
            self.data = self.data[::-1]
            tmp = self.adjTiles["up"]
            self.adjTiles["up"] = self.adjTiles["down"]
            self.adjTiles["down"] = tmp
            tmp = self.edgeFlip["up"]
            self.edgeFlip["up"] = self.edgeFlip["down"]
            self.edgeFlip["down"] = tmp
            self.edgeFlip["left"] = not self.edgeFlip["left"]
            self.edgeFlip["right"] = not self.edgeFlip["right"]
        else:
            self.data = [i[::-1] for i in self.data]
            tmp = self.adjTiles["left"]
            self.adjTiles["left"] = self.adjTiles["right"]
            self.adjTiles["right"] = tmp
            tmp = self.edgeFlip["left"]
            self.edgeFlip["left"] = self.edgeFlip["right"]
            self.edgeFlip["right"] = tmp
            self.edgeFlip["up"] = not self.edgeFlip["up"]
            self.edgeFlip["down"] = not self.edgeFlip["down"]
    
    def buildMap (self, src = None, srcDir = "", srcFlip = False):
        #print("self = {} src = {} srcDir = {} srcFlip = {}".format(self.id, src.id if src else 0, srcDir, srcFlip))
        #print([(i, self.adjTiles[i].id if self.adjTiles[i] else 0) for i in self.adjTiles])
        #print([(i, self.edgeFlip[i]) for i in self.edgeFlip])
        if src:
            rots = 0
            origDir = ""
            for i in self.adjTiles:
                if self.adjTiles[i] == src:
                    origDir = i
                    break
            while self.adjTiles[srcDir] != src:
                self.rotateAnticlockwise()
                rots += 1
            #print("Rotate {} from {}".format(srcDir, origDir))
            if (isXAxis(origDir) and (rots == 2 or rots == 3)) or (not isXAxis(origDir) and (rots == 1 or rots == 2)):
                srcFlip = not srcFlip
                
            for i in self.edgeFlip:
                if (not isXAxis(i) and (rots == 2 or rots == 3)) or (isXAxis(i) and (rots == 1 or rots == 2)):
                    self.edgeFlip[i] = not self.edgeFlip[i]
            #print([(i, self.adjTiles[i].id if self.adjTiles[i] else 0) for i in self.adjTiles])
            #print([(i, self.edgeFlip[i]) for i in self.edgeFlip])
            if srcFlip:
                self.flip(isXAxis(srcDir))
                #print("Src flip ({})".format(srcFlip))
                #print([(i, self.adjTiles[i].id if self.adjTiles[i] else 0) for i in self.adjTiles])
                #print([(i, self.edgeFlip[i]) for i in self.edgeFlip])
    
    def printRep (self, goDown = True):
        print(self.id, end="\t")
        if self.adjTiles["right"]:
            self.adjTiles["right"].printRep(False)
        if goDown:
            print()
            if self.adjTiles["down"]:
                self.adjTiles["down"].printRep(True)

    def stitchMap (self, goDown = True):
        d = [i[1:len(i) - 1] for i in self.data[1:len(self.data) - 1]]
        #d = self.data
        if not self.adjTiles["right"]:
            return d
        other = self.adjTiles["right"].stitchMap(False)
        comb = [d[i] + other[i] for i in range(0, len(d))]
        if goDown and self.adjTiles["down"]:
            comb += self.adjTiles["down"].stitchMap(True)
        return comb

def getOpposite (d):
    dirs = ["right", "up", "left", "down"]
    return dirs[(dirs.index(d) + 2) % 4]
def getDataStr (data):
    return "\n".join(["".join([' ' if j == ' ' else ('#' if j else '.') for j in i]) for i in data])

def flipData (data, isX):
    if isX:
        return data[::-1]
    else:
        return [i[::-1] for i in data]

def rotateDataAnticlockwise (data):
    return [[j[len(data[0]) - i] for j in data] for i in range(1, len(data[0]) + 1)]

def getEdge (data, d):
    if d == "up":
        return data[0]
    elif d == "down":
        return data[-1]
    elif d == "left":
        return [i[0] for i in data]
    else:
        return [i[-1] for i in data]

def isXAxis (d):
    return d in ["left", "right"]

def checkMonster (data, monster, x, y):
    if len(data) < y + len(monster):
        return False
    for i in range(0, len(monster)):
        if len(data[y + i]) < x + len(monster[i]):
            return False
        for j in range(0, len(monster[i])):
            #print("{} {} {} {} {} {}".format(j, i, x, y, len(data), len(data[y + i])))
            if monster[i][j] == '#' and not data[y + i][x + j]:
                return False
    return True

def buildMap (startTile):
    queue = [(startTile, None, "", False)]
    startTile.explored = True
    while len(queue) > 0:
        curr = queue.pop(0)
        curr[0].buildMap(curr[1], curr[2], curr[3])
        for i in curr[0].adjTiles:
            t = curr[0].adjTiles[i]
            if t and not t.explored:
                t.explored = True
                queue.append((t, curr[0], getOpposite(i), curr[0].edgeFlip[i]))



f = open("day20.txt")

tileParts = [i.strip() for i in f.read().split("\n\n")]

tiles = []

for i in tileParts:
    lines = [j.strip() for j in i.split("\n")]
    n = lines[0].split(" ")[1]
    tiles.append(Tile(int(n[:len(n) - 1]), [[k == '#' for k in j] for j in lines[1:]]))


for i in tiles:
    i.getAdjacentTiles(tiles)

mul = 1
for i in tiles:
    #print(i.id)
    #print([(j, i.adjTiles[j].id if i.adjTiles[j] else 0) for j in i.adjTiles])

    if i.isCorner():
        mul *= i.id

print("Id multiple: {}".format(mul))

cornerStart = None
for i in tiles:
    if i.isCorner():
        cornerStart = i
        break

cornerStart.rotateToTopLeft()
buildMap(cornerStart)
#print("{} {}".format(cornerStart.id, [(i, cornerStart.adjTiles[i].id if cornerStart.adjTiles[i] else 0) for i in cornerStart.adjTiles]))
#print("{} {}".format(cornerStart.id, [(i, cornerStart.edgeFlip[i]) for i in cornerStart.adjTiles]))
#print("{} {}".format(cornerStart.adjTiles["down"].id, [(i, cornerStart.adjTiles["down"].adjTiles[i].id if cornerStart.adjTiles["down"].adjTiles[i] else 0) for i in cornerStart.adjTiles["down"].adjTiles]))

#cornerStart.printRep()

dataOrig = cornerStart.stitchMap()
data = dataOrig[:]
#print(getDataStr(dataOrig))

monster = ["                  # ", "#    ##    ##    ###", " #  #  #  #  #  #   "]
numMonsters = 0
for i in range(0, 4):
    for j in range(0, 4):
        dataFlip = data[:]
        if j % 2 == 1:
            dataFlip = flipData(dataFlip, True)
        if j // 2 == 1:
            dataFlip = flipData(dataFlip, False)
        #print("\n{}".format(getDataStr(dataFlip)))
        for y in range(0, len(dataFlip)):
            for x in range(0, len(dataFlip[y])):
                if checkMonster(dataFlip, monster, x, y):
                    numMonsters += 1
        if numMonsters > 0:
            break
    if numMonsters > 0:
        break
    data = rotateDataAnticlockwise(data)
print("Num monsters: {}".format(numMonsters))
roughness = 0
for i in dataOrig:
    for j in i:
        if j:
            roughness += 1
monsterNum = 0
for i in monster:
    for j in i:
        if j == '#':
            monsterNum += 1

roughness -= monsterNum * numMonsters

print("Water roughness: {}".format(roughness))

