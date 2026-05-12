mod permanent;
mod trash;
use colored::*;
use permanent::{Delete as PermanentDelete, PermanentDeleter};
use std::env;
use std::path::Path;
use trash::manager::{Delete as TrashDelete, Restore, TrashManager};

/// Imprime la ayuda de uso
fn print_usage() {
    println!("del - Eliminar archivos/carpetas de forma segura o permanente\n");
    println!("Uso:");
    println!("  del [opciones] <archivo/carpeta> [...]");
    println!("  del -p, --permanent <archivo/carpeta>  Elimina permanentemente");
    println!("  del -r, --restore [N]                  Restaurar último o por índice");
    println!("  del --history                          Mostrar historial de eliminaciones");
    println!("  del --clear-history                    Limpiar historial");
    println!("  del --help                             Muestra esta ayuda");
    println!("\nOpciones:");
    println!("  -p, --permanent          Elimina permanentemente con confirmación");
    println!("  -r, --restore [N]        Restaurar archivo/carpeta (último o por índice N)");
    println!("  --history                Mostrar historial de eliminaciones");
    println!("  --clear-history          Limpiar historial de eliminaciones");
    println!("  --help                   Muestra esta ayuda");
}

fn suggest_flag(unknown: &str) -> Option<&'static str> {
    const KNOWN: &[&str] = &[
        "-p", "--permanent", "-r", "--restore",
        "--history", "--clear-history", "--help", "-h",
    ];

    let mut best: Option<&'static str> = None;
    let mut best_score = 0usize;

    for &flag in KNOWN {
        let score = unknown.chars()
            .zip(flag.chars())
            .take_while(|(a, b)| a == b)
            .count();
        if score > best_score {
            best_score = score;
            best = Some(flag);
        }
    }

    if best_score >= 3 { best } else { None }
}

fn main() {
    // Configuración
    let enable_color = true;

    // Detectar sistema operativo y definir rutas de trash/historial
    #[cfg(target_os = "windows")]
    fn get_trash_and_history() -> (std::path::PathBuf, std::path::PathBuf) {
        use std::env;
        let userprofile = env::var("USERPROFILE").expect("No se pudo obtener USERPROFILE");
        let trash =
            std::path::PathBuf::from(format!("{}\\AppData\\Local\\Temp\\Trash", userprofile));
        let history =
            std::path::PathBuf::from(format!("{}\\AppData\\Local\\del_history", userprofile));
        (trash, history)
    }

    #[cfg(not(target_os = "windows"))]
    fn get_trash_and_history() -> (std::path::PathBuf, std::path::PathBuf) {
        let home = dirs::home_dir().expect("No se pudo obtener el directorio home");
        let trash = home.join(".local/share/Trash");
        let history = home.join(".local/share/del_history");
        (trash, history)
    }

    let (trash_dir, history_file) = get_trash_and_history();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }

    // Subcomandos y flags
    let mut permanent = false;
    let mut restore = false;
    let mut restore_index: Option<usize> = None;
    let mut show_history = false;
    let mut clear_history = false;
    let mut help = false;
    let mut files: Vec<String> = Vec::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-p" | "--permanent" => permanent = true,
            "-r" | "--restore" => {
                restore = true;
                if i + 1 < args.len() {
                    if let Ok(n) = args[i + 1].parse::<usize>() {
                        if let Some(idx) = n.checked_sub(1) {
                            restore_index = Some(idx);
                        }
                        i += 1;
                    } else if !args[i + 1].starts_with('-') {
                        eprintln!("{} '{}' no es un índice válido, se restaurará el último",
                            "✗".red(), args[i + 1]);
                        i += 1;
                    }
                }
            }
            "--history" => show_history = true,
            "--clear-history" => clear_history = true,
            "--help" | "-h" => help = true,
            arg if !arg.starts_with('-') => files.push(arg.to_string()),
            arg => {
                let suggestion = suggest_flag(arg);
                if let Some(s) = suggestion {
                    eprintln!("{} Flag desconocido: '{}'. ¿Quizás quiso decir '{}'?",
                        "✗".red(), arg, s);
                } else {
                    eprintln!("{} Flag desconocido: '{}'", "✗".red(), arg);
                }
            }
        }
        i += 1;
    }

    let trash_manager = TrashManager::new(trash_dir, history_file, enable_color);
    let permanent_deleter = PermanentDeleter::new(enable_color);

    if help {
        print_usage();
        return;
    }

    if show_history {
        trash_manager.list_history();
        return;
    }

    if clear_history {
        if let Err(e) = trash_manager.clear_history() {
            eprintln!("{} Error al limpiar historial: {}", "✗".red(), e);
        }
        return;
    }

    if restore {
        let result = if let Some(idx) = restore_index {
            trash_manager.restore_by_index(idx)
        } else {
            trash_manager.restore()
        };
        if let Err(e) = result {
            eprintln!("{} Error al restaurar: {}", "✗".red(), e);
        }
        return;
    }

    if files.is_empty() {
        eprintln!(
            "{} Debe especificar al menos un archivo o carpeta",
            "✗".red()
        );
        print_usage();
        return;
    }

    for file in files {
        let path = Path::new(&file);
        if !path.exists() {
            eprintln!("{} '{}' no existe", "✗".red(), file);
            continue;
        }
        if permanent {
            if let Err(e) = permanent_deleter.delete(path) {
                eprintln!("{} Error al eliminar '{}': {}", "✗".red(), file, e);
            }
        } else {
            if let Err(e) = trash_manager.delete(path) {
                eprintln!("{} Error al mover a trash '{}': {}", "✗".red(), file, e);
            }
        }
    }
}
