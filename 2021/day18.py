import math
class SnailfishNumber:
    def reduce (self):
        b, newSelf, _, _ = self.tryExplode()
        if b:
            return b, newSelf
        
        return self.trySplit()
    
    def fullReduce (self):
        curr = self
        while True:
            b, curr = curr.reduce()
            if not b:
                break
        
        return curr

    def tryExplode (self, depth=0):
        return False, self, None, None
    
    def trySplit (self):
        raise NotImplementedError()
    
    def propagate (self, n, isLeft):
        raise NotImplementedError()
    
    def toStr (self) -> str:
        raise NotImplementedError()
    
    def add (self, other):
        if other == None:
            return self
        return NestedSnailfishNumber(self, other)
    
    def addAndReduce (self, other):
        return self.add(other).fullReduce()
    
    def getMagnitude (self):
        raise NotImplementedError
    
    def copy (self):
        raise NotImplementedError

class NestedSnailfishNumber(SnailfishNumber):
    def __init__ (self, left: SnailfishNumber, right: SnailfishNumber):
        self.left = left
        self.right = right
    
    @classmethod
    def parseNum (cls, inList):
        return cls(parseSnailfishNum(inList[0]), parseSnailfishNum(inList[1]))
    
    def tryExplode(self, depth=0):
        if depth == 4:
            return True, LiteralSnailfishNumber(0), self.left, self.right

        b, newLeft, lProp, rProp = self.left.tryExplode(depth + 1)
        if b:
            self.left = newLeft if newLeft is not None else self.left
            self.right = self.right.propagate(rProp, True)
            return True, self, lProp, None
        
        b, newRight, lProp, rProp = self.right.tryExplode(depth + 1)

        if b:
            self.right = newRight if newRight is not None else self.right
            self.left = self.left.propagate(lProp, False)
            return True, self, None, rProp
        
        return super().tryExplode(depth)
    
    def trySplit(self):
        b, newLeft = self.left.trySplit()
        if b:
            self.left = newLeft
            return True, self
        b, newRight = self.right.trySplit()

        if b:
            self.right = newRight
            return True, self
        
        return False, self
    
    def propagate(self, n, isLeft):
        if n is None:
            return self

        if isLeft:
            self.left = self.left.propagate(n, True)
            return self
        else:
            self.right = self.right.propagate(n, False)
            return self

    def toStr(self) -> str:
        return f"[{self.left.toStr()},{self.right.toStr()}]"

    def getMagnitude(self):
        return 3 * self.left.getMagnitude() + 2 * self.right.getMagnitude()

    def copy(self):
        return NestedSnailfishNumber(self.left.copy(), self.right.copy())


class LiteralSnailfishNumber(SnailfishNumber):
    def __init__ (self, value):
        self.value = value
    
    def add(self, other: SnailfishNumber):
        if isinstance(other, LiteralSnailfishNumber):
            return LiteralSnailfishNumber(self.value + other.value)
        return super().add(other)
    
    def trySplit(self):
        if self.value >= 10:
            return True, NestedSnailfishNumber(LiteralSnailfishNumber(self.value // 2), LiteralSnailfishNumber(int(math.ceil(self.value / 2))))
        
        return False, self

    def propagate(self, n, isLeft):
        return self.add(n)

    def toStr(self) -> str:
        return str(self.value)

    def getMagnitude(self):
        return self.value
    
    def copy(self):
        return LiteralSnailfishNumber(self.value)


def parseSnailfishNum (inList) -> SnailfishNumber:
    if isinstance(inList, int):
        return LiteralSnailfishNumber(inList)
    else:
        return NestedSnailfishNumber.parseNum(inList)

def parseNumStr (line: str):
    line = line.strip()

    if line[0] == "[":
        subline = line[1:len(line) - 1]
        level = 0
        i = 0
        while subline[i] != "," or level > 0:
            if subline[i] == "[":
                level += 1
            elif subline[i] == "]":
                level -= 1
            i += 1
        
        return [parseNumStr(subline[:i]), parseNumStr(subline[i + 1:])]

    else:
        return int(line)

def sumNums (numbers):
    result = numbers.pop(0)

    while len(numbers) > 0:
        result = result.addAndReduce(numbers.pop(0))
    
    return result

f = open("data/day18.txt")

numbers = [parseSnailfishNum(parseNumStr(i)) for i in f.readlines() if i != ""]

for i in range(len(numbers)):
    numbers[i] = numbers[i].fullReduce()

    print(numbers[i].toStr())

print()

result = sumNums([i.copy() for i in numbers])

print(result.toStr())
print(f"Magnitude of sum: {result.getMagnitude()}")
    
print(f"Max magnitude: {max(sumNums([numbers[i].copy(), numbers[j].copy()]).getMagnitude() for i in range(len(numbers)) for j in range(len(numbers)) if i != j)}")