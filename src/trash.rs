//! Gestion de mover a trash y restaurar desde historial.

use chrono::Local;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::domain::{
    Delete, DeleteOutcome, DeletePreview, Error, HistoryEntry, HistoryRepository, Restore,
    RestoreOutcome, RestorePreview,
};

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
    fn read_history(&self, persist_pruned: bool) -> (Vec<HistoryEntry>, usize) {
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
            if persist_pruned {
                let _ = self.history.replace_all(&active);
            }
            (active, pruned)
        } else {
            (entries, 0)
        }
    }

    fn list_history_with_pruning(
        &self,
        persist_pruned: bool,
    ) -> Result<(Vec<HistoryEntry>, usize), Error> {
        let (mut entries, pruned) = self.read_history(persist_pruned);
        for entry in &mut entries {
            if entry.size == 0 {
                entry.size = calculate_item_size(Path::new(&entry.trash_path));
            }
        }
        Ok((entries, pruned))
    }

    /// Devuelve historial y cantidad de entradas obsoletas eliminadas.
    pub fn list_history(&self) -> Result<(Vec<HistoryEntry>, usize), Error> {
        self.list_history_with_pruning(true)
    }

    /// Devuelve historial sin persistir la limpieza de entradas obsoletas.
    pub fn preview_history(&self) -> Result<(Vec<HistoryEntry>, usize), Error> {
        self.list_history_with_pruning(false)
    }

    /// Vacía el historial si existe.
    pub fn clear_history(&self) -> Result<(), Error> {
        if !self.history.exists() {
            return Err(Error::NoHistory);
        }
        self.history.replace_all(&[])?;
        Ok(())
    }

    /// Devuelve cuantas entradas se limpiarian del historial.
    pub fn preview_clear_history(&self) -> Result<usize, Error> {
        if !self.history.exists() {
            return Err(Error::NoHistory);
        }
        Ok(self.history.read_all()?.len())
    }

    /// Calcula el destino de trash sin crear carpetas ni mover archivos.
    pub fn preview_delete(&self, path: &Path) -> Result<DeletePreview, Error> {
        let file_name = path
            .file_name()
            .ok_or_else(|| Error::NoFileName(path.to_path_buf()))?;
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let mut trash_name = file_name.to_os_string();
        trash_name.push(format!("_{}", timestamp));
        let trash_path = self.trash_dir.join(&trash_name);

        Ok(DeletePreview::Trash {
            source: path.to_path_buf(),
            dest: trash_path,
        })
    }

    /// Calcula que entrada se restauraria y su destino sin mover archivos.
    pub fn preview_restore(&self, index: Option<usize>) -> Result<RestorePreview, Error> {
        let (entries, _) = self.read_history(false);
        if entries.is_empty() {
            return Err(Error::NoHistory);
        }

        let index = index.unwrap_or(entries.len() - 1);
        if index >= entries.len() {
            return Err(Error::InvalidIndex {
                given: index + 1,
                count: entries.len(),
            });
        }

        let entry = &entries[index];
        let orig_path = Path::new(&entry.original_path);
        let trash_path = Path::new(&entry.trash_path);
        let dest = if orig_path.exists() {
            let name = orig_path
                .file_name()
                .ok_or_else(|| Error::NoFileName(orig_path.to_path_buf()))?;
            orig_path.with_file_name(format!("{}_restaurado", name.to_string_lossy()))
        } else {
            orig_path.to_path_buf()
        };

        Ok(RestorePreview {
            source: trash_path.to_path_buf(),
            dest,
        })
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
        let file_name = path
            .file_name()
            .ok_or_else(|| Error::NoFileName(path.to_path_buf()))?;
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
        let (entries, _) = self.read_history(true);
        if entries.is_empty() {
            return Err(Error::NoHistory);
        }
        self.restore_by_index(entries.len() - 1)
    }

    fn restore_by_index(&self, index: usize) -> Result<RestoreOutcome, Error> {
        let (entries, _) = self.read_history(true);
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
    use crate::history::FileHistoryRepository;

    fn temp_dir(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("del_test_trash_{}", name))
    }

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

    #[test]
    fn test_preview_delete_does_not_move_or_write_history() {
        let dir = temp_dir("preview_delete");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();

        let file_path = dir.join("test.txt");
        let trash_dir = dir.join("trash");
        let history_path = dir.join("history");
        fs::write(&file_path, b"content").unwrap();

        let mgr = TrashManager::new(
            trash_dir.clone(),
            Box::new(FileHistoryRepository::new(history_path.clone())),
        );
        let preview = mgr.preview_delete(&file_path).unwrap();

        match preview {
            DeletePreview::Trash { source, dest } => {
                assert_eq!(source, file_path);
                assert!(dest.starts_with(&trash_dir));
            }
            other => panic!("expected trash preview, got {:?}", other),
        }
        assert!(file_path.exists());
        assert!(!trash_dir.exists());
        assert!(!history_path.exists());

        fs::remove_dir_all(&dir).unwrap_or(());
    }

    #[test]
    fn test_preview_history_does_not_prune_history_file() {
        let dir = temp_dir("preview_history");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();

        let history_path = dir.join("history");
        let stale_entry = HistoryEntry {
            original_path: dir.join("original.txt").display().to_string(),
            file_name: "original.txt".to_string(),
            trash_path: dir.join("missing-trash.txt").display().to_string(),
            timestamp: "20260101_120000".to_string(),
            size: 10,
        };
        let repo = FileHistoryRepository::new(history_path.clone());
        repo.append(&stale_entry).unwrap();

        let mgr = TrashManager::new(dir.join("trash"), Box::new(repo));
        let (entries, pruned) = mgr.preview_history().unwrap();

        assert!(entries.is_empty());
        assert_eq!(pruned, 1);
        assert!(
            fs::read_to_string(&history_path)
                .unwrap()
                .contains("original.txt")
        );

        fs::remove_dir_all(&dir).unwrap_or(());
    }

    #[test]
    fn test_preview_restore_does_not_move_or_update_history() {
        let dir = temp_dir("preview_restore");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();

        let trash_path = dir.join("trashed.txt_20260101_120000");
        let original_path = dir.join("restored.txt");
        let history_path = dir.join("history");
        fs::write(&trash_path, b"content").unwrap();

        let entry = HistoryEntry {
            original_path: original_path.display().to_string(),
            file_name: "restored.txt".to_string(),
            trash_path: trash_path.display().to_string(),
            timestamp: "20260101_120000".to_string(),
            size: 7,
        };
        let repo = FileHistoryRepository::new(history_path.clone());
        repo.append(&entry).unwrap();

        let mgr = TrashManager::new(dir.join("trash"), Box::new(repo));
        let preview = mgr.preview_restore(None).unwrap();

        assert_eq!(preview.source, trash_path);
        assert_eq!(preview.dest, original_path);
        assert!(preview.source.exists());
        assert!(
            fs::read_to_string(&history_path)
                .unwrap()
                .contains("restored.txt")
        );

        fs::remove_dir_all(&dir).unwrap_or(());
    }
}
