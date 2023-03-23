# Getting Data from input.txt file

with open('input.txt') as f:

    data = [i for i in f.read().strip().split("\n")]

    # print(data)

max = 0
count = 0

for item in data:
    if item == '':
        count = 0
    else:
        num = int(item)
        count += num

    if count > max:
        max = count

    print("The total of Calories carried by one ELF is :", max)
