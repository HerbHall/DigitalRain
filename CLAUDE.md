# DigitalRain - Project Instructions

## Overview
Terminal-based Matrix digital rain effect built in Rust using crossterm.
Classic green phosphor CRT aesthetic with gold highlight characters.

## Tech Stack
- **Language**: Rust (edition 2024)
- **Terminal**: crossterm (cross-platform, Windows-first)
- **CLI**: clap (argument parsing)
- **RNG**: rand (character selection, timing)
- **Config**: toml + serde (TOML configuration file with presets)
- **Platform**: dirs (platform-native config directory)

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
  main.rs           - Entry point, CLI args, main loop, crossfade wiring
  terminal.rs       - crossterm setup/teardown, raw mode, alternate screen
  buffer.rs         - 2D cell buffer (char + fg/bg color per cell)
  timing.rs         - Frame timing, FPS control, delta time
  config.rs         - CLI parsing, TOML config file, presets, resolution
  crt.rs            - CRT monitor simulation post-processing filter
  transition.rs     - Crossfade transitions between effects
  overlay.rs        - Help and status message overlays
  color/
    mod.rs          - Color types and utilities
    palette.rs      - Named color palettes (classic, gold, custom)
    gradient.rs     - Linear interpolation for trail fading, scale_color
  effects/
    mod.rs          - Effect trait definition
    registry.rs     - Effect discovery, creation, and listing
    classic.rs      - Classic Matrix rain
    binary.rs       - Dense binary 0/1 data stream
    cascade.rs      - Wave-front column spawning
    pulse.rs        - Rain with brightness wave overlay
    glitch.rs       - Rain with digital corruption events
    fire.rs         - Cellular automata fire simulation
    ocean.rs        - Sine-wave water surface simulation
    parallax.rs     - Multi-layer rain with depth
  rain/
    mod.rs          - Rain simulation coordinator, render_rain_column()
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
- Commit to GitHub after successful build.
- Start a new github branch before implementing new features or making large changes to the core.
- Please include proper references in the code and support files for sources used in building this project.
- give credit if we use someone's code or data. 