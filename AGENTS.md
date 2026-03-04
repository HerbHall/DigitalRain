<!--
  Scope: AGENTS.md guides the Copilot coding agent and Copilot Chat.
  For code completion and code review patterns, see .github/copilot-instructions.md
  For Claude Code, see CLAUDE.md
-->

# DigitalRain

Matrix-style digital rain terminal visual effect written in Rust.

## Tech Stack

- **Language**: Rust (edition 2024, MSRV 1.85)
- **Terminal**: crossterm 0.29 (cross-platform terminal manipulation)
- **CLI**: clap 4 (command-line argument parsing with derive macros)
- **Config**: toml + serde (TOML config file parsing)
- **RNG**: rand 0.9 (random number generation for rain effects)
- **Paths**: dirs 6 (platform-specific config directory resolution)

## Build and Test Commands

```bash
# Build
cargo build

# Build release (optimized)
cargo build --release

# Test
cargo test

# Lint
cargo clippy -- -D warnings

# Format check
cargo fmt --check

# Full verification (run before any PR)
cargo build && cargo test && cargo clippy -- -D warnings && cargo fmt --check
```

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
├── assets/              - Static assets (screenshots, etc.)
├── docs/                - Project documentation
├── scripts/             - Build and CI helper scripts
├── .github/             - CI workflows and Copilot config
├── Cargo.toml           - Rust package manifest
└── CLAUDE.md            - Claude Code instructions
```

## Workflow Rules

### Always Do

- Create a feature branch for every change (`feature/issue-NNN-description`)
- Use conventional commits: `feat:`, `fix:`, `refactor:`, `docs:`, `test:`, `chore:`
- Run build, test, and lint before opening a PR
- Use the `?` operator for error propagation with meaningful context
- Fix every error you find, regardless of who introduced it
- Use `cargo fmt` to format code before committing

### Ask First

- Adding new dependencies (check if `std` covers the need)
- Architectural changes (new modules, major trait changes)
- Changes to CI/CD workflows
- Removing or renaming public APIs or CLI flags
- Changes to the config file schema

### Never Do

- Commit directly to `main` -- always use feature branches
- Skip tests or lint checks -- even for "small changes"
- Use `--no-verify` or `--force` flags
- Commit secrets, credentials, or API keys
- Add TODO comments without a linked issue number
- Mark work as complete when build, test, or lint failures remain

## Core Principles

These are unconditional -- no optimization or time pressure overrides them:

1. **Quality**: Once found, always fix, never leave. There is no "pre-existing" error.
2. **Verification**: Build, test, and lint must pass before any commit.
3. **Safety**: Never force-push `main`. Never skip hooks. Never commit secrets.
4. **Honesty**: Never mark work as complete when it is not.

## Error Handling

```rust
// Use the ? operator to propagate errors with context
fn load_config(path: &Path) -> Result<Config, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("failed to read config at {}: {}", path.display(), e))?;
    let config: Config = toml::from_str(&content)
        .map_err(|e| format!("failed to parse config: {}", e))?;
    Ok(config)
}

// Use Option for values that may be absent (not sentinel values)
fn find_column(&self, x: u16) -> Option<&RainColumn> {
    self.columns.iter().find(|c| c.x == x)
}
```

## Testing Conventions

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default_values() {
        let config = Config::default();
        assert!(config.speed > 0.0);
        assert!(!config.color_scheme.is_empty());
    }

    #[test]
    fn test_buffer_dimensions() {
        let buffer = Buffer::new(80, 24);
        assert_eq!(buffer.width(), 80);
        assert_eq!(buffer.height(), 24);
    }
}
```

## Commit Format

```text
feat: add configurable rain density

Adds --density CLI flag and config option to control drop spawn rate.

Closes #42
Co-Authored-By: GitHub Copilot <copilot@github.com>
```

Types: `feat` (new feature), `fix` (bug fix), `refactor` (no behavior change),
`docs` (documentation only), `test` (tests only), `chore` (build/tooling).
