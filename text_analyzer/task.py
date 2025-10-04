def count_lines(file):
    """Подсчитывает количество строк в файле"""
    return len(file.splitlines())

def count_chars(file):
    """Подсчитывает количество символов в файле"""
    return len(file)

def count_empty_lines(file):
    """Подсчитывает количество пустых строк"""
    lines = file.splitlines()
    empty_count = 0
    for line in lines:
        if line.strip() == '':
            empty_count += 1
    return empty_count

def char_freq(file):
    """Создает частотный словарь символов"""
    freq_dict = {}
    for char in file:
        if char in freq_dict:
            freq_dict[char] += 1
        else:
            freq_dict[char] = 1
    return freq_dict


file_name = input("Введите название файла: ")

file = open(file_name, 'r', encoding='utf-8')
file_content = file.read()

print("Вывести:")
print("1 - количество строк")
print("2 - количество символов")
print("3 - количество пустых строк")
print("4 - частотный словарь символов")

output_options = input().split()

if "1" in output_options:
    print(count_lines(file_content))

if "2" in output_options:
    print(count_chars(file_content))

if "3" in output_options:
    print(count_empty_lines(file_content))

if "4" in output_options:
    print(char_freq(file_content))

file.close()
