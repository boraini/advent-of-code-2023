parsed_input = [];

with open("../input.txt") as file:
    for line in file:
        [card_part, numbers_part] = line.split(": ")
        number = int(card_part.split()[1])
        [winning_numbers, your_numbers] = numbers_part.split(" | ")
        winning_numbers = [int(x) for x in winning_numbers.split()]
        your_numbers = [int(x) for x in your_numbers.split()]
        parsed_input.append({
            "winning_numbers": winning_numbers,
            "your_numbers": your_numbers,
            "number": number
        })