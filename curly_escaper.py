import time

lines = ""
while True:
    try:
        line = input().replace("{", "{{").replace("}", "}}")
        lines += line + "\n"
    except EOFError:
        time.sleep(0.1)
        break

print(lines)
