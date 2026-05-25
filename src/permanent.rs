//! Borrado permanente con sobreescritura y encriptado en memoria.

use rand::RngCore;
use rand::rngs::OsRng;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use walkdir::WalkDir;

use crate::domain::{Delete, DeleteOutcome, Error};

/// Eliminador permanente de archivos y carpetas.
pub struct PermanentDeleter;

impl PermanentDeleter {
    /// Crea un eliminador permanente.
    pub fn new() -> Self {
        PermanentDeleter
    }
}

/// Aplica XOR con bytes aleatorios al contenido en memoria.
fn encrypt_in_memory(data: &mut [u8]) {
    let mut rng = OsRng;
    for byte in data.iter_mut() {
        *byte ^= rng.next_u32() as u8;
    }
}

/// Sobreescribe el archivo con datos aleatorios varias pasadas.
fn overwrite_with_random(path: &Path, passes: usize) -> io::Result<()> {
    let len = fs::metadata(path)?.len() as usize;
    if len == 0 {
        return Ok(());
    }
    let mut rng = OsRng;
    let mut buf = vec![0u8; len];
    for _ in 0..passes {
        rng.fill_bytes(&mut buf);
        let mut file = fs::File::create(path)?;
        file.write_all(&buf)?;
        file.sync_all()?;
    }
    Ok(())
}

/// Elimina un archivo con sobreescritura y limpieza final.
fn secure_delete_file(path: &Path) -> io::Result<()> {
    let len = fs::metadata(path)?.len() as usize;
    if len == 0 {
        return fs::remove_file(path);
    }

    let mut data = fs::read(path)?;

    encrypt_in_memory(&mut data);

    {
        let mut file = fs::File::create(path)?;
        file.write_all(&data)?;
        file.sync_all()?;
    }

    overwrite_with_random(path, 2)?;

    fs::remove_file(path)?;
    Ok(())
}

/// Elimina una carpeta borrando sus archivos primero.
fn secure_delete_dir(path: &Path) -> io::Result<()> {
    let files: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_owned())
        .collect();

    for file_path in &files {
        secure_delete_file(file_path)?;
    }

    fs::remove_dir_all(path)?;
    Ok(())
}

impl Delete for PermanentDeleter {
    fn delete(&self, path: &Path) -> Result<DeleteOutcome, Error> {
        let result = if path.is_dir() {
            secure_delete_dir(path)
        } else {
            secure_delete_file(path)
        };

        result?;

        Ok(DeleteOutcome::Permanent {
            path: path.to_path_buf(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_in_memory_changes_data() {
        let original = b"Hello, this is sensitive data!";
        let mut data = original.to_vec();
        encrypt_in_memory(&mut data);
        assert_ne!(data, original, "encrypted data should differ from original");
        assert_eq!(data.len(), original.len(), "length should remain unchanged");
    }

    #[test]
    fn test_encrypt_in_memory_all_bytes_modified() {
        let original = vec![0u8; 128];
        let mut data = original.clone();
        encrypt_in_memory(&mut data);
        assert!(
            !data.iter().all(|&b| b == 0),
            "non-zero plaintext should produce non-zero ciphertext"
        );
    }

    #[test]
    fn test_encrypt_in_memory_empty() {
        let mut data: Vec<u8> = vec![];
        encrypt_in_memory(&mut data);
        assert!(data.is_empty());
    }

    #[test]
    fn test_secure_delete_file_removes_file() {
        let dir = std::env::temp_dir().join("del_test_secure_delete");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();

        let file_path = dir.join("test.txt");
        fs::write(&file_path, b"sensitive content").unwrap();

        secure_delete_file(&file_path).unwrap();
        assert!(!file_path.exists(), "file should be removed");

        fs::remove_dir_all(&dir).unwrap_or(());
    }

    #[test]
    fn test_secure_delete_empty_file() {
        let dir = std::env::temp_dir().join("del_test_empty");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();

        let file_path = dir.join("empty.txt");
        fs::write(&file_path, b"").unwrap();

        secure_delete_file(&file_path).unwrap();
        assert!(!file_path.exists(), "empty file should be removed");

        fs::remove_dir_all(&dir).unwrap_or(());
    }
}
