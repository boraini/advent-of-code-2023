with open("../input.txt") as file:
    global parsed_input

    parsed_input = [[int(x) for x in line.strip().split(" ")] for line in file]