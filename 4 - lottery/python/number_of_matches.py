from parsed_input import parsed_input

number_of_matches = [sum(1 if x in card["winning_numbers"] else 0 for x in card["your_numbers"]) for card in parsed_input]