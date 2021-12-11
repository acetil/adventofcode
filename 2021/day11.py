class Octopus:
    def __init__ (self, energy):
        self.neighbours = []
        self.energy = energy
    
        self.flashed = False
    
    def addNeighbour (self, neighbour):
        self.neighbours.append(neighbour)

    def update (self):
        self.energy += 1

        if not self.flashed and self.energy > 9:
            self.flashed = True
            for i in self.neighbours:
                i.update()
    
    def reset (self):
        if self.flashed:
            self.flashed = False
            self.energy = 0
            return 1
        return 0

f = open("data/day11.txt")

octopusLines = [[Octopus(int(j)) for j in i.strip()] for i in f.readlines()]

for y, line in enumerate(octopusLines):
    for x, octopus in enumerate(line):
        if x > 0:
            octopus.addNeighbour(octopusLines[y][x - 1])
        if x < len(octopusLines[0]) - 1:
            octopus.addNeighbour(octopusLines[y][x + 1])
        if y > 0:
            octopus.addNeighbour(octopusLines[y - 1][x])
        if y < len(octopusLines) - 1:
            octopus.addNeighbour(octopusLines[y + 1][x])
        if x > 0 and y > 0:
            octopus.addNeighbour(octopusLines[y - 1][x - 1])
        if x > 0 and y < len(octopusLines) - 1:
            octopus.addNeighbour(octopusLines[y + 1][x - 1])
        if x < len(octopusLines[0]) - 1 and y < len(octopusLines) - 1:
            octopus.addNeighbour(octopusLines[y + 1][x + 1])
        if x < len(octopusLines[0]) - 1 and y > 0:
            octopus.addNeighbour(octopusLines[y - 1][x + 1])

octopi = [j for i in octopusLines for j in i]

flashes = 0
currFlashes = 0
n = 0

#print("\n".join("".join(str(j.energy) for j in i) for i in octopusLines))

while currFlashes < len(octopusLines) * len(octopusLines[0]):
    for j in octopi:
        j.update()

    currFlashes = sum(j.reset() for j in octopi)
    flashes += currFlashes

    n += 1

    if n == 100:
        print(f"Num flashes after 100 steps: {flashes}")

print(f"First synchronisation: {n}")