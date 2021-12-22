class Cuboid:
    def __init__ (self, xRange, yRange, zRange):
        self.xRange = xRange
        self.yRange = yRange
        self.zRange = zRange
    
    def __eq__ (self, other):
        return self.xRange[0] == other.xRange[0] and self.xRange[1] == other.xRange[1] and self.yRange[0] == other.yRange[0] and self.yRange[1] == other.yRange[1] \
            and self.zRange[0] == other.zRange[0] and self.zRange[1] == other.zRange[1]
        
    def isEmpty (self):
        return self.xRange[0] > self.xRange[1] or self.yRange[0] > self.yRange[1] or self.zRange[0] > self.zRange[1]
    
    def getIntersection (self, other):
        return Cuboid((max(self.xRange[0], other.xRange[0]), min(self.xRange[1], other.xRange[1])), (max(self.yRange[0], other.yRange[0]), min(self.yRange[1], other.yRange[1])),
            (max(self.zRange[0], other.zRange[0]), min(self.zRange[1], other.zRange[1])))
    
    def doesIntersect (self, other):
        return not self.getIntersection(other).isEmpty()
    
    def splitAround (self, splitRange, axis):
        l = []
        if axis == "x":
            l.append(Cuboid(splitRange, self.yRange, self.zRange))
            l.append(Cuboid((self.xRange[0], splitRange[0] - 1), self.yRange, self.zRange))
            l.append(Cuboid((splitRange[1] + 1, self.xRange[1]), self.yRange, self.zRange)) 
        
        elif axis == "y":
            l.append(Cuboid(self.xRange, splitRange, self.zRange))
            l.append(Cuboid(self.xRange, (self.yRange[0], splitRange[0] - 1), self.zRange))
            l.append(Cuboid(self.xRange, (splitRange[1] + 1, self.yRange[1]), self.zRange)) 

        else:
            l.append(Cuboid(self.xRange, self.yRange, splitRange))
            l.append(Cuboid(self.xRange, self.yRange, (self.zRange[0], splitRange[0] - 1)))
            l.append(Cuboid(self.xRange, self.yRange, (splitRange[1] + 1, self.zRange[1]))) 
        
        return [i for i in l if not i.isEmpty()]

    def removeIntersecting (self, other):
        if not self.doesIntersect(other):
            return [self]
        
        intersection = self.getIntersection(other)

        if self.xRange[0] != intersection.xRange[0] or self.xRange[1] != intersection.xRange[1]:
            l = self.splitAround(intersection.xRange, "x")
            return l[1:] + l[0].removeIntersecting(other)
        elif self.yRange[0] != intersection.yRange[0] or self.yRange[1] != intersection.yRange[1]:
            l = self.splitAround(intersection.yRange, "y")
            return l[1:] + l[0].removeIntersecting(other)

        else:
            return self.splitAround(intersection.zRange, "z")[1:]
    
    def getSize (self):
        if self.isEmpty():
            return 0
            
        return (self.xRange[1] - self.xRange[0] + 1) * (self.yRange[1] - self.yRange[0] + 1) * (self.zRange[1] - self.zRange[0] + 1)

f = open("data/day22.txt")

instructions = [([tuple(int(k) for k in j[2:].split("..")) for j in i.strip().split(" ")[1].split(",")], i.split(" ")[0].strip() == "on") for i in f.readlines() if i.strip() != ""]

cuboids = []

for i in instructions:
    cuboid = Cuboid(i[0][0], i[0][1], i[0][2])
    
    if i[1]:
        newCuboids = [cuboid]

        for j in cuboids:
            newCuboids = [l for k in newCuboids for l in k.removeIntersecting(j)]
        
        cuboids += newCuboids
    
    if not i[1]:
        newCuboids = []
        for j in cuboids:
            newCuboids += j.removeIntersecting(cuboid)

        cuboids = newCuboids

print(f"Init region: {sum(Cuboid((-50, 50), (-50, 50), (-50, 50)).getIntersection(i).getSize() for i in cuboids)}")
print(f"Total size: {sum(i.getSize() for i in cuboids)}")
