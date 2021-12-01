import re

def evaluateExpression (tokens):
    numStack = []
    opStack = []
    for i in tokens:
        #print("{} {}".format(numStack, opStack))
        if i == '(':
            opStack.append(i)
        elif i == ')':
            while len(numStack) > 1 and len(opStack) > 0 and opStack[-1] != '(':
                op = opStack.pop()
                num2 = numStack.pop()
                num1 = numStack.pop()
                if op == '*':
                    numStack.append(num1 * num2)
                else:
                    numStack.append(num1 + num2)
            opStack.pop()
        elif i == "+":
            opStack.append(i)
        elif i == '*':
            while len(numStack) > 1 and len(opStack) > 0 and (opStack[-1] == '+' or opStack[-1] == '*'):
                op = opStack.pop()
                num2 = numStack.pop()
                num1 = numStack.pop()
                if op == '+':
                    numStack.append(num1 + num2)
                else:
                    numStack.append(num1 * num2)
            opStack.append('*')
        else:
            numStack.append(int(i))
    while len(numStack) > 1 and len(opStack) > 0:
        op = opStack.pop()
        num2 = numStack.pop()
        num1 = numStack.pop()
        if op == '*':
            numStack.append(num1 * num2)
        else:
            numStack.append(num1 + num2)
    return numStack[-1]
f = open("day18.txt")

l = [i.strip() for i in f.readlines()]
results = []
for line in l:
    tokens = re.findall(r"(\(|\)|\*|\+|\d+)", line)
    results.append(evaluateExpression(tokens))
    #print(results[-1])
#print(len(results))
print("Sum of results: {}".format(sum(results)))