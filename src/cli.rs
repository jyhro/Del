//! Parseo de argumentos y definicion de comandos.

use std::path::PathBuf;

use crate::output;
use rich_rust::prelude::*;

/// Acciones disponibles desde la CLI.
#[derive(Debug, PartialEq)]
pub enum Command {
    Delete {
        files: Vec<PathBuf>,
        permanent: bool,
        dry_run: bool,
        force: bool,
    },
    Restore {
        index: Option<usize>,
        dry_run: bool,
    },
    ShowHistory {
        dry_run: bool,
    },
    ClearHistory {
        dry_run: bool,
        force: bool,
    },
    Help,
    Version,
}

/// Convierte los argumentos en un comando ejecutable.
pub fn parse_args(console: &Console, args: &[String]) -> Command {
    if args.len() < 2 {
        output::print_usage();
        return Command::Help;
    }

    let mut permanent = false;
    let mut restore = false;
    let mut restore_index: Option<usize> = None;
    let mut show_history = false;
    let mut clear_history = false;
    let mut dry_run = false;
    let mut force = false;
    let mut help = false;
    let mut version = false;
    let mut files: Vec<PathBuf> = Vec::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-p" | "--permanent" => permanent = true,
            "-r" | "--restore" => {
                restore = true;
                if i + 1 < args.len() {
                    if let Ok(n) = args[i + 1].parse::<usize>() {
                        if let Some(idx) = n.checked_sub(1) {
                            restore_index = Some(idx);
                        }
                        i += 1;
                    } else if !args[i + 1].starts_with('-') {
                        output::warn(
                            console,
                            format!(
                                "'{}' no es un índice válido, se restaurará el último",
                                args[i + 1]
                            ),
                        );
                        i += 1;
                    }
                }
            }
            "--history" => show_history = true,
            "--clear-history" => clear_history = true,
            "--dry-run" => dry_run = true,
            "--force" => force = true,
            "--version" | "-v" => version = true,
            "--help" | "-h" => help = true,
            arg if !arg.starts_with('-') => files.push(PathBuf::from(arg.to_string())),
            arg => {
                if let Some(s) = suggest_flag(arg) {
                    output::unknown_flag_with_suggestion(console, arg, s);
                } else {
                    output::unknown_flag(console, arg);
                }
            }
        }
        i += 1;
    }

    if version {
        return Command::Version;
    }

    if help {
        output::print_usage();
        return Command::Help;
    }

    if show_history {
        return Command::ShowHistory { dry_run };
    }

    if clear_history {
        return Command::ClearHistory { dry_run, force };
    }

    if restore {
        return Command::Restore {
            index: restore_index,
            dry_run,
        };
    }

    if files.is_empty() {
        output::error(console, "Debe especificar al menos un archivo o carpeta");
        output::print_usage();
        return Command::Help;
    }

    Command::Delete {
        files,
        permanent,
        dry_run,
        force,
    }
}

/// Sugiere flags conocidos si el prefijo coincide lo suficiente.
fn suggest_flag(unknown: &str) -> Option<&'static str> {
    const KNOWN: &[&str] = &[
        "-p",
        "--permanent",
        "-r",
        "--restore",
        "--history",
        "--clear-history",
        "--dry-run",
        "--force",
        "--help",
        "-h",
        "--version",
        "-v",
    ];

    let mut best: Option<&'static str> = None;
    let mut best_score = 0usize;

    for &flag in KNOWN {
        let score = unknown
            .chars()
            .zip(flag.chars())
            .take_while(|(a, b)| a == b)
            .count();
        if score > best_score {
            best_score = score;
            best = Some(flag);
        }
    }

    if best_score >= 3 {
        best
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_delete_single_file() {
        let console = Console::new();
        let args = vec!["del".to_string(), "file.txt".to_string()];
        match parse_args(&console, &args) {
            Command::Delete {
                files,
                permanent,
                dry_run,
                force,
            } => {
                assert_eq!(files, vec![PathBuf::from("file.txt")]);
                assert!(!permanent);
                assert!(!dry_run);
                assert!(!force);
            }
            other => panic!("expected Command::Delete, got {:?}", other),
        }
    }

    #[test]
    fn test_parse_permanent_delete() {
        let console = Console::new();
        let args = vec!["del".to_string(), "-p".to_string(), "file.txt".to_string()];
        match parse_args(&console, &args) {
            Command::Delete {
                files, permanent, ..
            } => {
                assert!(permanent);
                assert_eq!(files.len(), 1);
            }
            other => panic!("expected Command::Delete, got {:?}", other),
        }
    }

    #[test]
    fn test_parse_restore_no_index() {
        let console = Console::new();
        let args = vec!["del".to_string(), "-r".to_string()];
        match parse_args(&console, &args) {
            Command::Restore { index, dry_run } => {
                assert_eq!(index, None);
                assert!(!dry_run);
            }
            other => panic!("expected Command::Restore, got {:?}", other),
        }
    }

    #[test]
    fn test_parse_restore_with_index() {
        let console = Console::new();
        let args = vec!["del".to_string(), "-r".to_string(), "3".to_string()];
        match parse_args(&console, &args) {
            Command::Restore { index, .. } => assert_eq!(index, Some(2)),
            other => panic!("expected Command::Restore, got {:?}", other),
        }
    }

    #[test]
    fn test_parse_show_history() {
        let console = Console::new();
        let args = vec!["del".to_string(), "--history".to_string()];
        assert_eq!(
            parse_args(&console, &args),
            Command::ShowHistory { dry_run: false }
        );
    }

    #[test]
    fn test_parse_clear_history() {
        let console = Console::new();
        let args = vec!["del".to_string(), "--clear-history".to_string()];
        assert_eq!(
            parse_args(&console, &args),
            Command::ClearHistory {
                dry_run: false,
                force: false
            }
        );
    }

    #[test]
    fn test_parse_force_permanent_delete() {
        let console = Console::new();
        let args = vec![
            "del".to_string(),
            "--force".to_string(),
            "--permanent".to_string(),
            "file.txt".to_string(),
        ];

        match parse_args(&console, &args) {
            Command::Delete {
                permanent, force, ..
            } => {
                assert!(permanent);
                assert!(force);
            }
            other => panic!("expected Command::Delete, got {:?}", other),
        }
    }

    #[test]
    fn test_parse_force_clear_history() {
        let console = Console::new();
        let args = vec![
            "del".to_string(),
            "--force".to_string(),
            "--clear-history".to_string(),
        ];

        assert_eq!(
            parse_args(&console, &args),
            Command::ClearHistory {
                dry_run: false,
                force: true
            }
        );
    }

    #[test]
    fn test_parse_help() {
        let console = Console::new();
        let args = vec!["del".to_string(), "--help".to_string()];
        assert_eq!(parse_args(&console, &args), Command::Help);
    }

    #[test]
    fn test_parse_version() {
        let console = Console::new();
        let args = vec!["del".to_string(), "--version".to_string()];
        assert_eq!(parse_args(&console, &args), Command::Version);
    }

    #[test]
    fn test_parse_version_short() {
        let console = Console::new();
        let args = vec!["del".to_string(), "-v".to_string()];
        assert_eq!(parse_args(&console, &args), Command::Version);
    }

    #[test]
    fn test_parse_multiple_files() {
        let console = Console::new();
        let args = vec!["del".to_string(), "a.txt".to_string(), "b.txt".to_string()];
        match parse_args(&console, &args) {
            Command::Delete { files, .. } => assert_eq!(files.len(), 2),
            other => panic!("expected Command::Delete, got {:?}", other),
        }
    }

    #[test]
    fn test_parse_dry_run_delete() {
        let console = Console::new();
        let args = vec![
            "del".to_string(),
            "--dry-run".to_string(),
            "file.txt".to_string(),
        ];
        match parse_args(&console, &args) {
            Command::Delete { dry_run, .. } => assert!(dry_run),
            other => panic!("expected Command::Delete, got {:?}", other),
        }
    }

    #[test]
    fn test_parse_dry_run_restore() {
        let console = Console::new();
        let args = vec!["del".to_string(), "--dry-run".to_string(), "-r".to_string()];
        match parse_args(&console, &args) {
            Command::Restore { dry_run, .. } => assert!(dry_run),
            other => panic!("expected Command::Restore, got {:?}", other),
        }
    }

    #[test]
    fn test_parse_dry_run_clear_history() {
        let console = Console::new();
        let args = vec![
            "del".to_string(),
            "--dry-run".to_string(),
            "--clear-history".to_string(),
        ];
        assert_eq!(
            parse_args(&console, &args),
            Command::ClearHistory {
                dry_run: true,
                force: false
            }
        );
    }

    #[test]
    fn test_suggest_flag_exact_match() {
        assert_eq!(suggest_flag("--permanent"), Some("--permanent"));
    }

    #[test]
    fn test_suggest_flag_prefix() {
        assert_eq!(suggest_flag("--perm"), Some("--permanent"));
    }

    #[test]
    fn test_suggest_flag_dry_run() {
        assert_eq!(suggest_flag("--dry"), Some("--dry-run"));
    }

    #[test]
    fn test_suggest_flag_short() {
        assert_eq!(suggest_flag("--x"), None);
    }

    #[test]
    fn test_suggest_flag_no_match() {
        assert_eq!(suggest_flag("--nonexistent"), None);
    }
}
