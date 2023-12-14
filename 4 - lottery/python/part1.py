from number_of_matches import number_of_matches

total = 0
for card_count in number_of_matches:
    card_score = 2 ** (card_count - 1) if card_count > 0 else 0
    total += card_score

print(f"Part 1 total score is {total}.")