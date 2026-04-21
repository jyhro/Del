//! Implementación de TrashManager y traits relacionados
use chrono::Local;
use colored::*;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

pub trait Delete {
    fn delete(&self, path: &Path) -> io::Result<()>;
}

pub trait Restore {
    fn restore(&self) -> io::Result<()>;
}

pub trait List {
    fn list(&self);
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
}

impl Delete for TrashManager {
    fn delete(&self, path: &Path) -> io::Result<()> {
        if !self.trash_dir.exists() {
            fs::create_dir_all(&self.trash_dir)?;
        }
        let file_name = path.file_name().unwrap();
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let mut trash_name = file_name.to_os_string();
        trash_name.push(format!("_{}", timestamp));
        let trash_path = self.trash_dir.join(trash_name);
        fs::rename(path, &trash_path)?;
        let mut hist = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.history_file)?;
        let size = trash_path.metadata().map(|m| m.size()).unwrap_or(0);
        writeln!(
            hist,
            "{}|{}|{}|{}|{}",
            path.display(),
            file_name.to_string_lossy(),
            trash_path.display(),
            timestamp,
            size
        )?;
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
        if !self.history_file.exists() {
            println!("No hay historial de eliminaciones");
            return Ok(());
        }
        let hist = fs::read_to_string(&self.history_file).unwrap_or_default();
        let mut last: Option<(&str, &str, &str, &str, &str)> = None;
        for line in hist.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 5 {
                last = Some((parts[0], parts[1], parts[2], parts[3], parts[4]));
            }
        }
        if let Some((orig, _base, trash_path, _ts, _size)) = last {
            let orig_path = Path::new(orig);
            let trash_path = Path::new(trash_path);
            if !trash_path.exists() {
                println!("No se encontró el archivo en trash");
                return Ok(());
            }
            let dest = if orig_path.exists() {
                orig_path.with_file_name(format!(
                    "{}_restaurado",
                    orig_path.file_name().unwrap().to_string_lossy()
                ))
            } else {
                orig_path.to_path_buf()
            };
            if let Err(e) = fs::rename(trash_path, &dest) {
                eprintln!("{} Error al restaurar: {}", "✗".red(), e);
            } else if self.enable_color {
                println!("{} Restaurado en: {}", "✓".green(), dest.display());
            } else {
                println!("Restaurado en: {}", dest.display());
            }
        } else {
            println!("No hay archivos para restaurar");
        }
        Ok(())
    }
}

impl List for TrashManager {
    fn list(&self) {
        if !self.trash_dir.exists() {
            println!("Trash vacío");
            return;
        }
        let entries = match fs::read_dir(&self.trash_dir) {
            Ok(e) => e,
            Err(_) => {
                println!("Trash vacío");
                return;
            }
        };
        println!("\nArchivos en trash:");
        for entry in entries.flatten() {
            let file_name = entry.file_name();
            println!("- {}", file_name.to_string_lossy());
        }
    }
}
