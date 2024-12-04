#!/usr/bin/env python3

with open("inputs/02.txt", "r") as f:
    lines = f.readlines()

seqs = [list(map(int, line.split())) for line in lines]

# part a
valid_sols = 0
for seq in seqs:
    prev = seq[0]
    decreasing = seq[1] < seq[0]
    valid = True
    for el in seq[1:]:
        diff = el - prev
        if abs(diff) > 3 or diff == 0 or (diff < 0) != decreasing:
            valid = False
            break
        prev = el
    if valid:
        valid_sols += 1

print(valid_sols)


# part b
def check_valid(
    seq: list[int], i: int, prev_i: int, decreasing: bool, forgived
) -> bool:
    if i >= len(seq):
        return True

    diff = seq[i] - seq[prev_i]

    if abs(diff) > 3 or diff == 0 or (diff < 0) != decreasing:
        if not forgived:
            match prev_i:
                case 0:
                    return check_valid(
                        seq, i + 1, prev_i, seq[i + 1] < seq[prev_i], True
                    ) or check_valid(seq, i + 1, i, seq[i + 1] < seq[i], True)
                case 1:
                    return (
                        check_valid(seq, i + 1, prev_i, decreasing, True)
                        or check_valid(seq, i, i - 2, decreasing, True)
                        or check_valid(seq, i, prev_i, seq[i] < seq[prev_i], True)
                    )
                case _:
                    return check_valid(seq, i + 1, prev_i, decreasing, True) or (
                        check_valid(seq, i, i - 2, decreasing, True)
                    )
        else:
            return False
    else:
        return check_valid(seq, i + 1, i, decreasing, forgived)


print(sum(check_valid(seq, 1, 0, seq[1] < seq[0], False) for seq in seqs))
