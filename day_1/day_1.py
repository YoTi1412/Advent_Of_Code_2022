with open("input.txt", "r") as f:
    lines = [line.strip() for line in f.readlines()]

totals = {}
current_elf = 1
for line in lines:
    if line == "":
        current_elf += 1
    else:
        Calories = int(line)
        if current_elf not in totals:
            totals[current_elf] = 0
        totals[current_elf] += Calories

sorted_totals = sorted(totals.items(), key=lambda x: x[1], reverse=True)

top_three_totals = sum([x[1] for x in sorted_totals[:3]])

print("Answer of Part One:", sorted_totals[0][1])
print("Answer of Part Two:", top_three_totals)

