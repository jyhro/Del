# Issues achieved

## Move files/folders to trash with timestamp suffix

**Description**
Moves the item to the trash folder with a unique name based on date/time to avoid collisions.

## Deletion history in a file (read / append / replace, pipe-delimited format)

**Description**
Persists and retrieves history in a pipe-delimited text file, supporting read, append and replace.

## Show history (--history) and clear history (--clear-history) with confirmation

**Description**
Lists history and allows clearing it with interactive confirmation to avoid accidental deletes.

## Restore last or by index (-r/--restore [N]) and handle restore conflicts (_restaurado)

**Description**
Restores the most recent item or one by index; if destination exists, rename with suffix _restaurado.

## Prune stale history entries automatically when listing

**Description**
Removes records whose file in trash no longer exists to keep history consistent.

## Calculate file/folder size and human-readable format (format_size)

**Description**
Calculates actual sizes of files and folders and displays them in readable units.

## Secure permanent delete with in-memory encryption and random passes (PermanentDeleter)

**Description**
Overwrites data with in-memory encryption and random passes before permanently deleting.

## Interactive confirmations (s/n) for dangerous actions

**Description**
Prompts stdin confirmation before destructive operations to prevent mistakes.

## Unknown flag suggestion (did-you-mean)

**Description**
Suggests the correct flag when a prefix matches a known option.

## Messages and help in Spanish with colored output

**Description**
Centralizes CLI texts and help in Spanish and uses colors for states.

## Basic OS support for Trash and history location (Windows / Unix)

**Description**
Defines platform-specific base paths for trash and history.

## Inline unit tests for main modules

**Description**
Includes local tests in main modules to validate core behavior.

# Pending issues

## UX improvements: clearer and consistent messages, and a final summary with counts

**Description**
Unify texts and add a final summary after each run with action counts.

## Dry-run mode ("--dry-run") to simulate actions without touching files

**Description**
Shows what would be done without modifying filesystem or history.

## Advanced history listing and search (filter by date, name, extension, size)

**Description**
Adds filters and sorting to find entries quickly.

## Interactive restore with selector by index

**Description**
Allows choosing item to restore via a numbered prompt.

## Pattern (glob) support and exclusion ("--exclude")

**Description**
Accepts file patterns and exclusion rules when deleting.

## Configurable confirmations ("--yes", "--no") and remember last choice

**Description**
Supports non-interactive mode and repeat previous decisions.

## Integrate with system Trash on macOS and Linux instead of a custom folder

**Description**
Use native system Trash for better compatibility and recovery.

## Automatic trash size/space limit with LRU policy

**Description**
Automatically cleans when limit exceeded using LRU policy.

## Export/import history (CSV/JSON)

**Description**
Allows backing up and restoring history in standard formats.

## Stats ("del stats"): file counts, space saved, monthly trends

**Description**
Adds a command for usage metrics and trends.

## Multi-language localization (keeping Spanish as default)

**Description**
Enables translations with Spanish as the default language.

## Detailed logs with levels ("--verbose", "--quiet")

**Description**
Exposes verbosity levels for diagnostics or silence.

## Support restore to a different folder ("--restore-to")

**Description**
Allows restoring to a different path than the original.

## `undo` command to revert last deletion

**Description**
Shortcut to restore the most recent item with a single command.

## Security improvements for permanent delete (more passes, patterns, optional verification)

**Description**
Hardens deletion with more passes, patterns and optional verification.

## Confirmation hook for CI with "--force"

**Description**
Skips interactive prompts in pipelines and scripts.

## Additional edge-case tests (long paths, permissions, symlinks)

**Description**
Covers edge cases that fail on some systems.
