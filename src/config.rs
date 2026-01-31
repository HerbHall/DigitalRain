//! CLI argument parsing and runtime configuration.
//!
//! Uses clap's derive API to define command-line arguments. The parsed
//! arguments are converted into a Config struct that the rest of the
//! application uses.

use clap::Parser;

/// Terminal-based Matrix digital rain effect.
#[derive(Parser)]
#[command(name = "digital_rain", version, about)]
pub struct Cli {
    /// Effect to display
    #[arg(short, long, default_value = "classic")]
    pub effect: String,

    /// Animation speed multiplier (0.1 = slow, 1.0 = normal, 3.0 = fast)
    #[arg(short, long, default_value_t = 1.0, value_parser = clap::value_parser!(f64))]
    pub speed: f64,

    /// Rain density (0.1 = sparse, 1.0 = normal, 3.0 = heavy)
    #[arg(short, long, default_value_t = 1.0, value_parser = clap::value_parser!(f64))]
    pub density: f64,

    /// Color palette
    #[arg(short, long, default_value = "classic")]
    pub color: String,

    /// Character set to use
    #[arg(long, default_value = "matrix")]
    pub charset: String,

    /// Target frames per second
    #[arg(long, default_value_t = 30)]
    pub fps: u32,

    /// List available effects and exit
    #[arg(long)]
    pub list_effects: bool,

    /// List available color palettes and exit
    #[arg(long)]
    pub list_colors: bool,

    /// List available character sets and exit
    #[arg(long)]
    pub list_charsets: bool,

    /// Pick a random effect with random parameters
    #[arg(long)]
    pub random: bool,

    /// Auto-cycle to a random effect every N seconds (e.g. --timer 30)
    #[arg(long, value_parser = clap::value_parser!(f64))]
    pub timer: Option<f64>,

    /// Forward gradient direction (bright tail at top, dim head at bottom)
    #[arg(long)]
    pub forward: bool,

    /// Enable CRT monitor simulation (scanlines, phosphor glow, flicker, noise)
    #[arg(long)]
    pub crt: bool,

    /// CRT effect intensity (0.0 = off, 1.0 = maximum). Default: 0.7
    #[arg(long, default_value_t = 0.7, value_parser = clap::value_parser!(f64))]
    pub crt_intensity: f64,
}

/// Runtime configuration derived from CLI arguments.
/// This is what gets passed around to effects and subsystems.
pub struct Config {
    pub effect_name: String,
    pub speed_multiplier: f64,
    pub density_multiplier: f64,
    pub palette_name: String,
    pub charset_name: String,
    pub target_fps: u32,
    pub auto_cycle_secs: Option<f64>,
    pub forward: bool,
    pub crt_enabled: bool,
    pub crt_intensity: f64,
}

impl Config {
    /// Create a Config from parsed CLI arguments.
    pub fn from_cli(cli: &Cli) -> Self {
        Self {
            effect_name: cli.effect.clone(),
            speed_multiplier: cli.speed.clamp(0.1, 10.0),
            density_multiplier: cli.density.clamp(0.1, 10.0),
            palette_name: cli.color.clone(),
            charset_name: cli.charset.clone(),
            target_fps: cli.fps.clamp(10, 120),
            auto_cycle_secs: cli.timer.map(|t| t.max(1.0)),
            forward: cli.forward,
            crt_enabled: cli.crt,
            crt_intensity: cli.crt_intensity.clamp(0.0, 1.0),
        }
    }

    /// Create a randomized config.
    pub fn randomized() -> Self {
        use rand::Rng;
        let mut rng = rand::rng();

        let effects = crate::effects::registry::effect_names();
        let palettes = crate::color::palette::palette_names();
        let charsets = crate::rain::chars::charset_names();

        Self {
            effect_name: effects[rng.random_range(0..effects.len())].to_string(),
            speed_multiplier: rng.random_range(0.5..2.5),
            density_multiplier: rng.random_range(0.3..2.0),
            palette_name: palettes[rng.random_range(0..palettes.len())].to_string(),
            charset_name: charsets[rng.random_range(0..charsets.len())].to_string(),
            target_fps: 30,
            auto_cycle_secs: None,
            forward: false,
            crt_enabled: false,
            crt_intensity: 0.7,
        }
    }
}
