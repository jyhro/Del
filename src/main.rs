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
use trash::TrashManager;

#[cfg(target_os = "windows")]
fn get_trash_and_history() -> (PathBuf, PathBuf) {
    let userprofile = env::var("USERPROFILE").expect("No se pudo obtener USERPROFILE");
    let trash = PathBuf::from(format!("{}\\AppData\\Local\\Temp\\Trash", userprofile));
    let history = PathBuf::from(format!("{}\\AppData\\Local\\del_history", userprofile));
    (trash, history)
}

#[cfg(not(target_os = "windows"))]
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
    let (trash_dir, history_file) = get_trash_and_history();
    let args: Vec<String> = env::args().collect();
    let command = cli::parse_args(&args);

    match command {
        Command::Help => {}

        Command::ShowHistory => {
            if !history_file.exists() {
                output::show_no_history();
                return;
            }
            let mgr = make_mgr(trash_dir, history_file);
            match mgr.list_history() {
                Ok((entries, pruned)) => output::show_history(&entries, pruned),
                Err(e) => output::error(e.to_string()),
            }
        }

        Command::ClearHistory => {
            if !history_file.exists() {
                output::show_no_history();
                return;
            }
            output::show_clear_history_warning();
            match output::confirm() {
                Ok(true) => {
                    let mgr = make_mgr(trash_dir, history_file);
                    match mgr.clear_history() {
                        Ok(()) => output::show_history_cleared(),
                        Err(e) => output::error(e.to_string()),
                    }
                }
                Ok(false) => {}
                Err(e) => output::error(e.to_string()),
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
                Ok(outcome) => output::show_restore(&outcome),
                Err(Error::NoHistory) => output::show_no_archives(),
                Err(e) => output::error(e.to_string()),
            }
        }

        Command::Delete { files, permanent } => {
            let permanent_deleter = PermanentDeleter::new();
            let mgr = make_mgr(trash_dir, history_file);
            for path in &files {
                if !path.exists() {
                    output::error(format!("'{}' no existe", path.display()));
                    continue;
                }
                if permanent {
                    output::show_permanent_warning(path);
                    match output::confirm() {
                        Ok(true) => match permanent_deleter.delete(path) {
                            Ok(outcome) => output::show_delete(&outcome),
                            Err(e) => output::error(format!(
                                "Error al eliminar '{}': {}",
                                path.display(),
                                e
                            )),
                        },
                        Ok(false) => {}
                        Err(e) => output::error(e.to_string()),
                    }
                } else {
                    match mgr.delete(path) {
                        Ok(outcome) => output::show_delete(&outcome),
                        Err(e) => output::error(format!(
                            "Error al mover a trash '{}': {}",
                            path.display(),
                            e
                        )),
                    }
                }
            }
        }
    }
}
