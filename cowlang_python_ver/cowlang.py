import sys

class Cow:
    def __init__(self, code):
        self.code = code
        self.cells = {0: 0}
        self.current = 0       
        self.register = None   
        self.ip = 0            
        self.instructions = code.split()
        self.loop_map = {}     

        self._build_loop_map()

    def _build_loop_map(self):
        """
        Строим карту прыжков. 
        Согласно спецификации (картинке):
        MOO - начало цикла (push в стек)
        moo - конец цикла (pop из стека)
        """
        stack = []
        for i, command in enumerate(self.instructions):
            if command == "MOO":
                # MOO - это "Открывающая скобка"
                stack.append(i)
            elif command == "moo":
                # moo - это "Закрывающая скобка"
                if not stack:
                    raise ValueError(f"Ошибка синтаксиса: moo без MOO (команда №{i+1})")
                start = stack.pop()
                self.loop_map[start] = i
                self.loop_map[i] = start
        
        if stack:
            raise ValueError("Ошибка синтаксиса: MOO без moo (незакрытый цикл)")

    def _get_val(self):
        return self.cells.setdefault(self.current, 0)

    # --- Команды ---

    def MoO(self):
        """Значение текущей ячейки увеличить на 1"""
        self.cells[self.current] = self._get_val() + 1

    def MOo(self):
        """Значение текущей ячейки уменьшить на 1"""
        self.cells[self.current] = self._get_val() - 1

    def moO(self):
        """Сдвиг вправо"""
        self.current += 1

    def mOo(self):
        """Сдвиг влево"""
        self.current -= 1

    def MOO(self):
        """
        НАЧАЛО ЦИКЛА (Conditional Loop Start).
        Описание с картинки:
        Если текущее значение == 0, пропустить следующую команду и продолжить 
        выполнение ПОСЛЕ соответствующего moo.
        """
        if self._get_val() == 0:
            # Прыгаем к закрывающему moo. 
            # После завершения такта цикла run(), ip увеличится на 1,
            # и мы окажемся на команде, следующей сразу за moo.
            self.ip = self.loop_map[self.ip]

    def moo(self):
        """
        КОНЕЦ ЦИКЛА (Loop End / Repeat).
        Описание с картинки:
        Ищет назад соответствующий MOO и начинает выполнение снова С НЕГО.
        """
        # Прыгаем назад к MOO.
        # ВАЖНО: Мы ставим ip на (адрес MOO - 1).
        # Почему? Потому что в конце цикла run() произойдет self.ip += 1.
        # В итоге ip станет равен адресу MOO, и в следующем такте выполнится команда MOO (проверка условия).
        self.ip = self.loop_map[self.ip] - 1

    def OOM(self):
        """Вывод значения как числа"""
        print(self._get_val())

    def oom(self):
        """Ввод значения (целое число)"""
        try:
            print("Ввод числа > ", end='')
            val = int(input())
            self.cells[self.current] = val
        except ValueError:
            pass

    def mOO(self):
        """Выполняет функцию с кодом (сложная команда, пока пропуск)"""
        pass

    def Moo(self):
        """Если 0 - ввод, иначе вывод символа (ASCII)"""
        val = self._get_val()
        if val == 0:
            self.oom()
        else:
            print(chr(val), end='') 

    def OOO(self):
        """Обнулить ячейку"""
        self.cells[self.current] = 0

    def MMM(self):
        """Копирование в/из регистра"""
        if self.register is None:
            self.register = self._get_val()
        else:
            self.cells[self.current] = self.register
            self.register = None

    def run(self):
        moo_map = {
            "MoO": self.MoO,
            "MOo": self.MOo,
            "moO": self.moO,
            "mOo": self.mOo,
            "moo": self.moo, # Теперь это конец цикла
            "MOO": self.MOO, # Теперь это начало цикла
            "OOM": self.OOM,
            "oom": self.oom,
            "mOO": self.mOO,
            "Moo": self.Moo,
            "OOO": self.OOO,
            "MMM": self.MMM
        }

        while self.ip < len(self.instructions):
            cmd = self.instructions[self.ip]
            if cmd in moo_map:
                moo_map[cmd]()
            self.ip += 1

# --- Тестирование ---

# Теперь используем ваш ИСХОДНЫЙ пример кода, который падал.
# С новой логикой (где MOO открывает цикл) он должен отработать,
# если там действительно парные скобки.

code_example = """ MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO
 MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO
 MoO MoO Moo MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO Moo MoO MoO
 MoO MoO MoO MoO MoO Moo Moo MoO MoO MoO Moo OOO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO
 MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO Moo MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO
 MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO
 MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO
 MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO Moo MOo
 MOo MOo MOo MOo MOo MOo MOo MOo MOo MOo MOo MOo MOo MOo MOo MOo MOo MOo MOo MOo MOo MOo MOo MOo MOo MOo MOo MOo MOo MOo MOo MOo MOo MOo MOo
 MOo MOo MOo MOo MOo Moo MOo MOo MOo MOo MOo MOo MOo MOo Moo MoO MoO MoO Moo MOo MOo MOo MOo MOo MOo Moo MOo MOo MOo MOo MOo MOo MOo MOo Moo
 OOO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO MoO Moo """

print("Запуск интерпретатора...")
try:
    obj = Cow(code_example)
    obj.run()
    print("\n\nПрограмма выполнена успешно.")
except Exception as e:
    print(f"\nОшибка выполнения: {e}")
