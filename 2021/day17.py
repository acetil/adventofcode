import math

def getMaxHeight (vel):
    return getMaxDistance(vel[1])

def getMaxDistance (vel):
    if vel <= 0:
        return 0
    
    return vel * (vel + 1) // 2

def yEntryTimes (yVel, yRange):
    if yRange[1] > yRange[0]:
        return yEntryTimes(yVel, (yRange[1], yRange[0]))

    t = 0
    times = []
    height = 0

    if yRange[1] <= 0 <= yRange[0]:
        times.append(0)

    while yVel > 0 or height > yRange[1]:
        t += 1
        height += yVel
        yVel -= 1

        if yRange[1] <= height <= yRange[0]:
            times.append(t)
    
    return times

def findMaxY (yRange):
    if yRange[1] > yRange[0]:
        return findMaxY((yRange[1], yRange[0]))
    
    if yRange[1] <= 0 <= yRange[0]:
        return math.inf
    
    if yRange[0] >= 0:
        return yRange[0]
    else:
        return -yRange[1]

def findMinY (yRange):
    if yRange[1] > yRange[0]:
        return findMinY((yRange[1], yRange[0]))
    
    return min(yRange[1], 0)
    
def findMaxX (xRange):
    return max(xRange[0], xRange[1])

def xEntryTimes (xRange, xVel):
    if xRange[0] > xRange[1]:
        return xEntryTimes((xRange[1], xRange[0]), xVel)
    return [i for i in range(0, xVel) if xRange[0] <= xVel * (xVel + 1) // 2 - (xVel - i) * (xVel - i + 1) // 2 <= xRange[1]]


def getInitialVels (xRange, yRange):
    if xRange[1] < 0:
        vel = getInitialVels((-xRange[0], -xRange[1]), yRange)
        return (-vel[0], vel[1])
    
    if xRange[0] <= 0:
        return (0, findMaxY(yRange))

    entryTimes = dict((i, set(xEntryTimes(xRange, i))) for i in range(1, findMaxX(xRange) + 1))

    vels = []
    for i in range(findMinY(yRange), findMaxY(yRange) + 1):
        yEntries = set(yEntryTimes(i, yRange))

        for j in entryTimes:
            if len(yEntries.intersection(entryTimes[j])) != 0 or \
                (len(yEntries) > 0 and len(entryTimes[j]) > 0 and max(yEntries) >= max(entryTimes[j]) and \
                    (xRange[0] <= getMaxDistance(j) <= xRange[1] or xRange[1] <= getMaxDistance(j) <= xRange[0])):
                
                vels.append((j, i))
    
    return vels



f = open("data/day17.txt")

for i in f.readlines():
    if i != "":
        l = [j.strip()[2:] for j in i.strip()[13:].split(",")]

        vels = list(set(getInitialVels([int(j) for j in l[0].split("..")], [int(j) for j in l[1].split("..")])))

        vel = max(vels, key = lambda x: getMaxHeight(x))

        print(f"Number of vels: {len(vels)}, Max height vel: {vel}, height: {getMaxHeight(vel)}")

