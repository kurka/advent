#!/usr/bin/env python3

import itertools
import collections

patterns_auto = collections.defaultdict(list)
for i in itertools.product("01", repeat=10):
    a = int("".join(i), base=2)
    b = a % 0b1000
    b = b ^ 0b0001
    c = a // pow(2, b)
    b = b ^ 0b101
    b = b ^ c
    b = b % 0b1000
    patterns_auto[b].append("".join(i))


# 0000
# 0010 00x0
# 0100 0100
# 0110 0xx0

patterns_merged = {}
for k in patterns_auto:
    consolidated_items: dict[i, list[str] | int] = {}
    for p1 in patterns_auto[k]:
        p1_number = int(p1, base=2)
        merged = False
        merged_pattern = []
        for i in range(10):
            pair = p1_number ^ pow(2, 9 - i)
            if pair in consolidated_items:
                merged_pattern.append("x")
                # replace item in pair merged_pattern
                consolidated_items[pair] = p1_number
            else:
                merged_pattern.append(p1[i])
            consolidated_items[p1_number] = merged_pattern

    patterns_merged[k] = sorted(
        list(
            set(
                map(
                    lambda x: "".join(x),
                    filter(lambda x: type(x) != int, consolidated_items.values()),
                )
            )
        )
    )


for k in sorted(list(patterns_merged.keys())):
    print(k)
    for l in patterns_merged[k]:
        print(l)


# x.xxx.xx1.000 -> 000 (0)
# x.xxx.110.010 -> 000 (0)
# x.x00.0xx.100 -> 000 (0)
# x.xx0.01x.101 -> 000 (0)
# 0.10x.xxx.110 -> 000 (0)
# x.011.xxx.111 -> 000 (0)
#
# x.xxx.111.010 -> 001 (1)
# x.xxx.x11.011 -> 001 (1)
# x.x00.1xx.100 -> 001 (1)
# x.xx0.00x.101 -> 001 (1)
# 0.11x.xxx.110 -> 001 (1)
# x.010.xxx.111 -> 001 (1)
#
# x.xxx.100.010 -> 010 (2)
# x.x01.0xx.100 -> 010 (2)
# x.xx0.11x.101 -> 010 (2)
# 0.00x.xxx.110 -> 010 (2) 010 xor abc
# x.001.xxx.111 -> 010 (2)
#
# x.xxx.101.010 -> 011 (3)
# x.xxx.x10.011 -> 011 (3)
# x.x01.1xx.100 -> 011 (3)
# x.xx0.10x.101 -> 011 (3)
# 0.01x.xxx.110 -> 011 (3)
# x.000.xxx.111 -> 011 (3)
#
# x.xxx.xx0.000 -> 100 (4)
# x.xxx.010.010 -> 100 (4)
# x.x10.0xx.100 -> 100 (4)
# x.xx1.01x.101 -> 100 (4)
# 1.10x.xxx.110 -> 100 (4)
# x.111.xxx.111 -> 100 (4)
#
# x.xxx.xxx.001 -> 101 (5)
# x.xxx.011.010 -> 101 (5)
# x.xxx.x01.011 -> 101 (5)
# x.x10.1xx.100 -> 101 (5)
# x.xx1.00x.101 -> 101 (5)
# 1.11x.xxx.110 -> 101 (5)
# x.110.xxx.111 -> 101 (5)
#
# x.xxx.000.010 -> 110 (6)  110 xor abc
# x.x11.0xx.100 -> 110 (6)
# x.xx1.11x.101 -> 110 (6)
# 1.00x.xxx.110 -> 110 (6)
# x.101.xxx.111 -> 110 (6)
#
# x.xxx.001.010 -> 111 (7)
# x.xxx.x00.011 -> 111 (7)
# x.x11.1xx.100 -> 111 (7)
# x.xx1.10x.101 -> 111 (7)
# 1.01x.xxx.110 -> 111 (7)
# x.100.xxx.111 -> 111 (7)
#
#
# order: 2,4,1,1,7,5,1,5,4,0,0,3,5,5,3,0
patterns = {
    0: [
        "010xxxx110",
        "x011xxx111",
        "xx000xx100",
        "xxx001x101",
        "xxxx110010",
        "xxxxxx1000",
    ],
    1: [
        "xxxx111010",
        "xxxxx11011",
        "xx001xx100",
        "xxx000x101",
        "011xxxx110",
        "x010xxx111",
    ],
    2: [
        "xxxx100010",
        "xx010xx100",
        "xxx011x101",
        "000xxxx110",
        "x001xxx111",
    ],
    3: [
        "xxxx101010",
        "xxxxx10011",
        "xx011xx100",
        "xxx010x101",
        "001xxxx110",
        "x000xxx111",
    ],
    4: [
        "xxxxxx0000",
        "xxxxxxx001",
        "xxxx010010",
        "xx100xx100",
        "xxx101x101",
        "110xxxx110",
        "x111xxx111",
    ],
    5: [
        "xxxx011010",
        "xxxxx01011",
        "xx101xx100",
        "xxx100x101",
        "111xxxx110",
        "x110xxx111",
    ],
    6: [
        "xxxx000010",
        "xx110xx100",
        "xxx111x101",
        "100xxxx110",
        "x101xxx111",
    ],
    7: [
        "xxxx001010",
        "xxxxx00011",
        "xx111xx100",
        "xxx110x101",
        "101xxxx110",
        "x100xxx111",
    ],
}

# patterns_merged = patterns
patterns_merged = patterns_auto
target_sequence = [2, 4, 1, 1, 7, 5, 1, 5, 4, 0, 0, 3, 5, 5, 3, 0]

queue = patterns_merged[2][:]
for target in target_sequence[1:]:
    print(f"Target: {target}")
    queue_size = len(queue)
    for candidate_i in range(queue_size):
        candidate = queue.pop(0)
        # print(f"evaluating {candidate}")
        for pattern in patterns_merged[target]:
            # print(f"{target} {pattern} {candidate}")
            # print(f"{target} {pattern[3:]} {candidate[:-3]}")
            if not all(
                a == "x" or b == "x" or a == b
                for a, b in zip(pattern[3:], candidate[:7], strict=True)
            ):
                # print(f"dead end: {candidate} {pattern}")
                continue
            common_part = []
            for a, b in zip(pattern[3:], candidate[:-3]):
                if a == "x":
                    common_part.append(b)
                else:
                    common_part.append(a)
            queue.append(pattern[:3] + "".join(common_part) + candidate[7:])
            print(f"Added: {queue[-1]}")
            print(f"Added: {queue[-1]}")


for q in sorted(set(queue), reverse=True):
    print(q, int(q.replace("x", "0"), base=2))
