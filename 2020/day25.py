def encrypt (num, loopSize):
    n = 1
    for i in range(0, loopSize):
        n *= num
        n %= 20201227
    return n
f = open("day25.txt")
doorPublic = int(f.readline())
cardPublic = int(f.readline())

n = 0
testPublic = 1
while testPublic != doorPublic:
    testPublic *= 7
    testPublic %= 20201227
    n += 1
doorLoop = n
n = 0
testPublic = 1

while testPublic != cardPublic:
    testPublic *= 7
    testPublic %= 20201227
    n += 1
cardLoop = n

if encrypt(doorPublic, cardLoop) == encrypt(cardPublic, doorLoop):
    print("Loopsizes: door={}, card={}. Encryption key: {}".format(doorLoop, cardLoop, encrypt(cardPublic, doorLoop)))
else:
    print("Encryption keys not equal! Loopsizes: door={}, card={}".format(doorLoop, cardLoop))