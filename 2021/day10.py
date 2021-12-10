CONSUME_SUCCESS = 0
CONSUME_INVALID = 1
CONSUME_INCOMPLETE = 2

def getCost (inputL):
    return {
        ")" : 3,
        "]" : 57,
        "}" : 1197,
        ">" : 25137
    }.get(inputL[0], 0)

def getCompletionPoints (c):
    return {
        ")" : 1,
        "]" : 2,
        "}" : 3,
        ">" : 4
    }.get(c, 0)

def consumeCharacter (inputL, c):
    if len(inputL) == 0:
        return CONSUME_INCOMPLETE
    elif inputL[0] != c:
        return CONSUME_INVALID
    else:
        inputL.pop(0)
        return CONSUME_SUCCESS

def consumeBracketToken (inputL, opening, closing):
    if consumeCharacter(inputL, opening) == CONSUME_SUCCESS:
        v = consumeToken(inputL)
        if v == 0:
            res = consumeCharacter(inputL, closing)
            if res == CONSUME_SUCCESS:
                return 0
            elif res == CONSUME_INVALID:
                return getCost(inputL)
            else:
                return -getCompletionPoints(closing)
        else:
            return v if v > 0 else v * 5 - getCompletionPoints(closing)
    return None


def consumeToken (inputL):
    v = 0
    if (v := consumeBracketToken(inputL, "(", ")")) != None or (v := consumeBracketToken(inputL, "[", "]")) != None \
        or (v := consumeBracketToken(inputL, "<", ">")) != None or (v := consumeBracketToken(inputL, "{", "}")) != None:
        return v if v != 0 else consumeToken(inputL)
    
    return 0

f = open("data/day10.txt")

result = [consumeToken(list(i.strip())) for i in f.readlines()]

print(f"Sum invalid tokens: {sum(i for i in result if i > 0)}")

completedScores = sorted(-i for i in result if i < 0)

print(f"Completion score: {completedScores[len(completedScores) // 2]}")