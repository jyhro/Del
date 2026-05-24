# Contributing Guide (EN)

Thanks for contributing to del.

## Scope

- del is a Rust CLI binary (edition 2024).
- There is no lib.rs and no integration tests.
- The UI is in Spanish; prompts and messages should remain in Spanish.

## Requirements

- Stable Rust installed.

## Useful commands

```bash
cargo build
cargo build --release
cargo test
cargo test <name>
cargo run -- --help
```

## Project structure

```txt
src/
  main.rs      Entrypoint and wiring
  domain.rs    Domain types and logic
  output.rs    All console output
  cli.rs       Arg parsing and help
  history.rs   History IO
  trash.rs     Move to trash
  permanent.rs Permanent delete
```

## Code guidelines

- Business logic must not print or read stdin.
- All output goes through output.rs.
- Keep messages consistent and clear.
- Add inline tests inside each module when it applies.

## Changes and PRs

- Open one PR per logical change.
- Include a short, clear description.
- Add or update tests when relevant.
- Avoid unnecessary mass formatting.

## Bug reports

- Include OS details.
- Steps to reproduce.
- Expected vs actual result.
