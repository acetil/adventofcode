import functools
grps = [[]]
f = open("day6.txt")
for i in f.readlines():
    if i.strip() == "":
        grps.append([])
    else:
        grps[-1].append(i.strip())

if grps[-1] == []:
    grps.pop()

qs = [functools.reduce(lambda x, y: x.intersection(y), [set([i for i in s]) for s in grp]) for grp in grps]
print(qs)
print("Sum qs: {}".format(sum([len(i) for i in qs])))
