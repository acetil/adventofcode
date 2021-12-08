class Line:
    def __init__ (self, l: str):
        points = l.split(" -> ")
        self.start = tuple(int(i) for i in points[0].split(","))
        self.end = tuple(int(i) for i in points[1].split(","))

    @classmethod
    def fromPoints (cls, start, end):
        return cls(f"{start[0]},{start[1]} -> {end[0]},{end[1]}")

    def flip (self):
        return Line.fromPoints((self.start[1], self.start[0]), (self.end[1], self.end[0]))

    def isHorizontal (self):
        return self.start[1] == self.end[1]
    
    def isVertical (self):
        return self.start[0] == self.end[0]

    def isHorizontalVertical (self):
        return self.isHorizontal() or self.isVertical()

    def getPointsHorizVertical (self):
        if not self.isHorizontalVertical():
            return []
        
        if self.isVertical():
            return [(i[1], i[0]) for i in self.flip().getPointsHorizVertical()]
        elif self.start[0] > self.end[0]:
            return Line.fromPoints(self.end, self.start).getPointsHorizVertical()
        else:
            return [(i, self.start[1]) for i in range(self.start[0], self.end[0] + 1)]

    def getPoints (self):
        vec = ((self.end[0] - self.start[0]) / (abs(self.end[0] - self.start[0]) if self.start[0] != self.end[0] else 1), \
            (self.end[1] - self.start[1]) / (abs(self.end[1] - self.start[1]) if self.start[1] != self.end[1] else 1))

        curr = self.start
        points = []

        while curr[0] != self.end[0] or curr[1] != self.end[1]:
            points.append(curr)
            curr = (curr[0] + vec[0], curr[1] + vec[1])

        points.append(curr)

        return points


f = open("data/day05.txt")

lines = [Line(i) for i in f.readlines() if i.strip() != ""]

horizVerticalPoints = [j for i in lines for j in i.getPointsHorizVertical()]

points = {}
for i in horizVerticalPoints:
    points[i] = points.get(i, 0) + 1

points2L = [j for i in lines for j in i.getPoints()]


points2 = {}
for i in points2L:
    points2[i] = points2.get(i, 0) + 1

print(f"Overlapping horizontal vertical points: {sum(1 if points[i] >= 2 else 0 for i in points)}")
print(f"Overlapping all points: {sum(points2[i] >= 2 for i in points2)}")