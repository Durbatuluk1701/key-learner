import os
import msvcrt
import time
import random


def clear_screen():
    os.system("cls")


def get_char():
    return msvcrt.getch().decode("utf-8")


# charList = ["n", "r", "t", "a"]
charList = ["i", "e", "s", "o"]
count = 1
base_time = time.time()
timer = time.time()

while True:
    clear_screen()
    pick1 = random.choice(charList)
    pick2 = random.choice(charList)
    pick3 = random.choice(charList)
    pick4 = random.choice(charList)
    print(
        pick1 + pick2 + pick3 + pick4 + f"\tWPM: {count/((timer - base_time + 1)/60)}"
    )
    char1 = get_char()
    char2 = get_char()
    char3 = get_char()
    char4 = get_char()
    timer = time.time()
    while char1 != pick1 or char2 != pick2 or char3 != pick3 or char4 != pick4:
        if char1 == "!" and char2 == "!" and char3 == "!" and char4 == "!":
            exit(1)
        print("WRONG!")
        char1 = get_char()
        char2 = get_char()
        char3 = get_char()
        char4 = get_char()
        timer = time.time()
    count += 1
