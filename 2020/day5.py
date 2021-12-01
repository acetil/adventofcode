f = open("day5.txt")

l = f.readlines()

passes = []

for i in l:
    passes.append((int("".join(["0"  if j == "F" else "1" for j in i[:7]]), 2), (int("".join(["0"  if j == "L" else "1"for j in i[7:10]]), 2))))

ids = [i[0] * 8 + i[1] for i in passes]

print("Max id: {}".format(max(ids)))

ids.sort()

prev = 0
ans = 0
for i in ids:
    if prev != 0 and prev + 1 != i:
        ans = prev + 1
        break
    prev = i

print("Ans: {}".format(ans))