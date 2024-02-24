
secret = "This is my secret!!"

usr = input().strip()

if eval(f"f'{usr}' == 'password'"):
    print(secret)