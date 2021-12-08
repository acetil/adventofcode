import itertools
perms = list(itertools.permutations("abcdefg"))

def decodeDigit (digit, code):
    return "".join(sorted("abcdefg"[code.index(i)] for i in digit))

def solveLine (codedDigits, output):
    digits = ["abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"]

    code = None
    for i in perms:
        if all(decodeDigit(j, i) in digits for j in codedDigits):
            code = i
            break
    
    outputDigits = [decodeDigit(i, code) for i in output]

    return int("".join(str(digits.index(i)) for i in outputDigits))
   


f = open("day08.txt")

digits = []
outputs = []
for i in f.readlines():
    l = i.split("|")

    digits.append([i.strip() for i in l[0].strip().split(" ")])
    outputs.append([i.strip() for i in l[1].strip().split(" ")])


lens1478 = [2, 4, 3, 7]

print(f"Number of 1,4,7,8: {sum(sum(len(j) in lens1478 for j in i) for i in outputs)}")

print(f"Sum of numbers: {sum(solveLine(digits[i], outputs[i]) for i in range(0, len(digits)))}")