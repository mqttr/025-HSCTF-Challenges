import random

secret = "This is my secret!!"


for _ in range(1_000_000):
    answer = random.randint(1, 10_000)

    while True: 
        equation = input(f"Write an equation that equals {answer}: ")

        try:
            if eval(equation) == answer:
                break
            else:
                print("WRONG!")
        except:
            print("Whatever you put in doesn't work.")


print(f"Hey you beat my game! You get the flag! {secret}")