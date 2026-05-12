# del

A safe file deletion utility for Unix/macOS and Windows.

## Features

- **Safe deletion**: Move files to a trash directory instead of permanent deletion
- **Restore**: Restore files from trash by index or restore the last deleted file
- **History**: Track deletion history with timestamps and file sizes
- **Permanent deletion**: Securely delete files that cannot be recovered
- **Cross-platform**: Works on Unix/Linux, macOS, and Windows

## Usage

```bash
# Delete a file to trash
del file.txt

# Delete multiple files
del file1.txt file2.txt folder/

# Restore the last deleted file
del -r

# Restore by history index
del -r 3

# Show deletion history
del --history

# Clear all history
del --clear-history

# Delete permanently (with confirmation)
del -p important.log

# Show help
del --help
```

## How it works

Deleted files are moved to a trash directory (`~/.local/share/Trash` on Unix/macOS) with a timestamp suffix to avoid name collisions. A history file tracks original paths, timestamps, and file sizes for restore operations.

## Installation

```bash
cargo install del
```

## License

MIT
