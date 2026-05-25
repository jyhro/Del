//! Tipos de dominio y utilidades compartidas.

use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

/// Entrada persistida del historial de eliminaciones.
#[derive(Clone, Debug)]
pub struct HistoryEntry {
    pub original_path: String,
    pub file_name: String,
    pub trash_path: String,
    pub timestamp: String,
    pub size: u64,
}

/// Errores de negocio y de IO expuestos por el dominio.
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    InvalidIndex { given: usize, count: usize },
    NoHistory,
    NoFileName(PathBuf),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "{}", e),
            Error::InvalidIndex { given, count } => {
                write!(
                    f,
                    "Índice inválido: solo hay {} entradas (recibido {})",
                    count, given
                )
            }
            Error::NoHistory => write!(f, "No hay historial de eliminaciones"),
            Error::NoFileName(p) => write!(
                f,
                "La ruta '{}' no tiene un nombre de archivo válido",
                p.display()
            ),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl std::error::Error for Error {}

/// Capacidad de eliminar un archivo o carpeta.
pub trait Delete {
    fn delete(&self, path: &Path) -> Result<DeleteOutcome, Error>;
}

/// Capacidad de restaurar desde el historial.
pub trait Restore {
    fn restore(&self) -> Result<RestoreOutcome, Error>;
    fn restore_by_index(&self, index: usize) -> Result<RestoreOutcome, Error>;
}

/// Abstraccion para persistir historial (DIP).
#[allow(dead_code)]
pub trait HistoryRepository {
    fn read_all(&self) -> Result<Vec<HistoryEntry>, Error>;
    fn append(&self, entry: &HistoryEntry) -> Result<(), Error>;
    fn replace_all(&self, entries: &[HistoryEntry]) -> Result<(), Error>;
    fn exists(&self) -> bool;
}

/// Resultado de una eliminacion.
#[derive(Debug)]
pub enum DeleteOutcome {
    Trash {
        dest: PathBuf,
        history_warning: Option<String>,
    },
    Permanent {
        path: PathBuf,
    },
}

/// Resultado de una restauracion.
#[derive(Debug)]
pub enum RestoreOutcome {
    Restored { dest: PathBuf },
    StaleEntryRemoved,
}

/// Formatea un tamano en unidades legibles.
pub fn format_size(size: u64) -> String {
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

/// Cuenta cuantas entradas del historial ya no existen en trash.
pub fn prune_stale_entries(entries: &[HistoryEntry]) -> usize {
    let before = entries.len();
    let after = entries
        .iter()
        .filter(|e| Path::new(&e.trash_path).exists())
        .count();
    before - after
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size_bytes() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(1), "1 B");
        assert_eq!(format_size(999), "999 B");
    }

    #[test]
    fn test_format_size_kb() {
        assert_eq!(format_size(1_000), "1.0 KB");
        assert_eq!(format_size(1_500), "1.5 KB");
        assert_eq!(format_size(999_999), "1000.0 KB");
    }

    #[test]
    fn test_format_size_mb() {
        assert_eq!(format_size(1_000_000), "1.0 MB");
        assert_eq!(format_size(2_500_000), "2.5 MB");
        assert_eq!(format_size(999_999_999), "1000.0 MB");
    }

    #[test]
    fn test_format_size_gb() {
        assert_eq!(format_size(1_000_000_000), "1.0 GB");
        assert_eq!(format_size(3_700_000_000), "3.7 GB");
    }

    #[test]
    fn test_prune_stale_entries_all_stale() {
        let entries = vec![HistoryEntry {
            original_path: "/tmp/test.txt".to_string(),
            file_name: "test.txt".to_string(),
            trash_path: "/nonexistent/path.txt".to_string(),
            timestamp: "20260523_143022".to_string(),
            size: 100,
        }];
        let pruned = prune_stale_entries(&entries);
        assert_eq!(pruned, 1);
    }

    #[test]
    fn test_prune_stale_entries_none_stale() {
        let trash_dir = std::env::temp_dir().join("del_test_prune");
        let _ = std::fs::remove_dir_all(&trash_dir);
        std::fs::create_dir_all(&trash_dir).unwrap();

        let file_path = trash_dir.join("existing.txt");
        std::fs::write(&file_path, b"hello").unwrap();

        let entries = vec![HistoryEntry {
            original_path: "/tmp/test.txt".to_string(),
            file_name: "test.txt".to_string(),
            trash_path: file_path.display().to_string(),
            timestamp: "20260523_143022".to_string(),
            size: 5,
        }];
        let pruned = prune_stale_entries(&entries);
        assert_eq!(pruned, 0);

        std::fs::remove_dir_all(&trash_dir).unwrap_or(());
    }

    #[test]
    fn test_error_display() {
        let err = Error::InvalidIndex { given: 5, count: 3 };
        assert_eq!(
            err.to_string(),
            "Índice inválido: solo hay 3 entradas (recibido 5)"
        );
    }

    #[test]
    fn test_error_from_io() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err: Error = io_err.into();
        assert!(matches!(err, Error::Io(_)));
    }

    #[test]
    fn test_history_entry_roundtrip() {
        let entry = HistoryEntry {
            original_path: "/home/user/file.txt".to_string(),
            file_name: "file.txt".to_string(),
            trash_path: "/home/user/.local/share/Trash/file.txt_20260523_143022".to_string(),
            timestamp: "20260523_143022".to_string(),
            size: 12345,
        };

        let serialized = format!(
            "{}|{}|{}|{}|{}",
            entry.original_path, entry.file_name, entry.trash_path, entry.timestamp, entry.size
        );

        let parts: Vec<&str> = serialized.split('|').collect();
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0], "/home/user/file.txt");
        assert_eq!(parts[1], "file.txt");
        assert_eq!(parts[3], "20260523_143022");
        assert_eq!(parts[4], "12345");

        let deserialized = HistoryEntry {
            original_path: parts[0].to_string(),
            file_name: parts[1].to_string(),
            trash_path: parts[2].to_string(),
            timestamp: parts[3].to_string(),
            size: parts[4].parse().unwrap_or(0),
        };

        assert_eq!(entry.original_path, deserialized.original_path);
        assert_eq!(entry.file_name, deserialized.file_name);
        assert_eq!(entry.trash_path, deserialized.trash_path);
        assert_eq!(entry.timestamp, deserialized.timestamp);
        assert_eq!(entry.size, deserialized.size);
    }
}
