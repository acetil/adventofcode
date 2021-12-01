f = open("day10.txt")

l = [int(i) for i in f.readlines()]

lSorted = sorted(l)
lSorted.insert(0, 0)
lSorted.append(max(l) + 3)
print(lSorted)
diffs = [lSorted[i] - lSorted[i - 1] for i in range(1, len(lSorted))]
print(diffs)
d1s = 0
d3s = 0
for i in diffs:
    if i == 1:
        d1s += 1
    elif i == 3:
        d3s += 1

print("{} * {} = {}".format(d1s, d3s, d1s * d3s))

l1 = [i for i in range(0, len(lSorted) - 1) if diffs[i] == 3]
l1.insert(0, 0)
l2 = [diffs[l1[i]:l1[i + 1]] for i in range(0, len(l1) - 1)]
for i in l2:
    if i[0] == 3:
        i.pop(0)
l3 = [len(i) for i in l2]
print(l2)
print(l3)
l4 = []
for i in l3:
    if i == 0 or i == 1:
        l4.append(1)
    if i == 2:
        l4.append(2)
    if i == 3:
        l4.append(4)
    if i == 4:
        l4.append(7)
print(l4)
m = 1
for i in l4:
    m *= i
print(m)