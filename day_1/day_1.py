with open('input.txt') as f:

    data = [i for i in f.read().strip().split("\n")]

max_1 = 0
max_2 = 0
max_3 = 0
count = 0

for item in data:
    if item == '':
        count = 0

    else:
        num = int(item)
        count += num

    if count > max_1:
        max_1 = count

    elif count > max_2:
        max_2 = count

    elif count > max_3:
        max_3 = count

print("Answer of Part 1 : ", max_1)
print("Answer of Part 2 : ", max_1 + max_2 + max_3)

