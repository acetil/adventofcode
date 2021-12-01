f = open("day22.txt")


def calcScore (l):
    return sum(((len(l) - i) * num for i, num in enumerate(l)))
def playGame (player1, player2, gameNum = [0], indent = ""):
    gN = gameNum[0]
    #print("{}Playing game {}".format(indent, gN))
    roundNum = 1
    prevRounds = set("{} {}".format(player1, player2))
    while len(player1) != 0 and len(player2) != 0:
        card1 = player1.pop(0)
        card2 = player2.pop(0)
        #print("{}Round {} of game {}: {} {} {} {}".format(indent, roundNum, gN, card1, card2, player1, player2))
        if card1 > len(player1) or card2 > len (player2):
            if card2 > card1:
                #print("{}Player 2 wins round {} of game {} via deckout!".format(indent, roundNum, gN))
                player2.append(card2)
                player2.append(card1)
            else:
                #print("{}Player 1 wins round {} of game {} via deckout!".format(indent, roundNum, gN))
                player1.append(card1)
                player1.append(card2)
        else:
            gameNum[0] += 1
            result = playGame(player1[:card1], player2[:card2], gameNum, indent + " ")
            if result[0] == 0:
                player2.append(card2)
                player2.append(card1)
                #print("{}Player 2 wins round {} of game {}!".format(indent, roundNum, gN))
            else:
                player1.append(card1)
                player1.append(card2)
                #print("{}Player 1 wins round {} of game {}!".format(indent, roundNum, gN))
        rStr = "{} {}".format(player1, player2)
        if rStr in prevRounds:
            #print("{}Player 1 wins game {} via previous game!".format(indent, gN))
            return (calcScore(player1), 0)
        else:
            prevRounds.add(rStr)
            roundNum += 1
    #print("{}In game {}: Player 1 score: {}, Player 2 score = {}".format(indent, gN, calcScore(player1), calcScore(player2)))
    return (calcScore(player1), calcScore(player2))

playersText = [[j.strip() for j in i.strip().split("\n")][1:] for i in f.read().strip().split("\n\n")]

player1 = [int(i) for i in playersText[0]]
player1Original = player1[:]
player2 = [int(i) for i in playersText[1]]
player2Original = player2[:]

while len(player1) != 0 and len(player2) != 0:
    card1 = player1.pop(0)
    card2 = player2.pop(0)
    if card1 > card2:
        player1.append(card1)
        player1.append(card2)
    elif card2 > card1:
        player2.append(card2)
        player2.append(card1)
    else:
        player1.append(card1)
        player2.append(card2)

print("Player 1 score: {}".format(calcScore(player1)))
print("Player 2 score: {}".format(calcScore(player2)))

recGame = playGame(player1Original, player2Original)
print("Player 1 recursive score: {}".format(recGame[0]))
print("Player 2 recursive score: {}".format(recGame[1]))