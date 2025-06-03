use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Использование: {} <md5 директория1> <md5 директория2>", args[0]);
        std::process::exit(1);
    }
    
    let dir1 = Path::new(&args[1]);
    let dir2 = Path::new(&args[2]);

    let files1 = collect_files(dir1)?;
    let files2 = collect_files(dir2)?;

    // Проверяем файлы второй директории
    for (rel_path, full_path2) in &files2 {
        match files1.get(rel_path) {
            Some(full_path1) => {
                if !files_equal(full_path1, full_path2)? {
                    println!("Изменен: {}", rel_path.display());
                }
            }
            None => println!("Новый: {}", rel_path.display()),
        }
    }

    Ok(())
}

/// Рекурсивно собирает файлы с относительными путями
fn collect_files(root: &Path) -> io::Result<HashMap<PathBuf, PathBuf>> {
    let mut files = HashMap::new();
    
    for entry in WalkDir::new(root)
        .min_depth(1)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok()) // Игнорируем ошибки доступа
    {
        let path = entry.path();
        
        if path.is_file() {
            // Получаем относительный путь
            if let Ok(relative_path) = path.strip_prefix(root) {
                files.insert(relative_path.to_path_buf(), path.to_path_buf());
            }
        }
    }
    
    Ok(files)
}

/// Сравнивает содержимое двух файлов фиксированного размера
fn files_equal(path1: &Path, path2: &Path) -> io::Result<bool> {
    // Читаем всё содержимое файлов
    let content1 = fs::read(path1)?;
    let content2 = fs::read(path2)?;
    
    // Сравниваем содержимое
    Ok(content1 == content2)
}
