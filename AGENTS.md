# del — agent instructions

Single Rust binary crate (edition 2024). No workspace, no lib.rs, no integration tests.

## Module structure

```
src/
├── main.rs        Entrypoint, wiring, platform paths (#[cfg] blocks)
├── domain.rs      HistoryEntry, Error enum, Delete/Restore traits, format_size, prune_stale_entries
├── output.rs      All console output (println!/eprintln!/colored), confirm prompts
├── cli.rs         Arg parsing → Command enum, did-you-mean, print_usage
├── trash.rs       TrashManager: file move I/O, history file I/O (no printing)
└── permanent.rs   PermanentDeleter: secure overwrite + delete logic (no printing)
```

Architecture: **business logic never prints or reads stdin**. Modules return domain types (`DeleteOutcome`, `RestoreOutcome`, `Error`) and the output layer formats them.

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
- **Tests are inline** `#[cfg(test)] mod tests` blocks inside `cli.rs`, `domain.rs`, `trash.rs`, and `permanent.rs`. No separate test files.
- **Test cleanup** — some tests create temp dirs via `std::env::temp_dir()` and attempt cleanup with `remove_dir_all` dropped inside `unwrap_or(())`; they can leave residue on failure.
- **Console output and user interaction** are centralized in `output.rs`. To change how messages display, only modify that file.
- **CLI parsing** returns a `Command` enum. To add a new subcommand, add a variant to `Command`, update `cli::parse_args`, and handle it in `main.rs`.
