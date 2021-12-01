import re
f = open("day4.txt")

l = f.readlines()

passports = [[]]

for i in l:
    if i == "\n":
        passports.append([])
    l1 = [tuple([k.strip() for k in j.split(":")]) for j in i.split(" ")]
    passports[-1] += l1

if passports[-1] == []:
    passports.pop()

reqs = set(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"])

result = 0
for i in passports:
    #print(i)
    if reqs.intersection(set([j[0] for j in i])) != reqs:
        continue
    l = i[:]
    if l[0][0] == "":
        l.pop(0)
    #print(l)
    for j in l:
        if j[0] == "byr" and not (int(j[1]) >= 1920 and int(j[1]) <= 2002):
            break
        if j[0] == "iyr" and not (int(j[1]) >= 2010 and int(j[1]) <= 2020):
            break
        if j[0] == "eyr" and not (int(j[1]) >= 2020 and int(j[1]) <= 2030):
            break
        if j[0] == "hgt":
            if re.fullmatch(r"\d+\w\w", j[1]) == None:
                break
            s = j[1][len(j[1]) - 2:]
            if not(s == "cm" or s == "in"):
                break
            n = int(j[1][:len(j[1]) - 2])
            if s == "cm" and not (n >= 150 and n <= 193):
                break
            if s == "in" and not (n >= 59 and n <= 76):
                break
        if j[0] == "hcl" and re.fullmatch(r"#[0-9a-f]{6}", j[1]) == None:
            break
        if j[0] == "ecl" and j[1] not in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]:
            break
        if j[0] == "pid" and re.fullmatch(r"\d{9}", j[1]) == None:
            break
    else:
        result += 1


print("Valid: {}".format(result))
#print(passports)