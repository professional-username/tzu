import re

with open("artofwar-raw.txt", "r") as file:
    text = file.read()

new_text = ""
for line in text.splitlines():
    try:
        int(line[0])
        line = line[line.index(" ") + 1 :]
        new_text += line + "\n"
    except:
        continue
text = new_text

print(text[:100])

with open("artofwar.txt", "w") as file:
    file.write(text)
