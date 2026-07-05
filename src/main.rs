mod cli;
mod domain;
mod history;
mod output;
mod permanent;
mod trash;

use std::env;
use std::path::PathBuf;

use cli::Command;
use domain::{Delete, Error, Restore};
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

        Command::ShowHistory => {
            if !history_file.exists() {
                output::show_no_history(&console);
                return;
            }
            let mgr = make_mgr(trash_dir, history_file);
            match mgr.list_history() {
                Ok((entries, pruned)) => output::show_history(&console, &entries, pruned),
                Err(e) => output::error(&console, e.to_string()),
            }
        }

        Command::ClearHistory { force } => {
            if !history_file.exists() {
                output::show_no_history(&console);
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

        Command::Restore { index } => {
            let mgr = make_mgr(trash_dir, history_file);
            let result = if let Some(idx) = index {
                mgr.restore_by_index(idx)
            } else {
                mgr.restore()
            };
            match result {
                Ok(outcome) => match outcome {
                    domain::RestoreOutcome::Restored { dest } => {
                        output::show_restore(&console, &dest);
                    }
                    domain::RestoreOutcome::StaleEntryRemoved => {
                        output::warn(&console, "Entrada obsoleta eliminada del historial");
                    }
                },
                Err(Error::NoHistory) => output::show_no_archives(&console),
                Err(e) => output::error(&console, e.to_string()),
            }
        }

        Command::Delete {
            files,
            permanent,
            force,
        } => {
            let permanent_deleter = PermanentDeleter::new();
            let mgr = make_mgr(trash_dir, history_file);
            let show_spinner = !permanent && files.len() > 1;
            let mut spinner = if show_spinner {
                Some(output::Spinner::new())
            } else {
                None
            };

            for (i, path) in files.iter().enumerate() {
                if let Some(ref mut s) = spinner {
                    s.tick(i + 1, files.len(), path);
                }

                if !path.exists() {
                    if let Some(ref s) = spinner {
                        s.clear();
                    }
                    output::error(&console, format!("'{}' no existe", path.display()));
                    continue;
                }

                if permanent {
                    let confirmed = if force {
                        output::show_force_confirmation_skipped(&console);
                        Ok(true)
                    } else {
                        output::show_permanent_warning(path);
                        output::confirm()
                    };
                    match confirmed {
                        Ok(true) => match permanent_deleter.delete(path) {
                            Ok(_) => {
                                if let Some(ref s) = spinner {
                                    s.clear();
                                }
                                console.print(&format!(
                                    "[bold green]✓[/] Eliminado permanentemente: {}",
                                    path.display()
                                ));
                            }
                            Err(e) => {
                                if let Some(ref s) = spinner {
                                    s.clear();
                                }
                                output::error(
                                    &console,
                                    format!("Error al eliminar '{}': {}", path.display(), e),
                                );
                            }
                        },
                        Ok(false) => {}
                        Err(e) => {
                            if let Some(ref s) = spinner {
                                s.clear();
                            }
                            output::error(&console, e.to_string());
                        }
                    }
                } else {
                    match mgr.delete(path) {
                        Ok(outcome) => {
                            if let Some(ref s) = spinner {
                                s.clear();
                            }
                            if let domain::DeleteOutcome::Trash { dest, .. } = &outcome {
                                output::show_delete(&console, dest);
                            }
                        }
                        Err(e) => {
                            if let Some(ref s) = spinner {
                                s.clear();
                            }
                            output::error(
                                &console,
                                format!("Error al mover a trash '{}': {}", path.display(), e),
                            );
                        }
                    }
                }
            }

            if let Some(s) = spinner {
                s.finish();
            }
        }
    }
}
