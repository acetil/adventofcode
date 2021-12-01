f = open("day1.txt", "r")
l = [int(i) for i in f.readlines()]
for i in l:
    for j in l:
        for k in l:
            if i != j and j != k and i != k and i + j + k == 2020:
                print(i * j * k)
                break