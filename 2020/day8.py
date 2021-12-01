def runProgram (instr):
    index = 0
    acc = 0
    length = len(instr)
    seen = set()
    halted = True
    while index < length:
        if index in seen:
            halted = False
            break
        seen.add(index)
        i = instr[index]
        if i[0] == "nop":
            pass
        elif i[0] == "acc":
            acc += i[1]
        else:
            index += i[1] - 1
        index += 1
    return (acc, halted)

f = open("day8.txt")
instr = []
for l in f.readlines():
    a = l.strip().split(" ")
    instr.append([a[0], int(a[1])])


print("Num instructions: {}".format(len(instr)))
acc, halted = runProgram(instr)
print("Last before repeat: {}".format(acc))
for i in range(0, len(instr)):
    if instr[i][0] == "jmp":
        instr[i][0] = "nop"
        acc,halted = runProgram(instr)
        if halted:
            break
        instr[i][0] = "jmp"
    elif instr[i][0] == "nop":
        instr[i][0] = "jmp"
        acc,halted = runProgram(instr)
        if halted:
            break
        instr[i][0] = "nop"

if not halted:
    print("No code found!")
else:
    print("Accumulator value: {}".format(acc))