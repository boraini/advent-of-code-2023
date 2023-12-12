parsed_lines = []

with open("../input.txt", "r") as file:
    for line in file:
        splitted = [*line]
        splitted = filter(lambda c: c != '\r' and c != '\n', splitted)
        parsed_lines.append(list(splitted))