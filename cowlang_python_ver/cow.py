import sys

class Cow:
    cells = {0: 0}    # Память
    current = 0       # Указатель на ячейку памяти
    register = None   # Регистр для MMM
    
    # Новые поля для управления исполнением
    ip = 0            # Указатель текущей инструкции (Instruction Pointer)
    tokens = []       # Список всех команд программы
    jumps = {}        # Карта переходов (где находятся парные скобки)

    # --- Операторы ---

    def MoO():
        Cow.cells[Cow.current] = Cow.cells.get(Cow.current, 0) + 1

    def MOo():
        Cow.cells[Cow.current] = Cow.cells.get(Cow.current, 0) - 1

    def moO():
        Cow.current += 1

    def mOo():
        Cow.current -= 1

    # --- Циклы теперь управляют IP сами ---

    def moo():
        # Если текущая ячейка 0, прыгаем СРАЗУ на парный MOO
        if Cow.cells.get(Cow.current, 0) == 0:
            # Проверяем, есть ли куда прыгать
            if Cow.ip in Cow.jumps:
                Cow.ip = Cow.jumps[Cow.ip]

    def MOO():
        # Если текущая ячейка НЕ 0, прыгаем НАЗАД на парный moo
        if Cow.cells.get(Cow.current, 0) != 0:
            if Cow.ip in Cow.jumps:
                Cow.ip = Cow.jumps[Cow.ip]

    # --- Остальные операторы ---

    def OOM():
        val = Cow.cells.get(Cow.current, 0)
        if val > 0: # Вывод символа
            print(chr(val), end='')
        else:       # Вывод числа
            print(val, end='')

    def oom():
        try:
            val = input("\nВвод: ")
            Cow.cells[Cow.current] = int(val)
        except ValueError:
            pass

    def OOO():
        Cow.cells[Cow.current] = 0

    def MMM():
        if Cow.register is None:
            Cow.register = Cow.cells.get(Cow.current, 0)
        else:
            Cow.cells[Cow.current] = Cow.register
            Cow.register = None

    def Moo():
        if Cow.cells.get(Cow.current, 0) == 0:
            Cow.oom()
        else:
            Cow.OOM()

    def mOO():
        # Выполнить команду по коду из ячейки
        code = Cow.cells.get(Cow.current, 0)
        ops = [
            Cow.moo, Cow.mOo, Cow.moO, Cow.mOO, 
            Cow.Moo, Cow.MOo, Cow.MoO, Cow.OOO, 
            Cow.MMM, Cow.OOM, Cow.oom, Cow.MOO
        ]
        if 0 <= code < len(ops):
            ops[code]()

    # --- Интерпретатор ---

    @staticmethod
    def run(source):
        # 1. Инициализация
        Cow.cells = {0: 0}
        Cow.current = 0
        Cow.register = None
        Cow.tokens = source.split()
        Cow.jumps = {}
        Cow.ip = 0
        
        # 2. Парсинг скобок (построение карты переходов)
        stack = []
        for i, cmd in enumerate(Cow.tokens):
            if cmd == 'moo':
                stack.append(i)
            elif cmd == 'MOO':
                if stack:
                    start = stack.pop()
                    Cow.jumps[start] = i # moo -> MOO
                    Cow.jumps[i] = start # MOO -> moo
        
        # Словарь действий
        actions = {
            "MoO": Cow.MoO, "MOo": Cow.MOo,
            "moO": Cow.moO, "mOo": Cow.mOo,
            "OOM": Cow.OOM, "oom": Cow.oom,
            "mOO": Cow.mOO, "Moo": Cow.Moo,
            "OOO": Cow.OOO, "MMM": Cow.MMM,
            "moo": Cow.moo, "MOO": Cow.MOO
        }

        # 3. Главный цикл исполнения
        while Cow.ip < len(Cow.tokens):
            cmd = Cow.tokens[Cow.ip]
            
            if cmd in actions:
                actions[cmd]() # Вызов метода (он может изменить Cow.ip!)
            
            # Переход к следующей инструкции
            # Если moo/MOO изменили ip, мы перейдем к (новому ip + 1)
            Cow.ip += 1
        
        print() # Финальный перенос строки