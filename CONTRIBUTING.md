# Contributing to DigitalRain

Thanks for your interest in contributing! This guide covers everything you need to get started.

## Development Setup

### Prerequisites

- [Rust](https://rustup.rs/) 1.85 or later
- A terminal that supports true color (24-bit RGB)

### Building

```bash
git clone https://github.com/HerbHall/DigitalRain.git
cd DigitalRain
cargo build
```

### Running

```bash
cargo run -- --help       # Show all options
cargo run                 # Run with defaults
cargo run -- --color cyan # Run with a specific palette
```

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix all warnings
- Run `cargo test` and ensure all tests pass
- Prefer clarity over cleverness -- this project values readable code

## Making Changes

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Make your changes
4. Run the checks:

   ```bash
   cargo fmt
   cargo clippy
   cargo test
   ```

5. Commit with a descriptive message using [conventional commits](https://www.conventionalcommits.org/):
   - `feat:` for new features
   - `fix:` for bug fixes
   - `docs:` for documentation
   - `refactor:` for code restructuring
   - `test:` for adding tests
   - `chore:` for maintenance tasks
6. Push and open a pull request

## Adding a New Effect

Effects implement the `Effect` trait in [src/effects/mod.rs](src/effects/mod.rs). To add one:

1. Create `src/effects/your_effect.rs`
2. Implement the `Effect` trait (`name()`, `update()`, `render()`)
3. Register it in [src/effects/registry.rs](src/effects/registry.rs)
4. Add tests

## Adding a New Palette

Hand-tuned palettes go in [src/color/palette.rs](src/color/palette.rs). Add a constructor method on `Palette` and register it in `palette_by_name()` and `HAND_TUNED_NAMES`.

CSS named colors are automatically available as palettes via [src/color/css_colors.rs](src/color/css_colors.rs).

## Reporting Bugs

Use the [bug report template](https://github.com/HerbHall/DigitalRain/issues/new?template=bug_report.md) and include:

- Your terminal emulator and OS
- The command you ran
- What you expected vs. what happened
- A screenshot if it's a visual issue

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
