def print_grid(grid):
    for row in grid:
        print(' '.join(map(str, row)))

grid = []
moves = []
current_loc = (0, 0)

with open('../../input/day15-test.txt', 'r') as file:
    file_string = file.read()

    for line in file_string.split('\n'):
        if line.startswith('#'):
            row = []
            for i in range(len(line)):
                if line[i] == '@':
                    current_loc = (i, len(grid))
                row.append(line[i])
            grid.append(row)
        else:
            for i in range(len(line)):
                moves.append(line[i])

def find_robot(grid):
    for i in range(len(grid)):
        for j in range(len(grid[i])):
            if grid[i][j] == '@':
                return (i, j)

def rotate_90(matrix):
    return [list(row) for row in zip(*matrix[::-1])]

def rotate_180(matrix):
    return [row[::-1] for row in matrix[::-1]]


def rotate_270(matrix):
    return [list(row) for row in zip(*matrix)][::-1]

def push_right(lst, start_idx):
    if lst[start_idx] != 'O':
        return lst

    idx_O = lst.index('O', start_idx)
    idx_dot = None
    for i in range(idx_O + 1, len(lst)):
        if lst[i] == '#':
            break
        if lst[i] == '.':
            idx_dot = i
            break

    if idx_dot is not None:
        lst[idx_O], lst[idx_dot] = lst[idx_dot], lst[idx_O]
        lst[start_idx] = '@'
    else:
        lst[start_idx - 1] = '@'

    return lst


def make_move(move, grid):
    if move == '<':
        grid = rotate_180(grid)
    if move == '^':
        grid = rotate_90(grid)
    if move == 'v':
        grid = rotate_270(grid)

    current_loc = find_robot(grid)
    grid[current_loc[0]][current_loc[1]] = '.'
    if grid[current_loc[0]][current_loc[1] + 1] == '#':
        grid[current_loc[0]][current_loc[1]] = '@'
    if grid[current_loc[0]][current_loc[1] + 1] == '.':
        grid[current_loc[0]][current_loc[1] + 1] = '@'
    else:
        current_row = grid[current_loc[0]]
        push_right(current_row, current_loc[1] + 1)

    if move == '<':
        grid = rotate_180(grid)
    if move == '^':
        grid = rotate_270(grid)
    if move == 'v':
        grid = rotate_90(grid)

    return grid

for move in moves:
    grid = make_move(move, grid)

result = 0
for row in range(len(grid)):
    for col in range(len(grid[row])):
        if grid[row][col] == 'O':
            result += 100 * row + col

print_grid(grid)
print(result)