from parsed_lines import parsed_lines

def get_number_span(parsed_lines, rows, cols, r, c):
    start = 0
    end = rows
    for i in range(c, -1, -1):
        if not parsed_lines[r][i].isdigit():
            start = i + 1
            break

    for i in range(c, cols, 1):
        if not parsed_lines[r][i].isdigit():
            end = i
            break
    
    return (r, start, end)

def get_number(parsed_lines, span):
    return int("".join(parsed_lines[span[0]][span[1]:span[2]]))

neighbours = [
    (-1, -1), (0, -1), (1, -1),
    (-1, 0), (1, 0),
    (-1, 1), (0, 1), (1, 1),
]

rows = len(parsed_lines)
cols = len(parsed_lines[0])

sum = 0
for r in range(rows):
    for c in range(cols):
        if (parsed_lines[r][c] == '*'):
            neighbouring_spans = set()
            for neighbour in neighbours:
                nr = r + neighbour[0]
                nc = c + neighbour[1]

                if nr >= 0 and nr < rows and nc >= 0 and nc < cols:
                    if parsed_lines[nr][nc].isdigit():
                        neighbouring_spans.add(get_number_span(parsed_lines, rows, cols, nr, nc))

            if len(neighbouring_spans) == 2:
                numbers = [get_number(parsed_lines, span) for span in neighbouring_spans]
                sum += numbers[0] * numbers[1]

print(f"The sum is {sum}.")