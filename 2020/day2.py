f = open("day2.txt", "r")
l = []
for i in f.readlines():
    l1 = i.split(" ")
    l2 = [int(i) for i in l1[0].split("-")]
    c = l1[1][0]
    p = l1[2]
    l.append((l2, c, p))

tot = 0
for i in l:
    n = 0
    if i[2][i[0][0] - 1] == i[1]:
        n += 1
    if i[2][i[0][1] - 1] == i[1]:
        n += 1
    if n == 1:
        tot += 1

print(tot)