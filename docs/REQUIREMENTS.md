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

### CR-8: CRT Simulation
- Scanline effect using dim/bright alternating rows
- Phosphor glow approximation using background colors on adjacent cells
- Screen flicker/noise
- Toggleable via CLI flag (`--crt`) and runtime key

### CR-9: Depth/Parallax Layers
- Multiple rain layers at different speeds and brightness levels
- Foreground: fast, bright, large characters
- Background: slow, dim, creates depth illusion
- Configurable number of layers and speed/brightness ratios

### CR-10: Additional Effects
- **Classic**: Standard Matrix rain (CR-1)
- **Cascade**: Top-down wave revealing characters
- **Pulse**: Brightness waves radiating outward
- **Glitch**: Random block distortion and displacement
- **Binary**: 0s and 1s only, hacker aesthetic
- **Fire**: Warm color palette (red/orange/yellow) flowing upward
- **Ocean**: Blue palette, horizontal wave motion
- Each effect must implement the Effect trait (CR-5)
- Effects selectable via `--effect` flag and `n` key at runtime

## Future / Stretch Goals

### SG-1: Embedded Messages
- "WAKE UP NEO" style text appearing within the rain
- Configurable message text
- Message dissolves back into rain after display

### SG-2: Audio-Reactive Mode
- Modulate rain intensity/speed based on system audio

### SG-3: Performance Dashboard
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

### Phase 1: Foundation [COMPLETE]
- Project structure, build system
- Terminal setup/teardown (alternate screen, raw mode, cleanup)
- Cell buffer with dirty-cell tracking
- Frame timing loop (30fps target)
- Classic rain effect: multi-column, katakana + digits, true-color gradients,
  character mutation, gold highlights, variable speed/length
- Effect trait scaffolding

### Phase 2: CLI & Core Configuration
- clap-based argument parsing (--effect, --speed, --density, --color, --fps, etc.)
- Effect registry with --list-effects
- --random flag for random effect + parameters
- Speed, density, and color palette as runtime-configurable parameters
- Wire CLI values through to the rain simulation

### Phase 3: Interactive Controls & Polish
- Runtime keyboard controls (pause, speed, density, effect cycling)
- Keybindings overlay (`?` key)
- Terminal resize handling improvements
- Graceful degradation on limited terminals (256-color fallback)
- Performance tuning and frame timing refinement

### Phase 4: CRT Simulation & Depth Layers
- CRT scanline post-processing pass
- Phosphor glow approximation (background color bleed on adjacent cells)
- Screen flicker/noise overlay
- Multi-layer parallax rain (foreground + background at different speeds/brightness)
- `--crt` flag and runtime toggle

### Phase 5: Additional Effects
- Cascade effect
- Pulse effect
- Glitch effect
- Binary effect
- Fire effect
- Ocean effect
- Effect transitions (smooth crossfade between effects)

### Phase 6: Configuration & Presets
- TOML config file loading (~/.config/digitalrain/config.toml)
- Named presets ("classic", "gold", "monochrome", "cyberpunk")
- CLI overrides config file values
- `--preset <name>` flag
- `--save-preset <name>` to persist current settings
