# Roadmap (Upcoming releases)

## Completed tasks

- [x] Move files/folders to trash with timestamp suffix
- [x] Deletion history in a file (read / append / replace, pipe-delimited format)
- [x] Show history (`--history`) and clear history (`--clear-history`) with confirmation
- [x] Restore last or by index (`-r/--restore [N]`) and handle restore conflicts (`_restaurado`)
- [x] Prune stale history entries automatically when listing
- [x] Calculate file/folder sizes and human-readable formatting (`format_size`)
- [x] Secure permanent delete with in-memory encryption and random passes (PermanentDeleter)
- [x] Interactive confirmations (s/n) for dangerous actions
- [x] Unknown flag suggestion (did-you-mean)
- [x] Messages and help in Spanish with colored output
- [x] Basic OS support for Trash and history location (Windows / Unix)
- [x] Inline unit tests for main modules

## Improvements and features

- [x] UX improvements: clearer and consistent messages (rich_rust), final summary with counts, spinner during batch operations
- [ ] Dry-run mode (`--dry-run`) to simulate actions without touching files
- [ ] Advanced history listing and search (filter by date, name, extension, size)
- [ ] Interactive restore with selector by index
- [ ] Pattern (glob) support and exclusions (`--exclude`)
- [ ] Configurable confirmations (`--yes`, `--no`) and remember last choice
- [ ] Integrate with system Trash on macOS/Linux instead of a custom folder
- [ ] Automatic trash size/space limit with LRU policy
- [ ] Export/import history (CSV/JSON)
- [ ] Stats (`del stats`): file counts, space saved, monthly trends
- [ ] Multi-language localization (keeping Spanish as default)
- [ ] Detailed logs with levels (`--verbose`, `--quiet`) — include animated progress with rich_rust Live + ProgressBar
- [ ] Support restore to a different folder (`--restore-to`)
- [ ] `undo` command to revert last deletion
- [ ] Security improvements for permanent delete (more passes, patterns, optional verification)
- [ ] Confirmation hook for CI with `--force`
- [ ] Additional edge-case tests (long paths, permissions, symlinks)
