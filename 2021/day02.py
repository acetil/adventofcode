f = open("data/day02.txt")

totalVec1 = (0, 0)
totalVec2 = (0, 0)
aim = 0

for i in f.readlines():
    l = i.split(" ")
    vec = (0, 0)

    if l[0] == "forward":
        vec = [int(l[1]), 0]
        totalVec2 = (totalVec2[0] + int(l[1]), totalVec2[1] + int(l[1]) * aim)
    elif l[0] == "down":
        vec = [0, int(l[1])]
        aim += int(l[1])
    elif l[0] == "up":
        vec = [0, -int(l[1])]
        aim -= int(l[1])

    totalVec1 = (totalVec1[0] + vec[0], totalVec1[1] + vec[1])


print(f"Multiplied position part 1: {totalVec1[0] * totalVec1[1]}")
print(f"Multiplied position part 2: {totalVec2[0] * totalVec2[1]}")