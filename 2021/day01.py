f = open("day01.txt")

vals = [int(i) for i in f.readlines()]

print(f"Num increased: {sum(1 for i in range(1, len(vals)) if vals[i] > vals[i - 1])}")

print(f"Sliding num increased: {sum(1 for i in range(1, len(vals) - 2) if sum(vals[i:i + 3]) > sum(vals[i - 1:i + 2]))}")