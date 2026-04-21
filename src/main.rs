mod permanent;
mod trash;
use colored::*;
use permanent::{Delete as PermanentDelete, PermanentDeleter};
use std::env;
use std::path::Path;
use trash::manager::{Delete as TrashDelete, List, Restore, TrashManager};
/// Imprime la ayuda de uso
fn print_usage() {
    println!("del - Eliminar archivos/carpetas de forma segura o permanente\n");
    println!("Uso:");
    println!("  del [opciones] <archivo/carpeta> [...]");
    println!("  del -p, --permanent <archivo/carpeta>  Elimina permanentemente");
    println!("  del -r, --restore                     Restaurar desde trash");
    println!("  del --list                            Listar contenido del trash");
    println!("  del --help                            Muestra esta ayuda");
    println!("\nOpciones:");
    println!("  -p, --permanent    Elimina permanentemente con confirmación");
    println!("  -r, --restore      Restaurar archivo/carpeta");
    println!("  --list             Listar contenido del trash");
    println!("  --help             Muestra esta ayuda");
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
    let mut list = false;
    let mut help = false;
    let mut files: Vec<String> = Vec::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-p" | "--permanent" => permanent = true,
            "-r" | "--restore" => restore = true,
            "--list" => list = true,
            "--help" | "-h" => help = true,
            arg if !arg.starts_with('-') => files.push(arg.to_string()),
            _ => {}
        }
        i += 1;
    }

    let trash_manager = TrashManager::new(trash_dir, history_file, enable_color);
    let permanent_deleter = PermanentDeleter::new(enable_color);

    if help {
        print_usage();
        return;
    }

    if list {
        trash_manager.list();
        return;
    }

    if restore {
        if let Err(e) = trash_manager.restore() {
            eprintln!("{} Error: {}", "✗".red(), e);
        }
        return;
    }

    if files.is_empty() {
        eprintln!(
            "{} {}",
            "✗".red(),
            "Debe especificar al menos un archivo o carpeta"
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
                eprintln!("{} Error: {}", "✗".red(), e);
            }
        } else {
            if let Err(e) = trash_manager.delete(path) {
                eprintln!("{} Error: {}", "✗".red(), e);
            }
        }
    }
}
