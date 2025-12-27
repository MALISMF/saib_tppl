use std::collections::HashMap;
use std::io::{self, Write};

/// Структура, представляющая интерпретатор языка COW
#[derive(Debug)]
pub struct Cow {
    /// Хранилище ячеек памяти (адрес -> значение)
    pub cells: HashMap<i32, i32>,
    /// Текущая позиция указателя в памяти
    pub current: i32,
    /// Регистр для временного хранения значения (используется командой MMM)
    pub register: Option<i32>,
    /// Индекс текущей инструкции (instruction pointer)
    pub ip: usize,
    /// Список всех инструкций программы
    pub instructions: Vec<String>,
    /// Карта соответствий между началом и концом циклов (MOO <-> moo)
    pub loop_map: HashMap<usize, usize>,
}

impl Cow {
    /// Создает новый интерпретатор COW из исходного кода
    /// 
    /// # Аргументы
    /// * `code` - строка с исходным кодом на языке COW
    /// 
    /// # Возвращает
    /// * `Ok(Cow)` - успешно созданный интерпретатор
    /// * `Err(String)` - ошибка при парсинге (несоответствие циклов)
    pub fn new(code: String) -> Result<Self, String> {
        // Разбиваем код на отдельные инструкции по пробелам
        let instructions: Vec<String> = code
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        // Инициализируем память с нулевой ячейкой
        let mut cells = HashMap::new();
        cells.insert(0, 0);

        let mut cow = Cow {
            cells,
            current: 0,
            register: None,
            ip: 0,
            instructions,
            loop_map: HashMap::new(),
        };

        // Строим карту циклов для быстрого перехода между MOO и moo
        cow.build_loop_map()?;
        Ok(cow)
    }

    /// Строит карту соответствий между началом и концом циклов
    /// Использует стек для сопоставления пар MOO (начало) и moo (конец)
    /// 
    /// # Возвращает
    /// * `Ok(())` - карта успешно построена
    /// * `Err(String)` - ошибка синтаксиса (несоответствие циклов)
    pub fn build_loop_map(&mut self) -> Result<(), String> {
        let mut stack: Vec<usize> = Vec::new();

        // Проходим по всем инструкциям и находим пары MOO/moo
        for (i, command) in self.instructions.iter().enumerate() {
            if command == "MOO" {
                // Начало цикла - сохраняем позицию в стек
                stack.push(i);
            } else if command == "moo" {
                // Конец цикла - связываем с последним MOO из стека
                if stack.is_empty() {
                    return Err("Ошибка синтаксиса: moo без MOO".to_string());
                }
                let start = stack.pop().unwrap();
                // Создаем двустороннюю связь для перехода в обе стороны
                self.loop_map.insert(start, i);
                self.loop_map.insert(i, start);
            }
        }

        // Проверяем, что все циклы закрыты
        if !stack.is_empty() {
            return Err("Ошибка синтаксиса: MOO без moo".to_string());
        }

        Ok(())
    }

    /// Получает значение текущей ячейки памяти
    /// Если ячейка не существует, возвращает 0
    pub fn get_val(&self) -> i32 {
        *self.cells.get(&self.current).unwrap_or(&0)
    }

    /// Устанавливает значение текущей ячейки памяти
    pub fn set_val(&mut self, val: i32) {
        self.cells.insert(self.current, val);
    }

    /// Команда MoO: увеличивает значение текущей ячейки на 1
    pub fn moo_cmd(&mut self) {
        self.set_val(self.get_val() + 1);
    }

    /// Команда MOo: уменьшает значение текущей ячейки на 1
    pub fn moo_cmd_2(&mut self) {
        self.set_val(self.get_val() - 1);
    }

    /// Команда moO: перемещает указатель вправо (увеличивает current)
    /// Автоматически инициализирует новую ячейку нулем, если её нет
    pub fn moo_cmd_3(&mut self) {
        self.current += 1;
        if !self.cells.contains_key(&self.current) {
            self.cells.insert(self.current, 0);
        }
    }

    /// Команда mOo: перемещает указатель влево (уменьшает current)
    /// Автоматически инициализирует новую ячейку нулем, если её нет
    pub fn moo_cmd_4(&mut self) {
        self.current -= 1;
        // Инициализируем ячейку, если её нет
        if !self.cells.contains_key(&self.current) {
            self.cells.insert(self.current, 0);
        }
    }

    /// Команда MOO: начало цикла
    /// Если текущая ячейка равна 0, переходит к соответствующей команде moo
    pub fn moo_cmd_5(&mut self) {
        if self.get_val() == 0 {
            self.ip = self.loop_map[&self.ip];
        }
    }

    /// Команда moo: конец цикла
    /// Если текущая ячейка не равна 0, возвращается к соответствующей команде MOO
    pub fn moo_cmd_6(&mut self) {
        if self.get_val() != 0 {
            self.ip = self.loop_map[&self.ip] - 1;
        }
    }

    /// Команда OOM: выводит числовое значение текущей ячейки
    pub fn moo_cmd_7(&self) {
        println!("{}", self.get_val());
    }

    /// Команда oom: читает число из стандартного ввода и записывает в текущую ячейку
    pub fn moo_cmd_8(&mut self) {
        print!("Введите число: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(val) = input.trim().parse::<i32>() {
                self.set_val(val);
            }
        }
    }

    /// Команда mOO: выполнение инструкции по значению текущей ячейки
    /// В данной реализации не реализована (пустая функция)
    pub fn moo_cmd_9(&self) {}

    /// Команда Moo: условный ввод/вывод
    /// Если текущая ячейка равна 0, читает число (как oom)
    /// Иначе выводит символ, соответствующий значению ячейки
    pub fn moo_cmd_10(&mut self) {
        let val = self.get_val();
        if val == 0 {
            self.moo_cmd_8();
        } else {
            print!("{}", char::from_u32(val as u32).unwrap_or('?'));
            io::stdout().flush().unwrap();
        }
    }

    /// Команда OOO: обнуляет текущую ячейку
    pub fn moo_cmd_11(&mut self) {
        self.set_val(0);
    }

    /// Команда MMM: работа с регистром
    /// Если регистр пуст, сохраняет значение текущей ячейки в регистр
    /// Если регистр заполнен, копирует значение из регистра в текущую ячейку и очищает регистр
    pub fn moo_cmd_12(&mut self) {
        if self.register.is_none() {
            self.register = Some(self.get_val());
        } else {
            self.set_val(self.register.unwrap());
            self.register = None;
        }
    }

    /// Запускает выполнение программы
    /// Выполняет инструкции последовательно до конца программы
    /// 
    /// # Возвращает
    /// * `Ok(())` - программа выполнена успешно
    /// * `Err(String)` - ошибка во время выполнения (если возникнет)
    pub fn run(&mut self) -> Result<(), String> {
        // Выполняем инструкции пока не достигнем конца программы
        while self.ip < self.instructions.len() {
            let cmd = &self.instructions[self.ip].clone();

            // Выполняем соответствующую команду
            match cmd.as_str() {
                "MoO" => self.moo_cmd(),      // Увеличить значение
                "MOo" => self.moo_cmd_2(),    // Уменьшить значение
                "moO" => self.moo_cmd_3(),    // Сдвиг вправо
                "mOo" => self.moo_cmd_4(),    // Сдвиг влево
                "MOO" => self.moo_cmd_5(),    // Начало цикла
                "moo" => self.moo_cmd_6(),    // Конец цикла
                "OOM" => self.moo_cmd_7(),    // Вывод числа
                "oom" => self.moo_cmd_8(),    // Ввод числа
                "mOO" => self.moo_cmd_9(),    // Выполнение по значению (не реализовано)
                "Moo" => self.moo_cmd_10(),   // Условный ввод/вывод
                "OOO" => self.moo_cmd_11(),   // Обнуление ячейки
                "MMM" => self.moo_cmd_12(),   // Работа с регистром
                _ => {}                       // Неизвестная команда - игнорируем
            }

            // Переходим к следующей инструкции
            self.ip += 1;
        }

        Ok(())
    }
}

