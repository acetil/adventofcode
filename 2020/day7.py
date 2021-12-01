class Bag:
    def __init__ (self, colour):
        self.colour = colour
        self.contains = []
        self.gatheredContains = False
        self.totalContains = []
    def addContains (self, colour, n):
        self.contains.append((colour, n))
    def print (self):
        print("{}: ".format(self.colour), end="")
        for i in self.contains:
            print("{} ".format(i), end="")
        print("")
    def gatherContains (self, bagMap):
        if self.gatheredContains:
            return
        for c, n in self.contains:
            self.totalContains.append((c, n))
            b = bagMap[c]
            if b == None:
                continue
            if not b.gatheredContains:
                b.gatherContains(bagMap)
            self.totalContains += [(i[0], i[1] * n) for i in b.totalContains]
        self.gatheredContains = True
    def doesContain (self, colour):
        return colour in [i[0] for i in self.totalContains]
    def numContained (self):
        return sum([i[1] for i in self.totalContains])

f = open("day7.txt")
bags = []

for l in f.readlines():
    words = l.strip().split(" ")
    b = Bag(" ".join(words[0:2]))
    if words[4] != "no":
        curr = 4
        more = True
        while more:
            n = int(words[curr])
            name = " ".join(words[curr + 1:curr + 3])
            b.addContains(name, n)
            if words[curr + 3][-1] == ".":
                more = False
            else:
                curr += 4
    bags.append(b)

bagMap = dict()

for i in bags:
    bagMap[i.colour] = i

for i in bags:
    i.gatherContains(bagMap)

print("Contains shiny gold: {}".format(sum([1 for i in bags if i.doesContain("shiny gold")])))

print("Num contained: {}".format(bagMap["shiny gold"].numContained()))


        