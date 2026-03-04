# DigitalRain -- Copilot Instructions

Matrix-style digital rain terminal visual effect written in Rust.

## Tech Stack

- **Language**: Rust (edition 2024, MSRV 1.85)
- **Terminal**: crossterm 0.29 (cross-platform terminal manipulation)
- **CLI**: clap 4 with derive macros
- **Config**: toml + serde for TOML configuration files
- **RNG**: rand 0.9 for randomized rain behavior

## Project Structure

```text
DigitalRain/
├── src/
│   ├── main.rs          - Entry point, CLI parsing, main loop
│   ├── buffer.rs        - Terminal buffer management
│   ├── config.rs        - Configuration loading and defaults
│   ├── crt.rs           - CRT monitor visual effect
│   ├── overlay.rs       - Text overlay rendering
│   ├── terminal.rs      - Terminal setup and teardown
│   ├── timing.rs        - Frame timing and tick control
│   ├── transition.rs    - Visual transition effects
│   ├── color/           - Color schemes and gradient logic
│   ├── effects/         - Visual effect implementations
│   └── rain/            - Core rain drop simulation
├── .github/             - CI workflows and Copilot config
└── CLAUDE.md            - Claude Code instructions
```

## Code Style

- Conventional commits: `feat:`, `fix:`, `refactor:`, `docs:`, `test:`, `chore:`
- Co-author tag: `Co-Authored-By: GitHub Copilot <noreply@github.com>`
- Errors propagated with `?` operator and meaningful context via `.map_err()`
- Tests use `#[cfg(test)]` modules with descriptive function names
- All lint checks must pass before committing (`cargo clippy -- -D warnings`)
- Code must be formatted with `cargo fmt`

## Coding Guidelines

- Fix errors immediately -- never classify them as pre-existing
- Build, test, and lint must pass before any commit
- Never skip hooks (`--no-verify`) or force-push main
- Remove unused code completely; no backwards-compatibility hacks
- Prefer `Option` over sentinel values for absent data
- Use `match` or `if let` for pattern matching; avoid nested `unwrap()` chains
- Keep `unsafe` blocks to an absolute minimum; document safety invariants

## Available Commands

```bash
cargo build              # Compile the project (debug)
cargo build --release    # Compile optimized release binary
cargo test               # Run all tests
cargo clippy -- -D warnings  # Run linter (warnings are errors)
cargo fmt --check        # Verify formatting
cargo run                # Run the application
cargo run -- --help      # Show CLI help
```

## Do NOT

- Add `#[allow(clippy::...)]` directives without fixing the root cause first
- Use `unwrap()` or `expect()` in library/non-main code without justification
- Commit generated files without regenerating them first
- Add dependencies without running `cargo build` to update `Cargo.lock`
- Use `panic!` in library code; return `Result` instead
- Store secrets, tokens, or credentials in code or config files
- Mark work as complete when known errors remain
- Use `unsafe` without a `// SAFETY:` comment explaining the invariant
