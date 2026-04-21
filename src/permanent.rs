//! Eliminación permanente de archivos/carpeta
use colored::*;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub trait Delete {
    fn delete(&self, path: &Path) -> io::Result<()>;
}

pub struct PermanentDeleter {
    pub enable_color: bool,
}

impl PermanentDeleter {
    pub fn new(enable_color: bool) -> Self {
        PermanentDeleter { enable_color }
    }
}

impl Delete for PermanentDeleter {
    fn delete(&self, path: &Path) -> io::Result<()> {
        println!("⚠️  Advertencia: Esta acción no se puede deshacer");
        print!(
            "¿Está seguro de que desea eliminar permanentemente '{}'? (s/n): ",
            path.display()
        );
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("s") {
            println!("Cancelado");
            return Ok(());
        }
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
        if self.enable_color {
            println!(
                "{} Eliminado permanentemente: {}",
                "✓".green(),
                path.display()
            );
        } else {
            println!("Eliminado permanentemente: {}", path.display());
        }
        Ok(())
    }
}
