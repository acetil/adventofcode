f = open("day03.txt")

lines = f.readlines()

bitlen = len(lines[0].strip())

nums = [int(i, base=2) for i in lines]

gamma = 0

for i in range(0, bitlen):
    n = sum((j >> i) & 1 for j in nums)
    gamma |= (n >= len(nums) / 2) << i

print(f"Power consumption: {gamma * (gamma ^ ((1 << bitlen) - 1))}")

oxygenRating = 0
co2Rating = 0

for i in range(1, bitlen + 1):
    oxNums = [j >> (bitlen - i) & 1 for j in nums if (j >> (bitlen - i + 1)) == oxygenRating]
    co2Nums = [j >> (bitlen - i) & 1 for j in nums if (j >> (bitlen - i + 1)) == co2Rating]
    
    oxygenRating = oxygenRating << 1 | (sum(oxNums) >= len(oxNums) / 2)
    co2Rating = co2Rating << 1 | ((sum(co2Nums) < len(co2Nums) / 2 and sum(co2Nums) != 0) or sum(co2Nums) == len(co2Nums))

print(f"Oxygen rating: {oxygenRating}")
print(f"CO2 rating: {co2Rating}")
print(f"Life support rating: {oxygenRating * co2Rating}")