# DigitalRain

Terminal-based Matrix digital rain effect built in Rust. Cross-platform with Windows as the primary target.

Features true-color gradient trails, film-authentic katakana characters, gold highlights, character mutation, and a flexible effect system.

## Features

- **True-color gradients**: Smooth 24-bit RGB fading from bright white head through vivid green to dark tail
- **Film-authentic characters**: Half-width katakana mixed with digits and symbols
- **Gold highlights**: Occasional gold characters like in the original Matrix films
- **Character mutation**: Characters flicker and change over time
- **Multiple color palettes**: Classic green, gold, cyan, red, monochrome, purple
- **Multiple character sets**: Matrix, ASCII, binary, digits, katakana, latin
- **Configurable**: Speed, density, FPS, palette, and charset via CLI flags
- **Interactive controls**: Adjust speed, density, and effects in real-time with keyboard
- **Cross-platform**: Windows Terminal, PowerShell, cmd.exe, Linux, macOS

## Installation

### From source

Requires [Rust](https://rustup.rs/) 1.85+.

```bash
git clone https://github.com/YOUR_USERNAME/DigitalRain.git
cd DigitalRain
cargo build --release
```

The binary will be at `target/release/digital_rain.exe` (Windows) or `target/release/digital_rain` (Linux/macOS).

### Run directly

```bash
cargo run --release
```

## Usage

```
digital_rain [OPTIONS]
```

Press `q`, `Esc`, or `Ctrl+C` to quit. Press `?` while running to show the keybindings help overlay.

### Options

| Flag | Short | Description | Default |
|---|---|---|---|
| `--effect <name>` | `-e` | Effect to display | `classic` |
| `--speed <value>` | `-s` | Speed multiplier (0.1-10.0) | `1.0` |
| `--density <value>` | `-d` | Rain density (0.1-10.0) | `1.0` |
| `--color <palette>` | `-c` | Color palette | `classic` |
| `--charset <name>` | | Character set | `matrix` |
| `--fps <value>` | | Target frame rate | `30` |
| `--list-effects` | | List available effects | |
| `--list-colors` | | List available palettes | |
| `--list-charsets` | | List available character sets | |
| `--random` | | Random effect and parameters | |
| `--help` | `-h` | Show help | |
| `--version` | `-V` | Show version | |

### Color Palettes

| Name | Description |
|---|---|
| `classic` | Green phosphor (the Matrix default) |
| `gold` | Warm amber/gold CRT feel |
| `cyan` | Cold ice-blue digital |
| `red` | Crimson danger/alert |
| `monochrome` | White/grey on black |
| `purple` | Violet synthwave |

### Character Sets

| Name | Description |
|---|---|
| `matrix` | Half-width katakana + digits + symbols (film-authentic) |
| `ascii` | Full printable ASCII characters |
| `binary` | 0 and 1 only |
| `digits` | 0-9 only |
| `katakana` | Half-width katakana only |
| `latin` | A-Z, a-z letters |

### Examples

```bash
# Classic green Matrix rain
digital_rain

# Red rain, double speed, sparse density
digital_rain --color red --speed 2.0 --density 0.5

# Binary rain in cyan (hacker aesthetic)
digital_rain --charset binary --color cyan

# Slow gold rain, heavy density
digital_rain --color gold --speed 0.5 --density 2.0

# Purple synthwave at 60fps
digital_rain --color purple --fps 60

# Fully randomized effect and parameters
digital_rain --random
```

### Interactive Controls

While running, use these keys to adjust the rain in real-time:

| Key | Action |
|---|---|
| `Space` | Pause / Resume |
| `+` / `=` | Speed up (0.2x per press) |
| `-` | Speed down (0.2x per press) |
| `]` | Density up (0.2x per press) |
| `[` | Density down (0.2x per press) |
| `n` | Next effect |
| `r` | Randomize (effect, palette, speed) |
| `?` | Toggle keybindings help overlay |
| `q` / `Esc` | Quit |

Speed and density are clamped to the range 0.1x - 10.0x. Status messages appear briefly at the bottom of the screen when parameters change.

## Planned Features

- CRT simulation (scanlines, phosphor glow, flicker)
- Depth/parallax layers (foreground + background rain at different speeds)
- Additional effects: cascade, pulse, glitch, binary, fire, ocean
- TOML configuration file with named presets
- Effect transitions (smooth crossfade between effects)

## Version History

### v0.3.0 - Interactive Controls & Polish
- Pause/resume with Space key
- Runtime speed adjustment with +/- keys (0.2x steps, clamped 0.1-10.0)
- Runtime density adjustment with [/] keys (0.2x steps, clamped 0.1-10.0)
- Cycle through effects with `n` key
- Randomize effect, palette, and speed with `r` key
- Keybindings help overlay toggled with `?` key
- Status message overlay for parameter change feedback
- Terminal resize handling for all controls

### v0.2.0 - CLI & Configuration
- clap-based CLI argument parsing
- 6 color palettes: classic, gold, cyan, red, monochrome, purple
- 6 character sets: matrix, ascii, binary, digits, katakana, latin
- Configurable speed, density, and FPS
- Effect registry with `--list-effects`, `--list-colors`, `--list-charsets`
- `--random` flag for randomized parameters

### v0.1.0 - Foundation
- Terminal setup/teardown with alternate screen and raw mode
- Double-buffered screen rendering with dirty-cell tracking
- Frame timing loop targeting 30 FPS
- Classic Matrix rain effect with multi-column rain
- True-color gradient trails (white-hot head to dark green tail)
- Half-width katakana + digit + symbol character set
- Gold highlight characters
- Character mutation (flickering)
- Variable column speeds and trail lengths

## Tech Stack

- **Language**: Rust (edition 2024)
- **Terminal**: [crossterm](https://crates.io/crates/crossterm) (cross-platform terminal manipulation)
- **CLI**: [clap](https://crates.io/crates/clap) (argument parsing)
- **RNG**: [rand](https://crates.io/crates/rand) (character selection, timing)

## License

MIT
