# del

A safe file deletion utility for Unix/macOS and Windows.

## Features

- **Safe deletion**: Move files to a trash directory instead of permanent deletion
- **Restore**: Restore files from trash by index or restore the last deleted file
- **History**: Track deletion history with timestamps and file sizes
- **Permanent deletion**: Secure overwrite + deletion that cannot be recovered
- **Cross-platform**: Works on Unix/Linux, macOS, and Windows

## Usage

```bash
# Delete a file to trash
del file.txt

# Delete multiple files
del file1.txt file2.txt folder/

# Restore the last deleted file
del -r

# Restore by history index (1-based)
del -r 3

# Show deletion history
del --history

# Clear all history
del --clear-history

# Delete permanently — secure overwrite + confirmation prompt
del -p important.log

# Show help
del --help
```

## Installation

### From source

```bash
git clone https://github.com/jyhro/Del.git
cd Del
./install.sh          # Unix/macOS
# or
./install.ps1         # Windows
```

### From crates.io

```bash
cargo install del
```

## How it works

Deleted files are moved to a platform-specific trash directory:

| Platform | Trash directory |
| --- | --- |
| Unix / macOS | `~/.local/share/Trash` |
| Windows | `%USERPROFILE%\AppData\Local\Temp\Trash` |

Files receive a timestamp suffix (e.g. `report.pdf_20260524_135031`) to avoid name collisions. A history file tracks the original path, filename, trash path, timestamp, and file size for restore operations:

| Platform | History file |
| --- | --- |
| Unix / macOS | `~/.local/share/del_history` |
| Windows | `%USERPROFILE%\AppData\Local\del_history` |

History is stored as pipe-delimited CSV: `original_path|file_name|trash_path|timestamp|size`.

### Permanent deletion

Permanent delete (`-p`) runs a three-pass secure erase:

1. **XOR encrypt** — loads file into memory, XORs every byte with `OsRng`
2. **Random overwrite** — writes two passes of cryptographically random bytes
3. **Remove** — deletes the file

Zero-length files are removed directly without overwrite. Directories are walked and each file is securely erased individually.

## Architecture

```txt
src/
├── main.rs        Entrypoint, wiring, platform paths
├── domain.rs      Domain types, traits, Error enum
├── output.rs      Console output, confirm prompts
├── cli.rs         CLI arg parsing → Command enum
├── trash.rs       Trash move/restore + history I/O
└── permanent.rs   Secure overwrite + delete logic
```

Business logic never prints or reads stdin. Modules return domain types and the output layer handles display.

## Development

```bash
cargo build           # debug build
cargo build --release # release build
cargo test            # run all tests
cargo test <name>     # single test by name
cargo run -- <args>   # run the CLI
```

## License

MIT
