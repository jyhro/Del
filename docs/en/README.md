# del — Documentation (EN)

A small, cross-platform CLI for safer file deletion on Unix/macOS and Windows.

Core goals: move files to a trash folder, track history for easy restores, and provide an optional secure permanent delete.

**Highlights**

- Safe local trash instead of immediate permanent deletion
- Restore by history index or restore the most recent deletion
- Track timestamps and sizes in a compact history file
- Optional secure overwrite (`-p`) for irreversible deletion
- Human-friendly terminal output via `rich_rust`

## Quick examples

Delete a single file to the trash:

```bash
del file.txt
```

Delete multiple files or directories:

```bash
del file1.txt file2.txt my_folder/
```

Restore the last deleted entry:

```bash
del -r
```

Restore a specific history entry (1-based index):

```bash
del -r 3
```

List history and metadata:

```bash
del --history
```

Permanently and securely delete a file (requires confirmation):

```bash
del -p sensitive.log
```

Show help:

```bash
del --help
```

## Installation

From source:

```bash
git clone https://github.com/jyhro/Del.git
cd Del
./install.sh          # Unix/macOS
# or
./install.ps1         # Windows (PowerShell)
```

From crates.io:

```bash
cargo install del
```

## How it works (overview)

- Trash directory (platform-specific): files are moved here instead of removed.
- Files are renamed with a timestamp suffix to avoid collisions.
- A compact pipe-delimited history file records: `original_path|file_name|trash_path|timestamp|size`.

Platform defaults:

| Platform | Trash directory | History file |
| --- | --- | --- |
| MacOS | `~/.Trash` | `~/.del_history` |
| Unix | `~/.local/share/Trash` | `~/.local/share/del_history` |
| Windows | `%USERPROFILE%\\AppData\\Local\\Temp\\Trash` | `%USERPROFILE%\\AppData\\Local\\del_history` |

### Permanent delete (`-p`)

Permanent deletion performs the following steps (safe-by-default behavior documented here):

1. XOR in-memory pass using randomness from `OsRng` (memory-backed obfuscation)
2. Two passes of cryptographically-random overwrites to the file contents
3. Remove the file from disk

Zero-length files are removed directly without overwrite. Directories are processed recursively and each file is erased individually.

## Project layout

```txt
src/
├── main.rs        # Entrypoint and wiring (Console + Summary lifecycle)
├── domain.rs      # Domain types, errors, Summary counters
├── output.rs      # Rich terminal output and prompts (Spanish UI)
├── cli.rs         # Argument parsing → `Command` enum
├── trash.rs       # Move/restore + history repository implementation
└── permanent.rs   # Secure overwrite + delete logic
```

Design note: business logic never reads stdin or prints directly — `output.rs` handles all I/O and formatting.

## Development

Build and run locally:

```bash
cargo build           # debug build
cargo build --release # release build
cargo run -- <args>   # run the CLI
```

Run tests:

```bash
cargo test            # run unit tests (inline)
cargo test <name>     # run a single test by name
```

## Contributing

Contributions are welcome. See the contribution guides for the workflow and style guidelines:

- [CONTRIBUTING](CONTRIBUTING.md)

When adding features that change user-visible text, note this project currently uses Spanish UI strings in `output.rs`.

## Roadmap & issues

See [ROADMAP.md](../ROADMAP.md) for planned improvements. Report bugs or feature requests via GitHub issues.

## License

This project is licensed under the MIT License — see [LICENSE](../LICENSE).

## Acknowledgements

- `rich_rust` for terminal formatting
- Inspiration: traditional desktop trash / undelete workflows
