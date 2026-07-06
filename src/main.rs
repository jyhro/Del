mod cli;
mod domain;
mod history;
mod output;
mod permanent;
mod trash;

use std::env;
use std::path::PathBuf;

use cli::Command;
use domain::{Delete, Error, Restore, Summary};
use history::FileHistoryRepository;
use permanent::PermanentDeleter;
use rich_rust::prelude::*;
use trash::TrashManager;

#[cfg(target_os = "windows")]
fn get_trash_and_history() -> (PathBuf, PathBuf) {
    let userprofile = env::var("USERPROFILE").expect("No se pudo obtener USERPROFILE");
    let trash = PathBuf::from(format!("{}\\AppData\\Local\\Temp\\Trash", userprofile));
    let history = PathBuf::from(format!("{}\\AppData\\Local\\del_history", userprofile));
    (trash, history)
}

#[cfg(target_os = "macos")]
fn get_trash_and_history() -> (PathBuf, PathBuf) {
    let home = dirs::home_dir().expect("No se pudo obtener el directorio home");
    let trash = home.join(".Trash");
    let history = home.join(".del_history");
    (trash, history)
}

#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
fn get_trash_and_history() -> (PathBuf, PathBuf) {
    let home = dirs::home_dir().expect("No se pudo obtener el directorio home");
    let trash = home.join(".local/share/Trash");
    let history = home.join(".local/share/del_history");
    (trash, history)
}

fn make_mgr(trash_dir: PathBuf, history_file: PathBuf) -> TrashManager {
    let repo = Box::new(FileHistoryRepository::new(history_file));
    TrashManager::new(trash_dir, repo)
}

fn main() {
    let console = Console::new();
    let (trash_dir, history_file) = get_trash_and_history();
    let args: Vec<String> = env::args().collect();
    let command = cli::parse_args(&console, &args);

    match command {
        Command::Help => {}
        Command::Version => output::show_version(&console),

        Command::ShowHistory { dry_run } => {
            if !history_file.exists() {
                output::show_no_history(&console);
                return;
            }
            let mgr = make_mgr(trash_dir, history_file);
            let result = if dry_run {
                mgr.preview_history()
            } else {
                mgr.list_history()
            };
            match result {
                Ok((entries, pruned)) => output::show_history(&console, &entries, pruned, dry_run),
                Err(e) => output::error(&console, e.to_string()),
            }
        }

        Command::ClearHistory { dry_run, force } => {
            if !history_file.exists() {
                output::show_no_history(&console);
                return;
            }
            if dry_run {
                let mgr = make_mgr(trash_dir, history_file);
                match mgr.preview_clear_history() {
                    Ok(count) => {
                        let mut summary = Summary::new();
                        output::show_dry_run_clear_history(&console, count);
                        summary.record_dry_run();
                        output::show_summary(&console, &summary);
                    }
                    Err(e) => output::error(&console, e.to_string()),
                }
                return;
            }
            let confirmed = if force {
                output::show_force_confirmation_skipped(&console);
                Ok(true)
            } else {
                output::show_clear_history_warning();
                output::confirm()
            };
            match confirmed {
                Ok(true) => {
                    let mgr = make_mgr(trash_dir, history_file);
                    match mgr.clear_history() {
                        Ok(()) => output::show_history_cleared(&console),
                        Err(e) => output::error(&console, e.to_string()),
                    }
                }
                Ok(false) => {}
                Err(e) => output::error(&console, e.to_string()),
            }
        }

        Command::Restore { index, dry_run } => {
            let mgr = make_mgr(trash_dir, history_file);
            if dry_run {
                match mgr.preview_restore(index) {
                    Ok(preview) => {
                        let mut summary = Summary::new();
                        output::show_dry_run_restore(&console, &preview.source, &preview.dest);
                        summary.record_dry_run();
                        output::show_summary(&console, &summary);
                    }
                    Err(Error::NoHistory) => output::show_no_archives(&console),
                    Err(e) => output::error(&console, e.to_string()),
                }
                return;
            }
            let result = if let Some(idx) = index {
                mgr.restore_by_index(idx)
            } else {
                mgr.restore()
            };
            let mut summary = Summary::new();
            match result {
                Ok(outcome) => {
                    match &outcome {
                        domain::RestoreOutcome::Restored { dest } => {
                            output::show_restore(&console, dest);
                        }
                        domain::RestoreOutcome::StaleEntryRemoved => {
                            output::warn(&console, "Entrada obsoleta eliminada del historial");
                        }
                    }
                    summary.record_restore(&outcome);
                }
                Err(Error::NoHistory) => output::show_no_archives(&console),
                Err(e) => output::error(&console, e.to_string()),
            }
            output::show_summary(&console, &summary);
        }

        Command::Delete {
            files,
            permanent,
            dry_run,
            force,
        } => {
            let permanent_deleter = PermanentDeleter::new();
            let mgr = make_mgr(trash_dir, history_file);
            let show_spinner = !dry_run && !permanent && files.len() > 1;
            let mut spinner = if show_spinner {
                Some(output::Spinner::new())
            } else {
                None
            };
            let mut summary = Summary::new();

            for (i, path) in files.iter().enumerate() {
                if let Some(ref mut s) = spinner {
                    s.tick(i + 1, files.len(), path);
                }

                if !path.exists() {
                    if let Some(ref s) = spinner {
                        s.clear();
                    }
                    output::error(&console, format!("'{}' no existe", path.display()));
                    summary.record_fail();
                    continue;
                }

                if dry_run {
                    if permanent {
                        match permanent_deleter.preview_delete(path) {
                            domain::DeletePreview::Permanent { path } => {
                                output::show_dry_run_permanent(&console, &path);
                            }
                            domain::DeletePreview::Trash { source, dest } => {
                                output::show_dry_run_delete(&console, &source, &dest);
                            }
                        }
                    } else {
                        match mgr.preview_delete(path) {
                            Ok(domain::DeletePreview::Trash { source, dest }) => {
                                output::show_dry_run_delete(&console, &source, &dest);
                            }
                            Ok(domain::DeletePreview::Permanent { path }) => {
                                output::show_dry_run_permanent(&console, &path);
                            }
                            Err(e) => {
                                output::error(
                                    &console,
                                    format!("Error al simular trash '{}': {}", path.display(), e),
                                );
                                summary.record_fail();
                                continue;
                            }
                        }
                    }
                    summary.record_dry_run();
                    continue;
                }

                if permanent {
                    if let Some(ref s) = spinner {
                        s.clear();
                    }
                    let confirmed = if force {
                        output::show_force_confirmation_skipped(&console);
                        Ok(true)
                    } else {
                        output::show_permanent_warning(path);
                        output::confirm()
                    };
                    match confirmed {
                        Ok(true) => match permanent_deleter.delete(path) {
                            Ok(outcome) => {
                                if let Some(ref s) = spinner {
                                    s.clear();
                                }
                                if let domain::DeleteOutcome::Permanent { path } = &outcome {
                                    output::show_permanent_delete(&console, path);
                                }
                                summary.record_delete(&outcome);
                            }
                            Err(e) => {
                                if let Some(ref s) = spinner {
                                    s.clear();
                                }
                                output::error(
                                    &console,
                                    format!("Error al eliminar '{}': {}", path.display(), e),
                                );
                                summary.record_fail();
                            }
                        },
                        Ok(false) => {
                            summary.record_cancel();
                        }
                        Err(e) => {
                            if let Some(ref s) = spinner {
                                s.clear();
                            }
                            output::error(&console, e.to_string());
                            summary.record_fail();
                        }
                    }
                } else {
                    match mgr.delete(path) {
                        Ok(outcome) => {
                            if let Some(ref s) = spinner {
                                s.clear();
                            }
                            if let domain::DeleteOutcome::Trash {
                                dest,
                                history_warning,
                            } = &outcome
                            {
                                output::show_delete(&console, dest);
                                if let Some(warning) = history_warning {
                                    output::warn(&console, warning);
                                }
                            }
                            summary.record_delete(&outcome);
                        }
                        Err(e) => {
                            if let Some(ref s) = spinner {
                                s.clear();
                            }
                            output::error(
                                &console,
                                format!("Error al mover a trash '{}': {}", path.display(), e),
                            );
                            summary.record_fail();
                        }
                    }
                }
            }

            if let Some(s) = spinner {
                s.finish();
            }

            output::show_summary(&console, &summary);
        }
    }
}
