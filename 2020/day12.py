f = open("day12.txt")

l = [(i.strip()[0], int(i.strip()[1:])) for i in f.readlines()]

pos = [0, 0]
waypoint = [10, 1]

for i in l:
    if i[0] == 'N':
        waypoint[1] += i[1]
    elif i[0] == 'S':
        waypoint[1] -= i[1]
    elif i[0] == 'E':
        waypoint[0] += i[1]
    elif i[0] == 'W':
        waypoint[0] -= i[1]
    elif i[0] == 'F':
        pos[0] += i[1] * waypoint[0]
        pos[1] += i[1] * waypoint[1]
    else:
        rot = abs(i[1] // 90)
        #print(rot)
        for j in range(0, rot):
            temp = waypoint[0]
            waypoint[0] = waypoint[1]
            waypoint[1] = temp
            if i[0] == 'L':
                waypoint[0] *= -1
            else:
                waypoint[1] *= -1

print("Final pos: ({}, {})".format(pos[0], pos[1]))
print("Distance: {}".format(abs(pos[0]) + abs(pos[1])))
