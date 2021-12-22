def deterministic (playerStart):
    players = [(i, 0) for i in playerStart]


    curr = 0
    n = 0
    while players[0][1] < 1000 and players[1][1] < 1000:
        move = (n + 1) % 100 + (n + 2) % 100 + (n + 3) % 100

        n += 3
        pPos = (players[curr][0] + move) % 10

        players[curr] = (pPos, players[curr][1] + pPos + 1)

        curr = (curr + 1) % 2

    print(f"Player 1 score: {players[0][1]}, player 2 score: {players[1][1]}, n: {n}")
    return min(players[0][1], players[1][1]) * n

def quantumAlgo (p1score, p2score, p1pos, p2pos, memoised):
    if p1score >= 21:
        return (1, 0)
    elif p2score >= 21:
        return (0, 1)
    
    if memoised[p1score][p2score][p1pos][p2pos] is not None:
        return memoised[p1score][p2score][p1pos][p2pos]

    moves = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]

    result = (0, 0)

    for i in moves:
        pos = (p1pos + i[0]) % 10
        score = p1score + pos + 1

        x = quantumAlgo(p2score, score, p2pos, pos, memoised)

        result = (result[0] + x[1] * i[1], result[1] + x[0] * i[1])

    memoised[p1score][p2score][p1pos][p2pos] = result

    return result


def quantum (playerStart):
    memoised = [[[[None for _ in range(0, 10)] for _ in range(0, 10)] for _ in range(0, 21)] for _ in range(0, 21)]

    res = quantumAlgo(0, 0, playerStart[0], playerStart[1], memoised)

    print(f"Player 1 universes: {res[0]}, player 2 universes: {res[1]}")

    return max(res)


f = open("data/day21.txt")


players = [int(f.readline().strip()[28:]) - 1, int(f.readline().strip()[28:]) - 1]

print(f"Part 1 solution: {deterministic(players)}")
print(f"Part 2 solution: {quantum(players)}")

