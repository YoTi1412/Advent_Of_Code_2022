with open("input.txt") as f:
    rounds = [i for i in f.read().strip().split("\n")]

out = {
        "A X":4, "A Y":8, "A Z":3,
        "B X":1, "B Y":5, "B Z":9,
        "C X":7, "C Y":2, "C Z":6,
        }

total_pts = 0
for r in rounds:
    total_pts += out[r]

out2 = {
         "A X":3, "A Y":4, "A Z":8,
         "B X":1, "B Y":5, "B Z":9,
         "C X":2, "C Y":6, "C Z":7,
         }


total_pts2 = 0
for r in rounds:
    total_pts2 += out2[r]




print("Answer of Part one : ", total_pts)
print("Answer of Part two : ", total_pts2)
