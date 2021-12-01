import re
f = open("day21.txt")

lines = f.readlines()
foods = []
confAllergens = []
for i in lines:
    iCpy = i[:]
    foods.append(re.sub(r"\([a-z, ]+\)", "", iCpy).strip().split(" "))
    #print(re.sub(r"\([a-z, ]+\)", "", iCpy).strip())
    #print(re.search(r"\([a-z, ]+\)", i).group(0))
    confAllergens.append([j.strip() for j in re.sub(r"\(contains|\)", "", re.search(r"\([a-z, ]+\)", i).group(0)).strip().split(",")])

#print(foods)
#print(confAllergens)
allergenDict = dict()
for i, allergens in enumerate(confAllergens):
    for j in allergens:
        if j not in allergenDict:
            allergenDict[j] = set(foods[i])
        else:
            allergenDict[j] = set(foods[i]).intersection(allergenDict[j])

possibleAllergens = set()
for i in allergenDict:
    possibleAllergens.update(allergenDict[i])
allIngredients = set()
for i in foods:
    allIngredients.update(i)
n = 0

for i in foods:
    for j in i:
        if j not in possibleAllergens:
            #print("{} cannot be an allergen!".format(j))
            n += 1

print("{} occurences of non-allergens".format(n))

canonical = dict()
while len(allergenDict) > 0:
    solvedAllergen = ""
    name = ""
    for i in allergenDict:
        if len(allergenDict[i]) == 1:
            solvedAllergen = i
            name = list(allergenDict[i])[0]
            break
    for i in allergenDict:
        if name in allergenDict[i]:
            allergenDict[i].remove(name)
    allergenDict.pop(solvedAllergen)
    canonical[solvedAllergen] = name

allergens = [i for i in canonical]
allergens.sort()
#print(allergens)
print("Canonical list: \n{}".format(",".join((canonical[i] for i in allergens))))