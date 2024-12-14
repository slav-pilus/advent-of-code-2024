stones = "125 17".split(" ")
no_of_blinks_left = 25

print(stones)

while no_of_blinks_left != 0:
    current_iter = []
    no_of_blinks_left -=1
    for element in range(0, len(stones)):
        current = stones[element]
        if current == "0":
            current_iter.append("1")
        elif len(current) %2 != 0:
            as_num = int(current)
            current_iter.append(str(as_num * 2024))
        else:
            half = len(current)//2
            left = current[:half]
            right = current[half:]
            current_iter.append(left)
            current_iter.append(str(int(right)))

    stones = current_iter

print(len(stones))


