import re

with open("src/input", "r") as my_file:
    my_string = my_file.read()
    fixed_string = ""
    sum = 0
    for mat in re.findall(r"do\(\).*?(?:(?!don't\(\)).)*", my_string, re.DOTALL):
        fixed_string += mat
    print(fixed_string)
    for (num1, num2) in re.findall(r"mul\((\d+),(\d+)\)", fixed_string, re.DOTALL):
        sum += int(num1) * int(num2)
    print(sum)
