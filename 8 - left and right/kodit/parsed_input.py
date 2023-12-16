parsed_input = {}

with open("../input.txt") as file:
    global directions
    line_count = 0
    for line in file:
        if line_count == 0:
            directions = [c for c in line.strip()]
        elif line_count == 1:
            pass
        else:
            [key, value] = line.strip().split(" = ")
            [left, right] = value.split(", ")
            left = left[len("("):]
            right = right[:-len(")")]
            parsed_input[key] = {'L': left, 'R': right}

            line_count += 2
        line_count += 1