f = open("day17.txt")
l = [i.strip() for i in f.readlines()]
#print(l)
cubes = set(((j  - len(l[i]) // 2, i - len(l) // 2, 0) for i in range(0, len(l)) for j in range(0, len(l[i])) if l[i][j] == '#'))
#print(cubes)

n = 0
while n < 6:
    checkCubes = set()
    for c in cubes:
        for i in range(-1, 2):
            for j in range(-1, 2):
                for k in range(-1, 2):
                    checkCubes.add((c[0] + i, c[1] + j, c[2] + k))
    newCubes = set()
    for c in checkCubes:
        num = 0
        for i in range(-1, 2):
            for j in range(-1, 2):
                for k in range(-1, 2):
                    if i == 0 and j == 0 and k == 0:
                        continue
                    if (c[0] + i, c[1] + j, c[2] + k) in cubes:
                        num += 1
        if c in cubes and (num == 2 or num == 3):
            newCubes.add(c)
        elif num == 3:
            newCubes.add(c)

    cubes = newCubes
    #print(cubes)
    n += 1
print("Num cubes 3d: {}".format(len(cubes)))

cubes = set(((j  - len(l[i]) // 2, i - len(l) // 2, 0, 0) for i in range(0, len(l)) for j in range(0, len(l[i])) if l[i][j] == '#'))

n = 0
while n < 6:
    checkCubes = set()
    for c in cubes:
        for i in range(-1, 2):
            for j in range(-1, 2):
                for k in range(-1, 2):
                    for m in range(-1, 2):
                        checkCubes.add((c[0] + i, c[1] + j, c[2] + k, c[3] + m))
    newCubes = set()
    for c in checkCubes:
        num = 0
        for i in range(-1, 2):
            for j in range(-1, 2):
                for k in range(-1, 2):
                    for m in range(-1, 2):
                        if i == 0 and j == 0 and k == 0 and m == 0:
                            continue
                        if (c[0] + i, c[1] + j, c[2] + k, c[3] + m) in cubes:
                            num += 1
        if c in cubes and (num == 2 or num == 3):
            newCubes.add(c)
        elif num == 3:
            newCubes.add(c)

    cubes = newCubes
    #print(cubes)
    n += 1
print("Num cubes 4d: {}".format(len(cubes)))