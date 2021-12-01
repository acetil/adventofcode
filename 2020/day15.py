f = open("day15.txt")

lastSeen = dict()

l = [int(i) for i in f.readline().split(',')]

for i, n in enumerate(l[0:len(l) - 1]):
    lastSeen[n] = i

n = len(l) - 1
last = l[-1]
while n < 30000000 - 1:
    if last not in lastSeen:
        lastSeen[last] = n
        last = 0
    else:
        m = n - lastSeen[last]
        lastSeen[last] = n
        last = m
    #print(last)
    n += 1

print("2020th num: {}".format(last))
