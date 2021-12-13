def addDot (mat, x, y, val):
    while len(mat) <= y:
        mat.append([False for i in range(0, len(mat[0]))])
    
    while len(mat[0]) <= x:
        for i in mat:
            i.append(False)
    
    mat[y][x] |= val

def flipMat (mat):
    newMat = [[]]
    for y, l in enumerate(mat):
        for x, b in enumerate(l):
            addDot(newMat, y, x, b)
    return newMat

def foldMat (mat, dir, pos):
    if dir == "x":
        return flipMat(foldMat(flipMat(mat), "y", pos))

    newMat = [[]]

    size = max(pos, len(mat) - pos - 1)

    for y, l in enumerate(mat):
        if y != pos:
            for x, b in enumerate(l):
                addDot(newMat, x, (size - pos) + y if y < pos else (size - (y - pos)), b)
    
    return newMat
    

f = open("data/day13.txt")

l = f.readline()

mat = [[]]

while l.strip() != "":
    point = [int(i) for i in l.split(",")]
    addDot(mat, point[0], point[1], True)
    l = f.readline()


first = True
for line in f.readlines():
    if line != "":
        fold = line[11:].split("=")
        mat = foldMat(mat, fold[0], int(fold[1]))
        if first:
            print(f"Sum of first fold: {sum(sum(j for j in i) for i in mat)}")
            first = False

print("\n".join("".join("#" if j else "." for j in i) for i in mat))