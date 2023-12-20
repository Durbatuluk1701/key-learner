import os
import msvcrt
import random


def clear_screen():
    os.system("cls")


def get_char():
    return msvcrt.getch().decode("utf-8")


charList = ["n", "r", "t", "a"]

while True:
    clear_screen()
    pick = random.choice(charList)
    print(pick)
    char = get_char()
    while char != pick:
        if char == "!":
            exit(1)
        print("WRONG!")
        char = get_char()
