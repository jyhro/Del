//! Persistencia del historial en archivo delimitado por '|'.

use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::domain::{Error, HistoryEntry, HistoryRepository};

/// Implementacion de historial basada en archivo de texto.
pub struct FileHistoryRepository {
    pub history_file: PathBuf,
}

impl FileHistoryRepository {
    /// Crea un repositorio de historial usando la ruta indicada.
    pub fn new(history_file: PathBuf) -> Self {
        FileHistoryRepository { history_file }
    }
}

impl HistoryRepository for FileHistoryRepository {
    fn exists(&self) -> bool {
        self.history_file.exists()
    }

    fn read_all(&self) -> Result<Vec<HistoryEntry>, Error> {
        let content = match fs::read_to_string(&self.history_file) {
            Ok(c) => c,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(e) => return Err(Error::Io(e)),
        };

        let entries: Vec<HistoryEntry> = content
            .lines()
            .filter(|line| !line.is_empty())
            .filter_map(|line| {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() < 5 {
                    return None;
                }
                Some(HistoryEntry {
                    original_path: parts[0].to_string(),
                    file_name: parts[1].to_string(),
                    trash_path: parts[2].to_string(),
                    timestamp: parts[3].to_string(),
                    size: parts[4].parse().unwrap_or(0),
                })
            })
            .collect();

        Ok(entries)
    }

    fn append(&self, entry: &HistoryEntry) -> Result<(), Error> {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.history_file)?;
        writeln!(
            file,
            "{}|{}|{}|{}|{}",
            entry.original_path, entry.file_name, entry.trash_path, entry.timestamp, entry.size
        )?;
        Ok(())
    }

    fn replace_all(&self, entries: &[HistoryEntry]) -> Result<(), Error> {
        let mut file = fs::File::create(&self.history_file)?;
        for entry in entries {
            writeln!(
                file,
                "{}|{}|{}|{}|{}",
                entry.original_path, entry.file_name, entry.trash_path, entry.timestamp, entry.size
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_history_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("del_test_history_{}", name))
    }

    fn sample_entry() -> HistoryEntry {
        HistoryEntry {
            original_path: "/home/user/file.txt".to_string(),
            file_name: "file.txt".to_string(),
            trash_path: "/home/user/.local/share/Trash/file.txt_20260101_120000".to_string(),
            timestamp: "20260101_120000".to_string(),
            size: 4096,
        }
    }

    #[test]
    fn test_read_all_empty_file() {
        let path = temp_history_path("empty");
        let _ = fs::remove_file(&path);
        fs::write(&path, "").unwrap();

        let repo = FileHistoryRepository::new(path.clone());
        let entries = repo.read_all().unwrap();
        assert!(entries.is_empty());

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_read_all_nonexistent_file() {
        let path = temp_history_path("nonexistent");
        let _ = fs::remove_file(&path);

        let repo = FileHistoryRepository::new(path);
        let entries = repo.read_all().unwrap();
        assert!(entries.is_empty());
    }

    #[test]
    fn test_append_and_read_all() {
        let path = temp_history_path("append");
        let _ = fs::remove_file(&path);

        let repo = FileHistoryRepository::new(path.clone());
        let entry = sample_entry();
        repo.append(&entry).unwrap();

        let entries = repo.read_all().unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].original_path, "/home/user/file.txt");
        assert_eq!(entries[0].size, 4096);

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_replace_all() {
        let path = temp_history_path("replace");
        let _ = fs::remove_file(&path);

        let repo = FileHistoryRepository::new(path.clone());
        let repo2 = FileHistoryRepository::new(path.clone());

        repo.append(&sample_entry()).unwrap();

        let entries = repo2.read_all().unwrap();
        assert_eq!(entries.len(), 1);

        repo2.replace_all(&[]).unwrap();

        let entries = repo2.read_all().unwrap();
        assert!(entries.is_empty());

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_exists() {
        let path = temp_history_path("exists");
        let _ = fs::remove_file(&path);

        let repo = FileHistoryRepository::new(path.clone());
        assert!(!repo.exists());

        fs::write(&path, "").unwrap();
        assert!(repo.exists());

        let _ = fs::remove_file(&path);
    }
}
