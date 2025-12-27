use cow::Cow;
use std::fs;
use std::path::Path;

/// Главная функция программы
/// Выполняет все COW файлы из папки cow_files
fn main() -> Result<(), Box<dyn std::error::Error>> {

    let cow_files_dir = Path::new("cow_files");
    
    // Проверяем существование папки
    if !cow_files_dir.exists() {
        return Err("Папка cow_files не найдена".into());
    }

    // Читаем все файлы из папки
    let entries = fs::read_dir(cow_files_dir)?;
    
    let mut cow_files: Vec<_> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()?.to_str()? == "cow" {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    
    // Сортируем файлы по имени для предсказуемого порядка выполнения
    cow_files.sort();

    // Выполняем каждый COW файл
    for cow_file in cow_files {
        let file_name = cow_file.file_name().unwrap().to_string_lossy();
        println!("\nВыполнение файла: {}", file_name);
        
        // Читаем содержимое файла
        let code = fs::read_to_string(&cow_file)?;
        
        // Создаем интерпретатор и запускаем выполнение программы
        let mut cow = Cow::new(code)?;
        cow.run()?;
        
    }

    Ok(())
}


