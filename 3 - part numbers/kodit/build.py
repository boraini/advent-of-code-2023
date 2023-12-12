from parsed_lines import parsed_lines

with open("solution.kdt", "r") as file:
    global solution

    solution = file.read()

with open("dist.kdt", "w") as dist:
    dist.write(f"set rows {len(parsed_lines)}\n")
    dist.write(f"set cols {len(parsed_lines[0])}\n")
    dist.write(f"table input {len(parsed_lines)} {len(parsed_lines[0])}\n")
    for i in range(len(parsed_lines)):
        for j in range(len(parsed_lines[0])):
            dist.write(f"put input {i} {j} \"{parsed_lines[i][j]}\"\n")

    dist.write("\n")

    dist.write(solution)

