f = open("day16.txt")

comps = f.read().split("\n\n")

rules = []

for i in comps[0].split("\n"):
    l = i.split(":")
    l1 = l[1].strip().split(" ")
    #print(l1)
    r1 = tuple([int(i) for i in l1[0].split("-")])
    r2 = tuple([int(i) for i in l1[2].split("-")])
    rules.append((l[0].strip(), r1, r2))

#print(rules)
rate = 0
validTickets = []
for l in comps[2].split("\n")[1:]:
    #print(l)
    nums = [int(i) for i in l.strip().split(",")]
    valid = True
    for i in nums:
        for j in rules:
            if (j[1][0] <= i and j[1][1] >= i) or (j[2][0] <= i and j[2][1] >= i):
                break
        else:
            rate += i
            valid = False
    if valid:
        validTickets.append(nums)

print("Error rate: {}".format(rate))

ruleOptions = dict()
for i in range(0, len(validTickets[0])):
    validRules = rules[:]
    for t in validTickets:
        newValidRules = []
        for j in validRules:
            #print(j)
            if (j[1][0] <= t[i] and j[1][1] >= t[i]) or (j[2][0] <= t[i] and j[2][1] >= t[i]):
                newValidRules.append(j)
        validRules = newValidRules[:]
        if len(validRules) == 1:
            break
    #print(validRules)
    ruleOptions[i] = validRules

done = False
foundRules = dict()
while not done:
    #print(ruleOptions[16])
    done = True
    resolveList = []
    for i in ruleOptions:
        if len(ruleOptions[i]) == 1:
            resolveList.append(i)
            done = False
    #print(resolveList)
    for i in resolveList:
        foundRules[ruleOptions[i][0][0]] = i
        for j in ruleOptions:
            if ruleOptions[i][0] in ruleOptions[j] and j != i:
                ruleOptions[j].remove(ruleOptions[i][0])
        ruleOptions.pop(i)
print(foundRules)
myTicket = [int(i) for i in comps[1].strip().split("\n")[1].split(",")]
val = 1
for i in foundRules:
    if i.split(" ")[0].strip() == "departure":
        #print("Departure!")
        val *= myTicket[foundRules[i]]
print("Ticket value: {}".format(val))