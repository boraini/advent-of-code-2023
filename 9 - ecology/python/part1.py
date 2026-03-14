from parsed_input import parsed_input

my_sum = 0
my_sum_left = 0
flag = False

def diff(a, b):
    global flag
    diff = b - a
    if diff != 0:
        flag = True
    return diff

for measurements in parsed_input:
    differences = measurements
    last_differences = []
    last_differences_left = []
    flag = True
    while flag:
        last_differences_left.append(differences[0])
        last_differences.append(differences[-1])
        flag = False
        differences = [diff(differences[i], differences[i + 1]) for i in range(len(differences) - 1)]
    my_sum += sum(last_differences)
    s = 0
    for x in reversed(last_differences_left):
        s = -s + x
    my_sum_left += s

print(f"Part 1: {my_sum}")
print(f"Part 2: {my_sum_left}")