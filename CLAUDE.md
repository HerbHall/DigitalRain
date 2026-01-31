# DigitalRain - Project Instructions

## Overview
Terminal-based Matrix digital rain effect built in Rust using crossterm.
Classic green phosphor CRT aesthetic with gold highlight characters.

## Tech Stack
- **Language**: Rust (edition 2024)
- **Terminal**: crossterm (cross-platform, Windows-first)
- **CLI**: clap (argument parsing)
- **RNG**: rand (character selection, timing)

## Architecture
- Effect trait system: each visual effect implements a common `Effect` trait
- Double-buffered cell grid: compose frame in memory, flush once per frame
- CLI argument parsing for effect selection and parameter tuning
- TOML config file support for saving presets

## Key Design Decisions
- Windows is the primary target; Linux/macOS are secondary
- True color (24-bit RGB) for smooth gradients; graceful degradation to 256-color
- Half-width katakana (U+FF66-U+FF9F) for Matrix-authentic characters
- Single binary distribution (no runtime dependencies)

## Build & Run
```bash
cargo build --release
cargo run -- --help
```

## Project Structure
```
src/
  main.rs           - Entry point, CLI args, main loop
  terminal.rs       - crossterm setup/teardown, raw mode, alternate screen
  buffer.rs         - 2D cell buffer (char + fg/bg color per cell)
  timing.rs         - Frame timing, FPS control, delta time
  config.rs         - Configuration structs, TOML loading, CLI mapping
  color/
    mod.rs          - Color types and utilities
    palette.rs      - Named color palettes (classic, gold, custom)
    gradient.rs     - Linear interpolation for trail fading
  effects/
    mod.rs          - Effect trait, registry, effect selection
    classic.rs      - Classic Matrix rain
    (future effects added here)
  rain/
    mod.rs          - Rain simulation coordinator
    column.rs       - Individual rain column state
    chars.rs        - Character set definitions (katakana, ASCII, etc.)
```

## Conventions
- Keep code well-commented (developer is learning Rust)
- Prefer clarity over cleverness
- Use `cargo clippy` and `cargo fmt` before committing
- keep Readme.md file updated for GitHub repo
- Maintain documentation with version history and updated command usage.
- implement tests for debuging
