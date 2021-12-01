import re
class Mask1:
    def __init__ (self):
        self.andMask = 2 ** 36 - 1
        self.orMask = 0
    def setMask (self, s):
        n = 35
        self.andMask = 0
        self.orMask = 0
        for i in range(0, len(s)):
            if s[i] == '0':
                self.andMask |= 1 << n
            elif s[i] == '1':
                self.orMask |= 1 << n
            n -= 1
        self.andMask ^= 2 ** 36 - 1
    def mask (self, n):
        return n & self.andMask | self.orMask

class Mask2:
    def __init__ (self):
        self.andMask = 0
        self.orMask = 0
        self.floatMask = []
    def setMask (self, s):
        n = 35
        self.andMask = 0
        self.orMask = 0
        self.floatMask = []
        for i in range(0, len(s)):
            if s[i] == '1':
                self.orMask |= 1 << n
                self.andMask |= 1 << n
            elif s[i] == '0':
                self.andMask |= 1 << n
            else:
                self.floatMask.append(n)
            n -= 1
    def mask (self, n):
        mems = [n & self.andMask | self.orMask]
        for i in self.floatMask:
            memsNew = []
            for j in mems:
                memsNew.append(j)
                memsNew.append(j | 1 << i)
            mems = memsNew
        #print(mems)
        return mems

f = open("day14.txt")
l = []
for i in f.readlines():
    p = i.strip().split(" = ")
    l.append((p[0].strip(), p[1].strip()))

memory = dict()

mask = Mask1()

for i, s in l:
    if i == "mask":
        mask.setMask(s)
    else:
        n = int(s)
        index = int(re.sub(r"[a-z\[\]]", "", i))
        memory[index] = mask.mask(n)

total = 0

for i in memory:
    total += memory[i]

print("Total 1: {}".format(total))

memory = dict()
mask = Mask2()

for i, s in l:
    if i == "mask":
        mask.setMask(s)
    else:
        n = int(s)
        index = int(re.sub(r"[a-z\[\]]", "", i))
        mems = mask.mask(index)
        for j in mems:
            memory[j] = n
#print(memory)
total = 0
for i in memory:
    total += memory[i]

print("Total 2: {}".format(total)) 