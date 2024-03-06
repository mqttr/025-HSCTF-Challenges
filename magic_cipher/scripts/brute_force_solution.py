import subprocess

space = "!0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz"

cipher_to_plain = dict()
for first_char in space:
    for second_char in space:

        process = subprocess.Popen(f"./magic_cipher.exe -e {first_char + second_char}", stdout=subprocess.PIPE, stderr=subprocess.PIPE)

        result = process.stdout.read().splitlines()[1].decode().split()

        cipher_to_plain[f'{result[2]}'] = result[0]
    print(' '.join(result))

print(cipher_to_plain)

while True:
    print("What text would you like to decode?")

    password = input().strip()

    if len(password) % 2 == 1:
        print("Odd password not allowed. Try again.")
        continue

    for (first_char, second_char) in zip(password[0::2], password[1::2]):
        print(cipher_to_plain[first_char+second_char], end='')
    print()