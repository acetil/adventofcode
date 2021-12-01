def extGcd (a, b):
    rPair = [a, b]
    sPair = [1, 0]
    tPair = [0, 1]
    while rPair[1] != 0:
        q = rPair[0] // rPair[1]
        rPair = [rPair[1], rPair[0] - q * rPair[1]]
        sPair = [sPair[1], sPair[0] - q * sPair[1]]
        tPair = [tPair[1], tPair[0] - q * tPair[1]]
    return [sPair[0], tPair[0]]

def getCombinedModulo (a, b):
    #print("{} {}".format(a, b))
    p = extGcd(a[0], b[0])
    #print("{} {} {} {}".format(a, b, (a[0] * b[0], p[0] * a[0] + p[1] * b[0])))
    return (a[0] * b[0], (p[0] * a[0] * b[1] + p[1] * b[0] * a[1]) % (a[0] * b[0]))

f = open("day13.txt")

t = int(f.readline())

ids = [(int(n), i) for i, n in enumerate(f.readline().split(',')) if n != 'x']

dt = t
busId = -1
for i, n in ids:
    if i - (t % i) < dt:
        dt = i - (t % i)
        busId = i

print("Bus id: {}. Time waiting: {}. Answer: {}".format(busId, dt, busId * dt))

l = [(i, (i - n) % i) for i,n in ids]
#print(ids)
#print(l)
import functools
#print(getCombinedModulo(l[0], l[1]))
m, n = functools.reduce(getCombinedModulo, l)

print("Min num: {} % {}".format(n, m))