def markNumber (board, n):
    for i, l in enumerate(board):
        for j, x in enumerate(l):
            if x == n:
                board[i][j] = -1

def hasWon (board):
    return any(sum(i) == -len(board) for i in board) or any(sum(board[j][i] for j in range(len(board))) == -len(board[0]) for i in range(len(board[0])))


f = open("day04.txt")

rand = [int(i) for i in f.readline().split(",")]

f.readline()


boards = [i for i in f.read().split("\n\n")]

boards = [[[int(k) for k in j.split(" ") if k != ""] for j in i.strip().split("\n")] for i in boards]

winningOrder = []
winN = []
for i in rand:
    for j in boards:
        if not hasWon(j):
            markNumber(j, i)
        if hasWon(j) and j not in winningOrder:
            winningOrder.append(j)
            winN.append(i)
    
    if len(winningOrder) == len(boards):
        break

print(f"Winning score: {sum(sum(j for j in i if j != -1) for i in winningOrder[0]) * winN[0]}")
print(f"Last winning score: {sum(sum(j for j in i if j != -1) for i in winningOrder[-1]) * winN[-1]}")