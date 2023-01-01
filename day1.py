import time


f = open("input1.txt")

t = time.process_time()
elves = f.read().split("\n\n")

cals = [sum(list(map(int, filter(None, cal.split("\n"))))) for cal in elves]
cals.sort(reverse=True)

elapsed_time = time.process_time() - t
print(elapsed_time)
print(max(cals))
print(sum(cals[:3]))


