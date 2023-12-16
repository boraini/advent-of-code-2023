from parsed_input import parsed_input, directions

with open("solution.kdt") as file:
    global solution

    solution = file.read()

with open("dist.kdt", "w") as file:
    n = len(parsed_input)
    m = len(directions)
    i = 0

    file.write(f"set n {n}\n")
    file.write(f"set m {m}\n")

    file.write(f"table input {n} 9\n")
    for key in sorted(parsed_input.keys()):
        is_a_left = 1 if parsed_input[key]['L'][-1] == 'A' else 0
        is_z_left = 1 if parsed_input[key]['L'][-1] == 'Z' else 0
        is_a_right = 1 if parsed_input[key]['R'][-1] == 'A' else 0
        is_z_right = 1 if parsed_input[key]['R'][-1] == 'Z' else 0
        is_a = 1 if key[-1] == 'A' else 0
        is_z = 1 if key[-1] == 'Z' else 0

        file.write(f"put input {i} 0 \"{key}\"\n")
        file.write(f"put input {i} 1 \"{parsed_input[key]['L']}\"\n")
        file.write(f"put input {i} 2 \"{parsed_input[key]['R']}\"\n")
        file.write(f"put input {i} 3 {is_a_left}\n")
        file.write(f"put input {i} 4 {is_z_left}\n")
        file.write(f"put input {i} 5 {is_a_right}\n")
        file.write(f"put input {i} 6 {is_z_right}\n")
        file.write(f"put input {i} 7 {is_a}\n")
        file.write(f"put input {i} 8 {is_z}\n")
        i += 1

    file.write(f"\ntable directions {m}\n")

    i = 0
    for d in directions:
        file.write(f"put directions {i} \"{d}\"\n")
        i += 1
    
    file.write(solution)
