//! Implementación de TrashManager y traits relacionados
use chrono::Local;
use colored::*;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub trait Delete {
    fn delete(&self, path: &Path) -> io::Result<()>;
}

pub trait Restore {
    fn restore(&self) -> io::Result<()>;
}

#[derive(Clone)]
pub struct HistoryEntry {
    pub original_path: String,
    pub file_name: String,
    pub trash_path: String,
    pub timestamp: String,
    pub size: u64,
}

pub struct TrashManager {
    pub trash_dir: PathBuf,
    pub history_file: PathBuf,
    pub enable_color: bool,
}

impl TrashManager {
    pub fn new(trash_dir: PathBuf, history_file: PathBuf, enable_color: bool) -> Self {
        TrashManager {
            trash_dir,
            history_file,
            enable_color,
        }
    }

    pub fn read_history(&self) -> (Vec<HistoryEntry>, usize) {
        let content = fs::read_to_string(&self.history_file).unwrap_or_default();
        let mut entries: Vec<HistoryEntry> = Vec::new();
        for line in content.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 5 {
                entries.push(HistoryEntry {
                    original_path: parts[0].to_string(),
                    file_name: parts[1].to_string(),
                    trash_path: parts[2].to_string(),
                    timestamp: parts[3].to_string(),
                    size: parts[4].parse().unwrap_or(0),
                });
            }
        }
        let before = entries.len();
        entries.retain(|e| Path::new(&e.trash_path).exists());
        let pruned = before - entries.len();
        if pruned > 0 && let Err(e) = self.write_history(&entries) {
            eprintln!("⚠️  No se pudo actualizar el historial tras limpiar entradas obsoletas: {}", e);
        }
        (entries, pruned)
    }

    pub fn write_history(&self, entries: &[HistoryEntry]) -> io::Result<()> {
        let mut file = fs::File::create(&self.history_file)?;
        for entry in entries {
            writeln!(
                file,
                "{}|{}|{}|{}|{}",
                entry.original_path,
                entry.file_name,
                entry.trash_path,
                entry.timestamp,
                entry.size
            )?;
        }
        Ok(())
    }

    pub fn restore_by_index(&self, index: usize) -> io::Result<()> {
        let (entries, _) = self.read_history();
        if index >= entries.len() {
            eprintln!(
                "{} Índice inválido: solo hay {} entradas",
                "✗".red(),
                entries.len()
            );
            return Ok(());
        }
        let entry = &entries[index];
        let orig_path = Path::new(&entry.original_path);
        let trash_path = Path::new(&entry.trash_path);

        if !trash_path.exists() {
            eprintln!(
                "No se encontró el archivo en trash: {}",
                entry.trash_path
            );
            let mut new_entries = entries;
            new_entries.remove(index);
            self.write_history(&new_entries)?;
            return Ok(());
        }

        let dest = if orig_path.exists() {
            let name = orig_path.file_name().ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("La ruta '{}' no tiene un nombre de archivo válido", orig_path.display()),
                )
            })?;
            orig_path.with_file_name(format!("{}_restaurado", name.to_string_lossy()))
        } else {
            orig_path.to_path_buf()
        };

        if let Err(e) = fs::rename(trash_path, &dest) {
            eprintln!("{} Error al restaurar: {}", "✗".red(), e);
        } else {
            let mut new_entries = entries;
            new_entries.remove(index);
            self.write_history(&new_entries)?;
            if self.enable_color {
                println!("{} Restaurado en: {}", "✓".green(), dest.display());
            } else {
                println!("Restaurado en: {}", dest.display());
            }
        }
        Ok(())
    }

    pub fn list_history(&self) {
        if !self.history_file.exists() {
            println!("No hay historial de eliminaciones");
            return;
        }
        let (entries, pruned) = self.read_history();
        if entries.is_empty() {
            if pruned > 0 {
                println!("No hay historial de eliminaciones (entradas obsoletas eliminadas)");
            } else {
                println!("No hay historial de eliminaciones");
            }
            return;
        }
        println!("\nHistorial de eliminaciones:");
        for (i, entry) in entries.iter().enumerate() {
            let ts = &entry.timestamp;
            let formatted_ts = if ts.len() == 15 {
                format!(
                    "{}-{}-{} {}:{}:{}",
                    &ts[0..4],
                    &ts[4..6],
                    &ts[6..8],
                    &ts[9..11],
                    &ts[11..13],
                    &ts[13..15]
                )
            } else {
                ts.clone()
            };
            let real_size = if entry.size == 0 {
                calculate_item_size(Path::new(&entry.trash_path))
            } else {
                entry.size
            };
            let size_str = format_size(real_size);
            if self.enable_color {
                println!(
                    " {}. {} | {} | {}",
                    (i + 1).to_string().cyan(),
                    entry.original_path.cyan(),
                    formatted_ts.cyan(),
                    size_str.cyan()
                );
            } else {
                println!(
                    " {}. {} | {} | {}",
                    i + 1,
                    entry.original_path,
                    formatted_ts,
                    size_str
                );
            }
        }
        if pruned > 0 {
            if self.enable_color {
                eprintln!(
                    "\n  ({} entradas obsoletas eliminadas del historial)",
                    pruned.to_string().yellow()
                );
            } else {
                eprintln!(
                    "\n  ({} entradas obsoletas eliminadas del historial)",
                    pruned
                );
            }
        }
    }

    pub fn clear_history(&self) -> io::Result<()> {
        if !self.history_file.exists() {
            println!("No hay historial de eliminaciones");
            return Ok(());
        }
        println!("⚠️  Se eliminará todo el historial de eliminaciones");
        print!("¿Está seguro? (s/n): ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("s") {
            println!("Cancelado");
            return Ok(());
        }
        fs::write(&self.history_file, "")?;
        println!("Historial eliminado");
        Ok(())
    }
}

fn calculate_item_size(path: &Path) -> u64 {
    if path.is_dir() {
        WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter_map(|e| e.metadata().ok())
            .map(|m| m.len())
            .sum()
    } else {
        path.metadata().map(|m| m.len()).unwrap_or(0)
    }
}

fn format_size(size: u64) -> String {
    if size >= 1_000_000_000 {
        format!("{:.1} GB", size as f64 / 1_000_000_000.0)
    } else if size >= 1_000_000 {
        format!("{:.1} MB", size as f64 / 1_000_000.0)
    } else if size >= 1_000 {
        format!("{:.1} KB", size as f64 / 1_000.0)
    } else {
        format!("{} B", size)
    }
}

impl Delete for TrashManager {
    fn delete(&self, path: &Path) -> io::Result<()> {
        if !self.trash_dir.exists() {
            fs::create_dir_all(&self.trash_dir)?;
        }
        let file_name = path.file_name().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("La ruta '{}' no tiene un nombre de archivo válido", path.display()),
            )
        })?;
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let mut trash_name = file_name.to_os_string();
        trash_name.push(format!("_{}", timestamp));
        let trash_path = self.trash_dir.join(trash_name);
        let size = calculate_item_size(path);
        fs::rename(path, &trash_path)?;
        if let Err(e) = (|| -> io::Result<()> {
            let mut hist = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.history_file)?;
            writeln!(
                hist,
                "{}|{}|{}|{}|{}",
                path.display(),
                file_name.to_string_lossy(),
                trash_path.display(),
                timestamp,
                size
            )?;
            Ok(())
        })() {
            eprintln!("⚠️  Archivo movido a trash, pero no se pudo registrar en el historial: {}", e);
        }
        if self.enable_color {
            println!("{} Movido a trash: {}", "✓".green(), trash_path.display());
        } else {
            println!("Movido a trash: {}", trash_path.display());
        }
        Ok(())
    }
}

impl Restore for TrashManager {
    fn restore(&self) -> io::Result<()> {
        let (entries, _) = self.read_history();
        if entries.is_empty() {
            println!("No hay archivos para restaurar");
            return Ok(());
        }
        self.restore_by_index(entries.len() - 1)
    }
}




