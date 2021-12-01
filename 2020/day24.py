def getAdjacent (pos):
    l = []
    for i in range(0, 6):
        xDir = 2 * (i % 2) - 1
        yDir = i // 2 - 1
        if yDir == 0:
            xDir *= 2
        l.append((xDir + pos[0], yDir + pos[1]))
    return l
f = open("day24.txt")
l = []
for i in f.readlines():
    l2 = []
    curr = ""
    for j in i.strip():
        if j in ['e', 'w']:
            l2.append(curr + j)
            curr = ""
        else:
            curr += j
    l.append(l2)

blackTiles = set()
for i in l:
    pos = [0, 0]
    for j in i:
        if j[-1] == 'e':
            pos[0] += 1
            if len(j) == 1:
                pos[0] += 1
        elif j[-1] == 'w':
            pos[0] -= 1
            if len(j) == 1:
                pos[0] -= 1
        if j[0] == 'n':
            pos[1] += 1
        elif j[0] == 's':
            pos[1] -= 1
    pStr = "{} {}".format(pos[0], pos[1])
    #print(pStr)
    if pStr in blackTiles:
        blackTiles.remove(pStr)
    else:
        blackTiles.add(pStr)

print("Num black tiles: {}".format(len(blackTiles)))
#print(getAdjacent((0,0)))
blackTiles = set([(int(i.split(" ")[0]), int(i.split(" ")[1])) for i in blackTiles])

n = 0
while n < 100:
    #print(blackTiles)
    checkTiles = set(blackTiles.copy())
    newBlackTiles = []
    for i in blackTiles:
        checkTiles.update([j for j in getAdjacent(i)])
    for i in checkTiles:
        colour = i in blackTiles
        adj = sum([1 for j in getAdjacent(i) if j in blackTiles])
        if colour and (adj == 1 or adj == 2):
            newBlackTiles.append(i)
        elif not colour and adj == 2:
            newBlackTiles.append(i)
    blackTiles = set(newBlackTiles)
    n += 1
    print("{}: {}".format(n, len(blackTiles)))

print("Num black tiles after 100: {}".format(len(blackTiles)))