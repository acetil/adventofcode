def getNum (c):
    if c == "L":
        return 0
    else:
        return 2

def getAdjacentSeats (m, x, y):
    n = 0
    for i in range(-1, 2):
        for j in range(-1, 2):
            if i == 0 and j == 0:
                continue
            vec = [i, j]
            hasSeen = False
            mul = 1
            while x + vec[0] * mul >= 0 and x + vec[0] * mul < len(m[0]) and y + vec[1] * mul >= 0 and y + vec[1] * mul < len(m):
                if m[y + vec[1] * mul][x + vec[0] * mul] == 1:
                    hasSeen = True
                    break
                elif m[y + vec[1] * mul][x + vec[0] * mul] == 0:
                    break
                mul += 1
            if hasSeen:
                n += 1
    return n
def updateSeats (m):
    mNew = [[0 for i in m[j]] for j in range(0, len(m))]
    for i in range(0, len(m)):
        for j in range(0, len(m[i])):
            if m[i][j] != 2:
                n = getAdjacentSeats(m, j, i)
                #print(n)
                if n == 0 and m[i][j] == 0:
                    mNew[i][j] = 1
                elif n >= 5 and m[i][j] == 1:
                    mNew[i][j] = 0
                else:
                    mNew[i][j] = m[i][j]
            else:
                mNew[i][j] = 2
    return mNew

def printMap (m):
    for i in m:
        for j in i:
            if j == 0:
                print("L", end="")
            elif j == 1:
                print("#", end="")
            else:
                print(".", end="")
        print()
    print()

f = open("day11.txt")

m = [[0 if j == "L" else 2 for j in i.strip()] for i in f.readlines()]

last = m
#printMap(m)
m = updateSeats(m)
while m != last:
    #printMap(m)
    last = m
    m = updateSeats(m)

#printMap(m)
print("Occupied seats: {}".format(sum([sum([i for i in j if i == 1]) for j in m])))