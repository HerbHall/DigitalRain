# DigitalRain - Requirements Specification

## Vision
A terminal-based visual effects application inspired by the Matrix digital rain.
Windows-first, cross-platform, with a flexible effect system and rich customization.

## Core Requirements

### CR-1: Classic Matrix Rain Effect
- Green phosphor characters falling in vertical columns
- Half-width katakana characters (film-authentic) mixed with ASCII digits and symbols
- Leading character is bright white/green, trail fades through progressively darker greens
- Characters randomly mutate (flicker/change) within active streams
- Variable column speeds, lengths, and densities
- Gold highlight characters appear occasionally (as in the film)

### CR-2: True Color Rendering
- 24-bit RGB color support for smooth gradient trails
- Phosphor green primary palette: bright white lead -> vivid green -> dark green -> near-black tail
- Gold accent palette for highlight characters
- Graceful degradation to 256-color on limited terminals

### CR-3: Windows-First Cross-Platform
- Primary target: Windows Terminal, PowerShell, cmd.exe
- Secondary: Linux terminals, macOS Terminal/iTerm2
- Single standalone binary (no runtime dependencies)
- Proper terminal cleanup on exit (Ctrl+C, window close)

### CR-4: Command-Line Interface
- `--effect <name>` - Select which effect to run
- `--speed <value>` - Control animation speed
- `--density <value>` - Control rain density (sparse to heavy)
- `--color <palette>` - Select color palette/theme
- `--charset <name>` - Select character set
- `--fps <value>` - Target frame rate (default 30)
- `--list-effects` - Show available effects
- `--random` - Pick a random effect with random parameters
- `--help` - Usage information

### CR-5: Effect System Architecture
- Common `Effect` trait that all effects implement
- Effects are self-contained modules with their own state
- Easy to add new effects by implementing the trait
- Effect registry for discovery and selection
- Compositor to manage active effect rendering

### CR-6: Configuration
- TOML config file for persistent settings (~/.config/digitalrain/config.toml)
- Named presets bundling palette + speed + density + charset + effect
- CLI arguments override config file values
- Built-in presets: "classic", "gold", "monochrome", "cyberpunk"

### CR-7: Interactive Runtime Controls
- `q` / `Esc` - Quit
- `Space` - Pause/resume
- `+` / `-` - Increase/decrease speed
- `[` / `]` - Decrease/increase density
- `n` - Next effect (cycle through effects)
- `r` - Randomize parameters
- `?` - Show keybindings overlay

## Future / Stretch Goals

### SG-1: CRT Simulation
- Scanline effect using dim/bright alternating rows
- Phosphor glow approximation using background colors on adjacent cells
- Screen flicker/noise

### SG-2: Depth/Parallax Layers
- Multiple rain layers at different speeds and brightness levels
- Foreground: fast, bright, large characters
- Background: slow, dim, creates depth illusion

### SG-3: Embedded Messages
- "WAKE UP NEO" style text appearing within the rain
- Configurable message text
- Message dissolves back into rain after display

### SG-4: Additional Effects
- Cascade: top-down wave revealing characters
- Pulse: brightness waves radiating outward
- Glitch: random block distortion and displacement
- Binary: 0s and 1s only, hacker aesthetic
- Fire: warm color palette flowing upward
- Ocean: blue palette, horizontal wave motion

### SG-5: Audio-Reactive Mode
- Modulate rain intensity/speed based on system audio

### SG-6: Performance Dashboard
- FPS counter overlay
- CPU usage display
- Adaptive frame rate based on terminal size

## Technical Constraints

- Minimum terminal size: 40x20
- Target frame rate: 30fps (configurable)
- Maximum CPU usage: reasonable for a background screensaver-like app
- Must handle terminal resize gracefully
- Must restore terminal state on any exit path (panic, Ctrl+C, normal exit)

## Dependencies (Planned)

| Crate | Purpose | Version |
|---|---|---|
| crossterm | Terminal manipulation | latest |
| clap | CLI argument parsing | 4.x |
| rand | Random number generation | latest |
| toml | Config file parsing | latest |
| serde | Serialization for config | latest |

## Phased Development

### Phase 1: Foundation
- Project structure, build system
- Terminal setup/teardown (alternate screen, raw mode, cleanup)
- Cell buffer and rendering pipeline
- Frame timing loop
- Basic single-column rain (proof of concept)

### Phase 2: Classic Rain Effect
- Full multi-column rain with variable speeds/lengths
- Character sets (katakana, ASCII, digits)
- True-color gradient trails
- Character mutation (flickering)
- Gold highlight characters

### Phase 3: CLI & Configuration
- clap-based argument parsing
- TOML config file support
- Built-in presets
- Effect selection via CLI

### Phase 4: Interactive Controls & Polish
- Runtime keyboard controls
- Keybindings overlay
- Terminal resize handling
- Graceful degradation on limited terminals
- Performance optimization

### Phase 5: Additional Effects
- Effect trait finalization
- Additional effect implementations
- Random effect mode
- Effect transitions
