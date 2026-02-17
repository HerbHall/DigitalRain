# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.7.0] - 2026-02-16

### Added

- All 148 CSS Level 4 named colors available as palettes (e.g. `--color coral`)
- Auto-generated gradients from base color via HSL color space math
- 3 new hand-tuned multi-hue palettes: fire, ocean, synthwave
- Grouped `--list-colors` output: featured palettes + CSS colors in columns
- Total palette count: 150+
- MIT LICENSE file
- CHANGELOG.md, CONTRIBUTING.md, CODE_OF_CONDUCT.md
- GitHub Actions CI workflow (Windows + Linux)
- GitHub Actions release workflow with pre-built binaries
- GitHub issue and PR templates

### Changed

- Renamed "monochrome" palette to "silver" (CSS standard); monochrome kept as alias

## [0.6.0] - 2026-01-31

### Added

- 7 new visual effects: binary, cascade, pulse, glitch, fire, ocean, parallax
- Crossfade transitions: smooth per-cell color blending when switching effects (~0.75s)
- TOML configuration file with named presets
- `--preset <name>` to load saved configurations
- `--save-preset <name>` to save current CLI args as a named preset
- `--list-presets` to view available presets
- `--config <path>` to use a custom config file path
- Priority resolution: CLI > preset > config defaults > hardcoded defaults

## [0.5.0] - 2026-01-31

### Added

- CRT monitor post-processing filter (scanlines, phosphor glow, screen flicker, noise)
- `--crt` flag to enable CRT simulation at startup
- `--crt-intensity` to control effect strength (0.0-1.0, default 0.7)
- `c` key to toggle CRT on/off at runtime
- CRT included in randomization (~7% chance) for `r` key and auto-cycle timer

### Fixed

- `--timer` and `--crt` flags no longer discarded when combined with `--random`

## [0.3.1] - 2026-01-31

### Added

- `--timer <seconds>` flag to auto-randomize effect at a configurable interval
- `t` key to toggle auto-cycle on/off at runtime
- Timer pauses when animation is paused (Space key)
- Manual randomize (`r`) resets the timer countdown

## [0.3.0] - 2026-01-31

### Added

- Pause/resume with Space key
- Runtime speed adjustment with +/- keys
- Runtime density adjustment with [/] keys
- Cycle through effects with `n` key
- Randomize effect, palette, and speed with `r` key
- Keybindings help overlay toggled with `?` key
- Status message overlay for parameter change feedback
- Terminal resize handling for all controls

## [0.2.0] - 2026-01-31

### Added

- clap-based CLI argument parsing
- 6 color palettes: classic, gold, cyan, red, silver, purple
- 6 character sets: matrix, ascii, binary, digits, katakana, latin
- Configurable speed, density, and FPS
- Effect registry with `--list-effects`, `--list-colors`, `--list-charsets`
- `--random` flag for randomized parameters

## [0.1.0] - 2026-01-31

### Added

- Terminal setup/teardown with alternate screen and raw mode
- Double-buffered screen rendering with dirty-cell tracking
- Frame timing loop targeting 30 FPS
- Classic Matrix rain effect with multi-column rain
- True-color gradient trails (white-hot head to dark green tail)
- Half-width katakana + digit + symbol character set
- Gold highlight characters
- Character mutation (flickering)
- Variable column speeds and trail lengths

[Unreleased]: https://github.com/HerbHall/DigitalRain/compare/v0.7.0...HEAD
[0.7.0]: https://github.com/HerbHall/DigitalRain/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/HerbHall/DigitalRain/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/HerbHall/DigitalRain/compare/v0.3.1...v0.5.0
[0.3.1]: https://github.com/HerbHall/DigitalRain/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/HerbHall/DigitalRain/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/HerbHall/DigitalRain/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/HerbHall/DigitalRain/releases/tag/v0.1.0
