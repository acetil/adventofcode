f = open("day9.txt")
l = [int(i) for i in f.readlines()]

index = 25

done = False
target = -1

while not done:
    check = l[index - 25:index]
    n = l[index]
    for i in range(0, 25):
        for j in range(i + 1, 25):
            if check[i] + check[j] == n:
                break
        else:
            continue
        break
    else:
        target = n
        done = True
    index += 1

if done:
    print("First num: {}".format(target))

sumRange = [0, 1]
s = sum(l[sumRange[0]:sumRange[1]])
while s != target:
    if s < target:
        sumRange[1] += 1
    else:
        sumRange[0] += 1
    s = sum(l[sumRange[0]:sumRange[1]])

small = min(l[sumRange[0]:sumRange[1]])
big = max(l[sumRange[0]:sumRange[1]])

print("Range: {} to {}".format(sumRange[0], sumRange[1]))
print("Small: {}".format(small))
print("Big: {}".format(big))
print("Sum: {}".format(small + big))