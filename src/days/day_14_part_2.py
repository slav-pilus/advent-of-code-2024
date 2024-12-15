import re

class Cell:
    def __init__(self, x, y):
        self.robots = []

    def __str__(self):
        return str(len(self.robots)) if len(self.robots) > 0 else "."

    def __repr__(self):
        return self.__str__()

class Robot:
    def __init__(self, right, down):
        self.right = right
        self.down = down

    def __str__(self):
        return str(self.right) + "," + str(self.down)

    def __repr__(self):
        return self.__str__()

def count_quadrants(grid):
    middle_row = len(grid) // 2
    middle_col = len(grid[0]) // 2

    q1 = 0
    for row in range(len(grid)):
        for col in range(len(grid[row])):
            if row < middle_row and col < middle_col:
                if len(grid[row][col].robots) > 0:
                    q1 += len(grid[row][col].robots)

    q2 = 0
    for row in range(len(grid)):
        for col in range(len(grid[row])):
            if row < middle_row and col > middle_col:
                if len(grid[row][col].robots) > 0:
                    q2 += len(grid[row][col].robots)

    q3 = 0
    for row in range(len(grid)):
        for col in range(len(grid[row])):
            if row > middle_row and col < middle_col:
                if len(grid[row][col].robots) > 0:
                    q3 += len(grid[row][col].robots)

    q4 = 0
    for row in range(len(grid)):
        for col in range(len(grid[row])):
            if row > middle_row and col > middle_col:
                if len(grid[row][col].robots) > 0:
                    q4 += len(grid[row][col].robots)

    return q1, q2, q3, q4

def int_grid(y, x):
    grid = []
    for i in range(x):
        row = []
        for j in range(y):
            row.append(Cell(i, j))

        grid.append(row)

    return grid

def print_grid(grid):
    for row in grid:
        print(' '.join(map(str, row)))

def step(r_row, c_col, r_col, c_row):
    new_col = (c_col + r_row) % wide
    new_row = (c_row + r_col) % tall
    return new_row, new_col

tall = 103
wide = 101
seconds = 100

grid = int_grid(wide, tall)

with open('../../input/day14.txt', 'r') as file:
    file_string = file.read()
    row_list = []
    for line in file_string.split('\n'):
        elements = line.split(' ')
        y, x = elements[0].split('=')[1].split(',')
        v_r, v_d = elements[1].split('=')[1].split(',')
        robot = Robot(int(v_r), int(v_d))
        grid[int(x)][int(y)].robots.append(robot)

found = False
def has_christmas_tree(grid):
    for row in range(len(grid)):
        r = str(grid[row]).replace(",", "").replace(" ", "")
        if bool(re.search(r'\b\d{9}\b', r)):
            return True

    return False

steps = 0
while not found:
    steps += 1
    new_grid = int_grid(wide, tall)
    seconds -= 1
    for row in range(len(grid)):
        for col in range(len(grid[row])):
                for robot in grid[row][col].robots:
                    new_location = step(robot.right, col, robot.down, row)
                    new_grid[new_location[0]][new_location[1]].robots.append(robot)

    grid = new_grid
    if has_christmas_tree(new_grid):
        found = True
        print("found after " + str(steps) + " steps")

print_grid(grid)