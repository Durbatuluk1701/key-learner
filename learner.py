import os
import msvcrt
import time
import random


def clear_screen():
    os.system("cls")


def get_char():
    return msvcrt.getch().decode("utf-8")


WINDOW = 5

charList = ["n", "r", "t", "a"]
charList += ["i", "e", "s", "o"]
# count = 1
# base_time = time.time()
# timer = time.time()

targ_buf = [random.choice(charList) for i in range(WINDOW)]


def pbuf(l):
    for i in l:
        print(i, end="")
    # print(f"\tWPM: {60 * count/(timer - base_time + 1)}")


while True:
    clear_screen()
    targ_buf.append(random.choice(charList))
    pick4 = random.choice(charList)
    print(targ_buf)
    # pbuf(targ_buf)
    char = get_char()
    # timer = time.time()
    while char != targ_buf[0]:
        if char == "!":
            exit(1)
        print("WRONG!")
        char = get_char()
        # timer = time.time()
    targ_buf.pop(0)
    # count += 1
