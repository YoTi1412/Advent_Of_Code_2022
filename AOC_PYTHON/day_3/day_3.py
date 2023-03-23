from string import ascii_letters

with open("input.txt") as f:
    rucksack = [i for i in f.read().strip().split("\n")]


totalSum = 0
for r in rucksack:
    half = len(r)//2
    left = set(r[:half])
    right = set(r[half:])

    for priority, char in enumerate(ascii_letters):
        if char in left and char in right:
            totalSum += priority +1

print("Answer of part one is :", totalSum)



j = 3
totalSum = 0
for i in range(0, len(rucksack), 3):
    r = rucksack[i:j]
    j += 3

    for priority, char in enumerate(ascii_letters):
        if char in r[0] and char in r[1] and char in r[2]:
            totalSum += priority + 1

print("Answer of part two is :", totalSum)











