import re
import difflib


file_path = 'parse-trace-macro-input.txt'  # Replace with the actual file path

with open(file_path, 'r') as file:
    input_string = file.read()

# Extract multiline strings from the input using regular expressions
pattern = r'= note: expanding `([^`]*)`(?:\n.*= note: to `([^`]*)`)*'
matches = re.finditer(pattern, input_string)

multilines = []
for match in matches:
    multilines.extend(match.groups())

# Calculate and display differences between expansion steps
for i in range(1, len(multilines), 2):
    from_str = multilines[i - 1]
    to_str = multilines[i]

    diff = difflib.ndiff(to_str.splitlines(), from_str.splitlines())
    print(f"Diff from\n'''\n{from_str}\n'''\nto\n'''\n{to_str}\n'''")
    print(":" * 50)
    print('\n'.join(diff))
    print("=" * 100)


