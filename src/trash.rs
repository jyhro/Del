//! Gestion de mover a trash y restaurar desde historial.

use chrono::Local;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::domain::{Delete, DeleteOutcome, Error, HistoryEntry, HistoryRepository, Restore, RestoreOutcome};

/// Orquesta el movimiento a trash y el historial asociado.
pub struct TrashManager {
    pub trash_dir: PathBuf,
    pub history: Box<dyn HistoryRepository>,
}

impl TrashManager {
    /// Crea un gestor de trash con su repositorio de historial.
    pub fn new(trash_dir: PathBuf, history: Box<dyn HistoryRepository>) -> Self {
        TrashManager { trash_dir, history }
    }

    /// Lee el historial y elimina entradas obsoletas si es necesario.
    fn read_history(&self) -> (Vec<HistoryEntry>, usize) {
        let entries = match self.history.read_all() {
            Ok(e) => e,
            Err(_) => return (Vec::new(), 0),
        };

        let pruned = crate::domain::prune_stale_entries(&entries);

        if pruned > 0 {
            let active: Vec<HistoryEntry> = entries
                .into_iter()
                .filter(|e| Path::new(&e.trash_path).exists())
                .collect();
            let _ = self.history.replace_all(&active);
            (active, pruned)
        } else {
            (entries, 0)
        }
    }

    /// Devuelve historial y cantidad de entradas obsoletas eliminadas.
    pub fn list_history(&self) -> Result<(Vec<HistoryEntry>, usize), Error> {
        let (mut entries, pruned) = self.read_history();
        for entry in &mut entries {
            if entry.size == 0 {
                entry.size = calculate_item_size(Path::new(&entry.trash_path));
            }
        }
        Ok((entries, pruned))
    }

    /// Vacía el historial si existe.
    pub fn clear_history(&self) -> Result<(), Error> {
        if !self.history.exists() {
            return Err(Error::NoHistory);
        }
        self.history.replace_all(&[])?;
        Ok(())
    }
}

/// Calcula el tamano total de un archivo o carpeta.
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

impl Delete for TrashManager {
    fn delete(&self, path: &Path) -> Result<DeleteOutcome, Error> {
        if !self.trash_dir.exists() {
            fs::create_dir_all(&self.trash_dir)?;
        }
        let file_name = path.file_name().ok_or_else(|| Error::NoFileName(path.to_path_buf()))?;
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let mut trash_name = file_name.to_os_string();
        trash_name.push(format!("_{}", timestamp));
        let trash_path = self.trash_dir.join(&trash_name);
        let size = calculate_item_size(path);

        fs::rename(path, &trash_path)?;

        let entry = HistoryEntry {
            original_path: path.display().to_string(),
            file_name: file_name.to_string_lossy().to_string(),
            trash_path: trash_path.display().to_string(),
            timestamp,
            size,
        };

        let history_warning = match self.history.append(&entry) {
            Ok(()) => None,
            Err(e) => Some(format!(
                "Archivo movido a trash, pero no se pudo registrar en el historial: {}",
                e
            )),
        };

        Ok(DeleteOutcome::Trash {
            dest: trash_path,
            history_warning,
        })
    }
}

impl Restore for TrashManager {
    fn restore(&self) -> Result<RestoreOutcome, Error> {
        let (entries, _) = self.read_history();
        if entries.is_empty() {
            return Err(Error::NoHistory);
        }
        self.restore_by_index(entries.len() - 1)
    }

    fn restore_by_index(&self, index: usize) -> Result<RestoreOutcome, Error> {
        let (entries, _) = self.read_history();
        if index >= entries.len() {
            return Err(Error::InvalidIndex {
                given: index + 1,
                count: entries.len(),
            });
        }
        let entry = &entries[index];
        let orig_path = Path::new(&entry.original_path);
        let trash_path = Path::new(&entry.trash_path);

        if !trash_path.exists() {
            let mut new_entries = entries.clone();
            new_entries.remove(index);
            self.history.replace_all(&new_entries)?;
            return Ok(RestoreOutcome::StaleEntryRemoved);
        }

        let dest = if orig_path.exists() {
            let name = orig_path
                .file_name()
                .ok_or_else(|| Error::NoFileName(orig_path.to_path_buf()))?;
            orig_path.with_file_name(format!("{}_restaurado", name.to_string_lossy()))
        } else {
            orig_path.to_path_buf()
        };

        fs::rename(trash_path, &dest)?;

        let mut new_entries = entries.clone();
        new_entries.remove(index);
        self.history.replace_all(&new_entries)?;

        Ok(RestoreOutcome::Restored { dest })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_item_size_file() {
        let dir = std::env::temp_dir().join("del_test_size");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();

        let file_path = dir.join("test.bin");
        let content = vec![0u8; 4096];
        fs::write(&file_path, &content).unwrap();

        let size = calculate_item_size(&file_path);
        assert_eq!(size, 4096);

        fs::remove_dir_all(&dir).unwrap_or(());
    }
}
