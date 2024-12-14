s = "125 17".split(" ")

def get_stones_after_x_blinks(stone, blinks):
    stones = [stone]
    for i in range(0, blinks):
        current_iter = []
        for element in range(0, len(stones)):
            current = stones[element]
            if current == "0":
                current_iter.append("1")
            elif len(current) % 2 != 0:
                as_num = int(current)
                current_iter.append(str(as_num * 2024))
            else:
                half = len(current) // 2
                left = current[:half]
                right = current[half:]
                current_iter.append(left)
                current_iter.append(str(int(right)))

        stones = current_iter

    return stones

after_x = []
for stone in s:
    after_x.extend(get_stones_after_x_blinks(stone, 10))

lookup = {}
out = 0
for element in range(0, len(after_x)):
    current = after_x[element]
    if lookup.get(current) is None:
        size = len(get_stones_after_x_blinks(current, 15))
        lookup[current] = size
        out += size
    else:
        out += lookup[current]

print(out)