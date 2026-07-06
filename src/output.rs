use rich_rust::markup;
use rich_rust::prelude::*;
use std::io::{self, Write};
use std::path::Path;

use crate::domain::{self, Error, Summary};

pub fn show_version(console: &Console) {
    console.print(&format!("[bold]del[/] v{}", env!("CARGO_PKG_VERSION")));
}

pub fn print_usage() {
    println!("del - Eliminar archivos/carpetas de forma segura o permanente\n");
    println!("Uso:");
    println!("  del [opciones] <archivo/carpeta> [...]");
    println!("  del -p, --permanent <archivo/carpeta>   Elimina permanentemente");
    println!("  del -r, --restore [N]                   Restaurar último o por índice");
    println!("  del --history                           Mostrar historial de eliminaciones");
    println!("  del --clear-history                     Limpiar historial");
    println!("  del --dry-run [opciones] <archivo>      Simular sin modificar archivos");
    println!("\nOpciones:");
    println!("  -p, --permanent         Elimina permanentemente con confirmación");
    println!("  -r, --restore [N]       Restaurar archivo/carpeta (último o por índice N)");
    println!("  --history               Mostrar historial de eliminaciones");
    println!("  --clear-history         Limpiar historial de eliminaciones");
    println!("  --dry-run               Simula la acción sin mover, eliminar ni restaurar");
    println!("  --force                 Omite confirmaciones interactivas");
    println!("  -v, --version           Muestra la versión");
    println!("  --help                  Muestra esta ayuda");
}

pub fn show_delete(console: &Console, dest: &Path) {
    console.print(&format!(
        "[bold green]✓[/] Movido a trash: {}",
        dest.display()
    ));
}

pub fn show_restore(console: &Console, dest: &Path) {
    console.print(&format!(
        "[bold green]✓[/] Restaurado en: {}",
        dest.display()
    ));
}

pub fn show_permanent_delete(console: &Console, path: &Path) {
    console.print(&format!(
        "[bold green]✓[/] Eliminado permanentemente: {}",
        path.display()
    ));
}

pub fn show_dry_run_delete(console: &Console, source: &Path, dest: &Path) {
    console.print(&format!(
        "[bold cyan]Simulación[/] Se movería a trash: {} -> {}",
        source.display(),
        dest.display()
    ));
}

pub fn show_dry_run_permanent(console: &Console, path: &Path) {
    console.print(&format!(
        "[bold cyan]Simulación[/] Se eliminaría permanentemente: {}",
        path.display()
    ));
}

pub fn show_dry_run_restore(console: &Console, source: &Path, dest: &Path) {
    console.print(&format!(
        "[bold cyan]Simulación[/] Se restauraría: {} -> {}",
        source.display(),
        dest.display()
    ));
}

pub fn show_dry_run_clear_history(console: &Console, count: usize) {
    console.print(&format!(
        "[bold cyan]Simulación[/] Se limpiarían {} entradas del historial",
        count
    ));
}

pub fn show_history(
    console: &Console,
    entries: &[domain::HistoryEntry],
    pruned: usize,
    dry_run: bool,
) {
    if entries.is_empty() {
        if pruned > 0 {
            if dry_run {
                warn(
                    console,
                    "No hay historial activo (entradas obsoletas detectadas; no se modificó el historial)",
                );
            } else {
                warn(
                    console,
                    "No hay historial de eliminaciones (entradas obsoletas eliminadas)",
                );
            }
        } else {
            show_no_history(console);
        }
        return;
    }

    let mut table = Table::new()
        .title("Historial de eliminaciones")
        .with_column(Column::new("#"))
        .with_column(Column::new("Archivo"))
        .with_column(Column::new("Fecha"))
        .with_column(Column::new("Tamaño").justify(JustifyMethod::Right));

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
        let idx_str = (i + 1).to_string();
        table.add_row_cells([
            idx_str.as_str(),
            entry.original_path.as_str(),
            formatted_ts.as_str(),
            size_str.as_str(),
        ]);
    }

    console.print_renderable(&table);

    if pruned > 0 {
        if dry_run {
            warn(
                console,
                format!(
                    "{} entradas obsoletas detectadas (no eliminadas por --dry-run)",
                    pruned
                ),
            );
        } else {
            warn(
                console,
                format!("{} entradas obsoletas eliminadas del historial", pruned),
            );
        }
    }
}

pub fn show_no_history(console: &Console) {
    console.print("[yellow]⚠[/] No hay historial de eliminaciones");
}

pub fn show_history_cleared(console: &Console) {
    console.print("[bold green]✓[/] Historial eliminado");
}

pub fn show_summary(console: &Console, summary: &Summary) {
    if !summary.has_work() {
        return;
    }

    let mut lines: Vec<String> = Vec::new();

    if summary.trash_count > 0 {
        lines.push(format!(
            "[bold green]✓[/] Movidos a trash: {}",
            summary.trash_count
        ));
    }
    if summary.permanent_count > 0 {
        lines.push(format!(
            "[bold red]✗[/] Eliminados permanentemente: {}",
            summary.permanent_count
        ));
    }
    if summary.restore_count > 0 {
        lines.push(format!(
            "[bold green]✓[/] Restaurados: {}",
            summary.restore_count
        ));
    }
    if summary.fail_count > 0 {
        lines.push(format!("[bold red]✗[/] Fallos: {}", summary.fail_count));
    }
    if summary.cancel_count > 0 {
        lines.push(format!("[yellow]⚠[/] Cancelados: {}", summary.cancel_count));
    }
    if summary.dry_run_count > 0 {
        lines.push(format!(
            "[bold cyan]Simulación[/] Acciones simuladas: {}",
            summary.dry_run_count
        ));
    }

    let content = lines.join("\n");
    let rich_content = markup::render_or_plain(&content);
    let panel = Panel::from_rich_text(&rich_content, usize::MAX).title("Resumen");
    console.print_renderable(&panel);
}

pub fn show_no_archives(console: &Console) {
    console.print("[yellow]⚠[/] No hay archivos para restaurar");
}

pub fn show_permanent_warning(path: impl AsRef<Path>) {
    println!("⚠️  Advertencia: Esta acción no se puede deshacer");
    print!(
        "¿Está seguro de que desea eliminar permanentemente '{}'? (s/n): ",
        path.as_ref().display()
    );
    let _ = io::stdout().flush();
}

pub fn show_clear_history_warning() {
    println!("⚠️  Se eliminará todo el historial de eliminaciones");
    print!("¿Está seguro? (s/n): ");
    let _ = io::stdout().flush();
}

pub fn show_force_confirmation_skipped(console: &Console) {
    warn(console, "Confirmación omitida por --force");
}

pub fn confirm() -> Result<bool, Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let confirm = input.trim().eq_ignore_ascii_case("s");
    if !confirm {
        println!("Cancelado");
    }
    Ok(confirm)
}

pub fn error(console: &Console, msg: impl AsRef<str>) {
    console.print(&format!("[bold red]✗[/] {}", msg.as_ref()));
}

pub fn warn(console: &Console, msg: impl AsRef<str>) {
    console.print(&format!("[yellow]⚠[/] {}", msg.as_ref()));
}

pub fn unknown_flag_with_suggestion(console: &Console, unknown: &str, suggestion: &str) {
    console.print(&format!(
        "[bold red]✗[/] Flag desconocido: '{}'. ¿Quizás quiso decir '{}'?",
        unknown, suggestion
    ));
}

pub fn unknown_flag(console: &Console, flag: &str) {
    console.print(&format!("[bold red]✗[/] Flag desconocido: '{}'", flag));
}

pub struct Spinner {
    frames: [char; 4],
    idx: usize,
}

impl Spinner {
    pub fn new() -> Self {
        Spinner {
            frames: ['|', '/', '-', '\\'],
            idx: 0,
        }
    }

    pub fn tick(&mut self, current: usize, total: usize, path: &Path) {
        let ch = self.frames[self.idx];
        self.idx = (self.idx + 1) % self.frames.len();
        eprint!("\r{} [{}/{}] {}", ch, current, total, path.display());
        let _ = io::stderr().flush();
    }

    pub fn clear(&self) {
        eprint!("\r{:w$}\r", "", w = 60);
        let _ = io::stderr().flush();
    }

    pub fn finish(self) {
        eprintln!();
    }
}
