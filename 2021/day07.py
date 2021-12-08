def getFuelCostP1 (l, n):
    return sum(abs(i - n) for i in l)

def getFuelCostP2 (l, n):
    return sum(abs(i - n) * (abs(i - n) + 1) // 2 for i in l)

f = open("day07.txt")

l = [int(i) for i in f.readline().split(",")]

print(f"Minimum fuel cost part 1: {min(getFuelCostP1(l, i) for i in range(min(l), max(l) + 1))}")
print(f"Minimum fuel cost part 2: {min(getFuelCostP2(l, i) for i in range(min(l), max(l) + 1))}")