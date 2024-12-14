import uuid

class Node:
    def __init__(self, value):
        self.value = value
        self.children = []
        self.id = str(uuid.uuid4())

    def __str__(self):
        return str(self.value) + " | " + str(len(self.children)) + " | " + self.id

    def __repr__(self):
        return self.__str__()


def find_path(n):
    leaves = []
    if n.value == 9:
        return [n.id]
    elif len(n.children) == 0:
        return []
    for child in n.children:
        leaves.extend(find_path(child))

    return leaves


grid = []
with open('../../input/day10.txt', 'r') as file:
    file_string = file.read()
    row_list = []
    for line in file_string.split('\n'):
        for char in line:
            row_list.append(Node(int(char)))
        grid.append(row_list)
        row_list = []

nodes = []
for row in range(len(grid)):
    for col in range(len(grid[row])):
        current = grid[row][col]
        if row > 0 and grid[row - 1][col].value == current.value + 1:
            current.children.append(grid[row - 1][col])
        if row < len(grid) - 1 and grid[row + 1][col].value == current.value + 1:
            current.children.append(grid[row + 1][col])
        if col > 0 and grid[row][col - 1].value == current.value + 1:
            current.children.append(grid[row][col - 1])
        if col < len(grid[row]) - 1 and grid[row][col + 1].value == current.value + 1:
            current.children.append(grid[row][col + 1])
        nodes.append(current)

unique_paths = []
for node in nodes:
    if node.value == 0:
        unique_paths.extend(list(set(find_path(node))))

print("part 1: " + str(len(unique_paths)))

all_paths = []
for node in nodes:
    if node.value == 0:
        all_paths.extend(list(find_path(node)))

print("part 2: " + str(len(all_paths)))
