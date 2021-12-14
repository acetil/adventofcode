def getOcurrences (polymerPairs):
    occurrences = {}

    for i in polymerPairs:
        occurrences[i[0]] = occurrences.get(i[0], 0) + polymerPairs[i]
    
    return occurrences

f = open("data/day14.txt")

polymer = f.readline().strip()

polymerPairs = {}

for i in [polymer[j:j + 2] for j in range(0, len(polymer))]:
    polymerPairs[i] = polymerPairs.get(i, 0) + 1

rules = {}

f.readline()

for i in f.readlines():
    if i.strip() != "":
        line = [j.strip() for j in i.split(" -> ")]

        rules[line[0]] = [line[0][0] + line[1], line[1] + line[0][1]]

for n in range(0, 40):
    newPairs = {}

    for i in polymerPairs:
        for j in rules.get(i, [i]):
            newPairs[j] = newPairs.get(j, 0) + polymerPairs[i]
    
    polymerPairs = newPairs
    
    if n == 9:
        print(f"Max - min after 10: {max(getOcurrences(polymerPairs).values()) - min(getOcurrences(polymerPairs).values())}")

print(f"Max - min after 40: {max(getOcurrences(polymerPairs).values()) - min(getOcurrences(polymerPairs).values())}")