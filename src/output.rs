//! Salida de consola y prompts interactivos.

use colored::*;
use std::io::{self, Write};
use std::path::Path;

use crate::domain::{self, DeleteOutcome, Error, RestoreOutcome};

/// Imprime la version del binario.
pub fn show_version() {
    println!("del v{}", env!("CARGO_PKG_VERSION"));
}

/// Imprime la ayuda de uso.
pub fn print_usage() {
    println!("del - Eliminar archivos/carpetas de forma segura o permanente\n");
    println!("Uso:");
    println!("  del [opciones] <archivo/carpeta> [...]");
    println!("  del -p, --permanent <archivo/carpeta>   Elimina permanentemente");
    println!("  del -r, --restore [N]                   Restaurar último o por índice");
    println!("  del --history                           Mostrar historial de eliminaciones");
    println!("  del --clear-history                     Limpiar historial");
    println!("\nOpciones:");
    println!("  -p, --permanent         Elimina permanentemente con confirmación");
    println!("  -r, --restore [N]       Restaurar archivo/carpeta (último o por índice N)");
    println!("  --history               Mostrar historial de eliminaciones");
    println!("  --clear-history         Limpiar historial de eliminaciones");
    println!("  -v, --version           Muestra la versión");
    println!("  --help                  Muestra esta ayuda");
}

/// Muestra el resultado de una eliminacion.
pub fn show_delete(outcome: &DeleteOutcome) {
    match outcome {
        DeleteOutcome::Trash {
            dest,
            history_warning,
            ..
        } => {
            println!("{} Movido a trash: {}", "✓".green(), dest.display());
            if let Some(w) = history_warning {
                eprintln!("{}", w);
            }
        }
        DeleteOutcome::Permanent { path } => {
            println!(
                "{} Eliminado permanentemente: {}",
                "✓".green(),
                path.display()
            );
        }
    }
}

/// Muestra el resultado de una restauracion.
pub fn show_restore(outcome: &RestoreOutcome) {
    match outcome {
        RestoreOutcome::Restored { dest } => {
            println!("{} Restaurado en: {}", "✓".green(), dest.display());
        }
        RestoreOutcome::StaleEntryRemoved => {
            warn("Entrada obsoleta eliminada del historial");
        }
    }
}

/// Imprime el historial formateado.
pub fn show_history(entries: &[domain::HistoryEntry], pruned: usize) {
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
        let size_str = domain::format_size(entry.size);
        println!(
            " {}. {} | {} | {}",
            (i + 1).to_string().cyan(),
            entry.original_path.cyan(),
            formatted_ts.cyan(),
            size_str.cyan()
        );
    }
    if pruned > 0 {
        eprintln!(
            "\n  ({} entradas obsoletas eliminadas del historial)",
            pruned.to_string().yellow()
        );
    }
}

/// Mensaje cuando no hay historial.
pub fn show_no_history() {
    println!("No hay historial de eliminaciones");
}

/// Mensaje de historial eliminado.
pub fn show_history_cleared() {
    println!("Historial eliminado");
}

/// Mensaje cuando no hay entradas para restaurar.
pub fn show_no_archives() {
    println!("No hay archivos para restaurar");
}

/// Advertencia y prompt de confirmacion para borrado permanente.
pub fn show_permanent_warning(path: impl AsRef<Path>) {
    println!("⚠️  Advertencia: Esta acción no se puede deshacer");
    print!(
        "¿Está seguro de que desea eliminar permanentemente '{}'? (s/n): ",
        path.as_ref().display()
    );
    let _ = io::stdout().flush();
}

/// Advertencia y prompt de confirmacion para limpiar historial.
pub fn show_clear_history_warning() {
    println!("⚠️  Se eliminará todo el historial de eliminaciones");
    print!("¿Está seguro? (s/n): ");
    let _ = io::stdout().flush();
}

/// Lee confirmacion por stdin (s/n).
pub fn confirm() -> Result<bool, Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let confirm = input.trim().eq_ignore_ascii_case("s");
    if !confirm {
        println!("Cancelado");
    }
    Ok(confirm)
}

/// Imprime un error en stderr.
pub fn error(msg: impl AsRef<str>) {
    eprintln!("{} {}", "✗".red(), msg.as_ref());
}

/// Imprime una advertencia en stderr.
pub fn warn(msg: impl AsRef<str>) {
    eprintln!("⚠️  {}", msg.as_ref());
}

/// Error de flag desconocido con sugerencia.
pub fn unknown_flag_with_suggestion(unknown: &str, suggestion: &str) {
    eprintln!(
        "{} Flag desconocido: '{}'. ¿Quizás quiso decir '{}'?",
        "✗".red(),
        unknown,
        suggestion
    );
}

/// Error de flag desconocido sin sugerencia.
pub fn unknown_flag(flag: &str) {
    eprintln!("{} Flag desconocido: '{}'", "✗".red(), flag);
}
