class CupLink:
    def __init__ (self, num):
        self.num = num
        self.next = None
    
f = open("day23.txt")
s = f.readline()
cups = [int(i) for i in s.strip()]
small = min(cups)
big = max(cups)
n = 0
#print(cups)
while n < 100:
    curr3 = cups[1:4]
    cups.pop(1)
    cups.pop(1)
    cups.pop(1)
    dest = cups[0] - 1
    #print("{} {}".format(dest, cups))
    while dest not in cups:
        dest -= 1
        if dest < small:
            dest = big
    for i, cup in enumerate(curr3):
        cups.insert(cups.index(dest) + i + 1, cup)
    curr = cups.pop(0)
    cups.append(curr)
    #print(cups)
    n += 1

while cups[0] != 1:
    curr = cups.pop(0)
    cups.append(curr)

print("Labels: {}".format("".join([str(i) for i in cups[1:]])))

cups = [int(i) for i in s.strip()] + list(range(10, 1000001))
#cups = [int(i) for i in s.strip()]
cupLinks = [CupLink(i) for i in range(1, 1000001)]
#cupLinks = [CupLink(i) for i in range(1, 10)]
for i in range(1, len(cups)):
    cupLinks[cups[i - 1] - 1].next = cupLinks[cups[i] - 1]
#print(cups)
cupLinks[cups[-1] - 1].next = cupLinks[cups[0] - 1]
small = 1
big = 1000000
#big = 9
n = 0
currCup = cupLinks[cups[0] - 1]
while n < 10000000:
    #for i in cupLinks:
        #print("{}->{}".format(i.num, i.next.num))
    #print("currCup: {}".format(currCup.num))
    curr3 = currCup.next
    #print("curr3: {}".format(curr3.num))
    end = curr3.next.next
    #print("end: {}".format(end.num))
    currCup.next = end.next
    end.next = None
    c3cups = []
    c = curr3
    while c != None:
        c3cups.append(c.num)
        c = c.next
    target = currCup.num - 1
    if target < small:
        target = big
    while target in c3cups:
        target -= 1
        if target < small:
            target = big
    tmp = cupLinks[target - 1].next
    cupLinks[target - 1].next = curr3
    end.next = tmp
    n += 1
    currCup = currCup.next
    #print()

#for i in cupLinks:
    #print("{}->{}".format(i.num, i.next.num))

print("Labels 2: {} * {} = {}".format(cupLinks[0].next.num, cupLinks[0].next.next.num, cupLinks[0].next.num * cupLinks[0].next.next.num))