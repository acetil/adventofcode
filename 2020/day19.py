class Rule:
    def __init__ (self, ruleStr, n):
        self.num = n
        if ruleStr[0] =='"':
            self.exactMatch = True
            self.subRulesNum = []
            self.matchChar = ruleStr[1]
        else:
            parts = [i.strip() for i in ruleStr.split('|')]
            self.exactMatch = False
            self.subRulesNum = []
            self.matchChar = None
            for i in parts:
                self.subRulesNum.append([int(j) for j in i.split(" ")])
        
    def buildRules (self, ruleDict):
        self.subRules = []
        for i in self.subRulesNum:
            self.subRules.append([ruleDict[j] for j in i])
    
    def match (self, s, indent = ""):
        #print("{}Matching {} with string {}".format(indent, self.num, s))
        if self.exactMatch:
            if len(s) == 0:
                #print("{}Match {} failed".format(indent, self.num))
                return []
            if s[0] == self.matchChar:
                #print("{}Match {} success".format(indent, self.num))
                return [1]
            else:
                #print("{}Match {} failed".format(indent, self.num))
                return []
        matches = []
        for i in self.subRules:
            nL = [0]
            for j in i:
                nLNew = []
                for n in nL:
                    nNew = j.match(s[n:], indent + " ")
                    nLNew += [n + k for k in nNew]

                nL = nLNew
            matches += nL
        #print("{}Match {} failed".format(indent, self.num))
        return matches
    



f = open("day19.txt")

parts = f.read().split("\n\n")
ruleDict = dict()
for i in parts[0].split("\n"):
    parts2 = [j.strip() for j in i.split(":")]
    n = int(parts2[0])
    ruleDict[n] = Rule(parts2[1], n)
    #print("{} {}".format(n, ruleDict[n].subRulesNum))

for i in ruleDict:
    ruleDict[i].buildRules(ruleDict)

n = 0

for i in parts[1].strip().split("\n"):
    if len(i.strip()) in ruleDict[0].match(i.strip()):
        n += 1
        #print("{} matches!".format(i.strip()))
    else:
        #print("{} does not match!".format(i.strip()))
        pass

print("Num matches: {}".format(n)) 