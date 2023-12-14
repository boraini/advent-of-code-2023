from number_of_matches import number_of_matches

card_counts = [1 for _ in number_of_matches]

for i in range(len(number_of_matches)):
    for j in range(i + 1, min(i + number_of_matches[i] + 1, len(number_of_matches))):
        card_counts[j] += card_counts[i]

total = sum(card_counts)
print(f"Part 2 total score is {total}.")