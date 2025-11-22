use std::collections::HashMap;
use std::io::{self, Write};

struct Cow {
    cells: HashMap<i32, i32>,
    current: i32,
    register: Option<i32>,
    ip: usize,
    instructions: Vec<String>,
    loop_map: HashMap<usize, usize>,
}

impl Cow {
    fn new(code: String) -> Result<Self, String> {
        let instructions: Vec<String> = code
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

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

        cow.build_loop_map()?;
        Ok(cow)
    }

    fn build_loop_map(&mut self) -> Result<(), String> {
        let mut stack: Vec<usize> = Vec::new();

        for (i, command) in self.instructions.iter().enumerate() {
            if command == "MOO" {
                stack.push(i);
            } else if command == "moo" {
                if stack.is_empty() {
                    return Err("Ошибка синтаксиса: moo без MOO".to_string());
                }
                let start = stack.pop().unwrap();
                self.loop_map.insert(start, i);
                self.loop_map.insert(i, start);
            }
        }

        if !stack.is_empty() {
            return Err("Ошибка синтаксиса: MOO без moo".to_string());
        }

        Ok(())
    }

    fn get_val(&self) -> i32 {
        *self.cells.get(&self.current).unwrap_or(&0)
    }

    fn set_val(&mut self, val: i32) {
        self.cells.insert(self.current, val);
    }

    // --- Команды ---

    fn moo_cmd(&mut self) {
        self.set_val(self.get_val() + 1);
    }

    fn moo_cmd_2(&mut self) {
        self.set_val(self.get_val() - 1);
    }

    fn moo_cmd_3(&mut self) {
        self.current += 1;
    }

    fn moo_cmd_4(&mut self) {
        self.current -= 1;
    }

    fn moo_cmd_5(&mut self) {
        if self.get_val() == 0 {
            self.ip = self.loop_map[&self.ip];
        }
    }

    fn moo_cmd_6(&mut self) {
        self.ip = self.loop_map[&self.ip] - 1;
    }

    fn moo_cmd_7(&self) {
        println!("{}", self.get_val());
    }

    fn moo_cmd_8(&mut self) {
        print!("Введите число: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(val) = input.trim().parse::<i32>() {
                self.set_val(val);
            }
        }
    }

    fn moo_cmd_9(&self) {
        // Placeholder для функции с кодом 3
    }

    fn moo_cmd_10(&mut self) {
        let val = self.get_val();
        if val == 0 {
            self.moo_cmd_8();
        } else {
            print!("{}", char::from_u32(val as u32).unwrap_or('?'));
            io::stdout().flush().unwrap();
        }
    }

    fn moo_cmd_11(&mut self) {
        self.set_val(0);
    }

    fn moo_cmd_12(&mut self) {
        if self.register.is_none() {
            self.register = Some(self.get_val());
        } else {
            self.set_val(self.register.unwrap());
            self.register = None;
        }
    }

    fn run(&mut self) -> Result<(), String> {
        while self.ip < self.instructions.len() {
            let cmd = &self.instructions[self.ip].clone();

            match cmd.as_str() {
                "MoO" => self.moo_cmd(),
                "MOo" => self.moo_cmd_2(),
                "moO" => self.moo_cmd_3(),
                "mOo" => self.moo_cmd_4(),
                "MOO" => self.moo_cmd_5(),
                "moo" => self.moo_cmd_6(),
                "OOM" => self.moo_cmd_7(),
                "oom" => self.moo_cmd_8(),
                "mOO" => self.moo_cmd_9(),
                "Moo" => self.moo_cmd_10(),
                "OOO" => self.moo_cmd_11(),
                "MMM" => self.moo_cmd_12(),
                _ => {} // Игнорируем неизвестные команды
            }

            self.ip += 1;
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code_example = r#"MoO MoO MoO MoO MoO MoO MoO MoO MOO moO MoO MoO MoO MoO MoO moO MoO MoO MoO MoO moO MoO MoO MoO MoO moO MoO MoO MoO MoO MoO MoO MoO
 MoO MoO moO MoO MoO MoO MoO mOo mOo mOo mOo mOo MOo moo moO moO moO moO Moo moO MOO mOo MoO moO MOo moo mOo MOo MOo MOo Moo MoO MoO 
 MoO MoO MoO MoO MoO Moo Moo MoO MoO MoO Moo MMM mOo mOo mOo MoO MoO MoO MoO Moo moO Moo MOO moO moO MOo mOo mOo MOo moo moO moO MoO 
 MoO MoO MoO MoO MoO MoO MoO Moo MMM MMM Moo MoO MoO MoO Moo MMM MOo MOo MOo Moo MOo MOo MOo MOo MOo MOo MOo MOo Moo mOo MoO Moo"#;

    println!("Запуск кода:");
    let mut cow = Cow::new(code_example.to_string())?;
    cow.run()?;

    Ok(())
}