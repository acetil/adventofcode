def updateFish (buckets):
    doubling = buckets[0]
    buckets = [i for i in buckets[1:]]
    buckets.append(doubling)
    buckets[6] += doubling

    return buckets

f = open("data/day06.txt")

l = [int(i) for i in f.readline().split(",")]


buckets = [l.count(0), l.count(1), l.count(2), l.count(3), l.count(4), l.count(5), l.count(6), l.count(7), l.count(8)]

for i in range(0, 256):
    buckets = updateFish(buckets)

    if i == 79:
        print(f"Num lanternfish after 80 days: {sum(buckets)}")

print(f"Num lanternfish after 256 days: {sum(buckets)}")