# del — agent instructions

Single Rust binary crate (edition 2024). No workspace, no lib.rs, no integration tests.

## Module structure

```txt
src/
├── main.rs        Entrypoint, wiring, platform paths (#[cfg] blocks), Console + Summary lifecycle
├── domain.rs      HistoryEntry, Error enum, Delete/Restore/HistoryRepository traits, Summary, format_size, prune_stale_entries
├── output.rs      All console output via rich_rust (Console, markup, Table, Panel, Spinner), confirm prompts
├── cli.rs         Arg parsing → Command enum, did-you-mean, print_usage
├── history.rs     FileHistoryRepository: history file I/O, CSV parse/serialize
├── trash.rs       TrashManager: file move I/O, depends on Box<dyn HistoryRepository>
└── permanent.rs   PermanentDeleter: secure overwrite + delete logic (no printing)
```

Architecture: **business logic never prints or reads stdin**. Modules return domain types (`DeleteOutcome`, `RestoreOutcome`, `Error`) and the output layer formats them. `TrashManager` depends on `Box<dyn HistoryRepository>` (DIP) — swap the repository to change storage format.

## Dev commands

```bash
cargo build               # debug build
cargo build --release     # release build
cargo test                # all tests (inline unit tests only, no --test dir)
cargo test <name>         # single test by fn name (run from project root)
cargo run -- <args>       # run the CLI, e.g. cargo run -- --help
cargo run -- file.txt     # move file.txt to trash
```

No CI config, formatter config, or linter config committed.

## Key facts an agent is likely to miss

- **UI language is Spanish** — all prompts, errors, help text, and status messages are in Spanish.
- **No lib.rs** — modules are declared in `main.rs` via `mod`; not importable as a library.
- **History file** location: `~/.local/share/del_history` (Unix) or `%USERPROFILE%\AppData\Local\del_history` (Windows). Format is pipe-delimited CSV: `original_path|file_name|trash_path|timestamp|size`.
- **Trash directory**: `~/.local/share/Trash` (Unix) or `%USERPROFILE%\AppData\Local\Temp\Trash` (Windows).
- **Permanent delete** (`-p`) uses XOR in-memory encryption + 2 random overwrite passes via `OsRng`. Zero-length files are removed without overwrite. Requires interactive `s/n` confirmation on stdin.
- **`--clear-history`** also requires interactive `s/n` confirmation.
- **Flag did-you-mean** — unknown flags with 3+ prefix-char matches get a suggestion; otherwise "Flag desconocido". Handled in `cli.rs`.
- **Tests are inline** `#[cfg(test)] mod tests` blocks inside `cli.rs`, `domain.rs`, `history.rs`, `trash.rs`, and `permanent.rs`. No separate test files.
- **Test cleanup** — some tests create temp dirs via `std::env::temp_dir()` and attempt cleanup with `remove_dir_all` dropped inside `unwrap_or(())`; they can leave residue on failure.
- **Console output and user interaction** are centralized in `output.rs`. To change how messages display, only modify that file.
- **CLI parsing** returns a `Command` enum. To add a new subcommand, add a variant to `Command`, update `cli::parse_args`, and handle it in `main.rs`.

## rich_rust integration

All styled output uses `rich_rust::prelude::*`. Key types:
- **`Console`** — created once in `main.rs`, passed as `&Console` to every output function. Uses markup syntax: `[bold green]✓[/]`, `[bold red]✗[/]`, `[yellow]⚠[/]`.
- **`Table`** — used in `show_history()` for the history display with columns.
- **`Panel`** — used in `show_summary()` for the final summary box.
- **`Spinner`** — simple struct with `tick()`/`finish()` for batch operation feedback (not from rich_rust, defined in `output.rs`).

The `Console` parameter is threaded through `cli::parse_args` too (for error/warning output during arg parsing). Tests create a throwaway `Console::new()`.

## Summary tracking

`domain::Summary` tracks counts across batch operations. Created in `main.rs`, passed through operation loops, and displayed via `output::show_summary()` at the end. Fields: `trash_count`, `permanent_count`, `restore_count`, `fail_count`, `cancel_count`. Methods: `record_delete`, `record_restore`, `record_fail`, `record_cancel`, `has_work`.

## Spinner usage

For batch normal deletes (≥2 files in one command), a `Spinner` shows a rotating `|/-\` cursor with progress `[current/total] filename`. The spinner line is overwritten on each tick via `\r`. After the loop, `finish()` advances to a new line. Permanent deletes skip the spinner (per-file confirmation makes it impractical).
