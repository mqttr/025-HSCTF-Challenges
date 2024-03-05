import random

ordered_space = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890_!"

ran = ''.join(random.sample(ordered_space, len(ordered_space)))

print(len(ran) == len(ordered_space))
print(ran)