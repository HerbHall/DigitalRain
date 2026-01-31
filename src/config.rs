//! CLI argument parsing, TOML configuration, and runtime configuration.
//!
//! Uses clap's derive API for command-line arguments and serde/toml for
//! persistent configuration files with named presets.
//!
//! Priority resolution: CLI explicit arg > preset value > config [defaults] > hardcoded default

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use clap::Parser;
use serde::{Deserialize, Serialize};

// ---------- CLI Definition ----------

/// Terminal-based Matrix digital rain effect.
#[derive(Parser)]
#[command(name = "digital_rain", version, about)]
pub struct Cli {
    /// Effect to display
    #[arg(short, long)]
    pub effect: Option<String>,

    /// Animation speed multiplier (0.1 = slow, 1.0 = normal, 3.0 = fast)
    #[arg(short, long, value_parser = clap::value_parser!(f64))]
    pub speed: Option<f64>,

    /// Rain density (0.1 = sparse, 1.0 = normal, 3.0 = heavy)
    #[arg(short, long, value_parser = clap::value_parser!(f64))]
    pub density: Option<f64>,

    /// Color palette
    #[arg(short, long)]
    pub color: Option<String>,

    /// Character set to use
    #[arg(long)]
    pub charset: Option<String>,

    /// Target frames per second
    #[arg(long, value_parser = clap::value_parser!(u32))]
    pub fps: Option<u32>,

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

    /// CRT effect intensity (0.0 = off, 1.0 = maximum)
    #[arg(long, value_parser = clap::value_parser!(f64))]
    pub crt_intensity: Option<f64>,

    /// Path to config file (default: platform config dir)
    #[arg(long)]
    pub config: Option<String>,

    /// Load a named preset from the config file
    #[arg(long)]
    pub preset: Option<String>,

    /// Save current CLI args as a named preset and exit
    #[arg(long)]
    pub save_preset: Option<String>,

    /// List available presets from the config file and exit
    #[arg(long)]
    pub list_presets: bool,
}

// ---------- TOML Config File Structs ----------

/// Top-level config file structure.
#[derive(Deserialize, Serialize, Default)]
pub struct ConfigFile {
    #[serde(default)]
    pub defaults: ConfigDefaults,
    #[serde(default)]
    pub presets: HashMap<String, PresetConfig>,
}

/// Default settings applied when no CLI or preset overrides.
#[derive(Deserialize, Serialize, Default)]
pub struct ConfigDefaults {
    pub effect: Option<String>,
    pub speed: Option<f64>,
    pub density: Option<f64>,
    pub color: Option<String>,
    pub charset: Option<String>,
    pub fps: Option<u32>,
    pub crt: Option<bool>,
    pub crt_intensity: Option<f64>,
}

/// A named preset: partial config that can override defaults.
#[derive(Deserialize, Serialize, Default)]
pub struct PresetConfig {
    pub effect: Option<String>,
    pub speed: Option<f64>,
    pub density: Option<f64>,
    pub color: Option<String>,
    pub charset: Option<String>,
    pub fps: Option<u32>,
    pub crt: Option<bool>,
    pub crt_intensity: Option<f64>,
}

// ---------- Config File I/O ----------

/// Get the default config file path for the current platform.
///
/// Windows: %APPDATA%\digitalrain\config.toml
/// Linux:   ~/.config/digitalrain/config.toml
/// macOS:   ~/Library/Application Support/digitalrain/config.toml
pub fn config_file_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("digitalrain").join("config.toml"))
}

/// Load and parse the config file. Returns Default if file doesn't exist or is invalid.
pub fn load_config_file(path: Option<&str>) -> ConfigFile {
    let path = match path {
        Some(p) => PathBuf::from(p),
        None => match config_file_path() {
            Some(p) => p,
            None => return ConfigFile::default(),
        },
    };

    match fs::read_to_string(&path) {
        Ok(content) => match toml::from_str(&content) {
            Ok(config) => config,
            Err(e) => {
                eprintln!(
                    "Warning: could not parse config file {}: {}",
                    path.display(),
                    e
                );
                ConfigFile::default()
            }
        },
        Err(_) => ConfigFile::default(), // File doesn't exist yet, that's fine
    }
}

/// Save a config file to disk, creating directories as needed.
fn save_config_file(config: &ConfigFile, path: Option<&str>) -> Result<(), String> {
    let path = match path {
        Some(p) => PathBuf::from(p),
        None => config_file_path().ok_or("Could not determine config directory")?,
    };

    // Create parent directory if needed
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Could not create config directory: {}", e))?;
    }

    let content =
        toml::to_string_pretty(config).map_err(|e| format!("Could not serialize config: {}", e))?;
    fs::write(&path, content).map_err(|e| format!("Could not write config file: {}", e))?;

    Ok(())
}

/// Save current CLI args as a named preset.
pub fn save_preset(cli: &Cli, name: &str) -> Result<PathBuf, String> {
    let config_path = cli.config.as_deref();
    let mut config_file = load_config_file(config_path);

    let preset = PresetConfig {
        effect: cli.effect.clone(),
        speed: cli.speed,
        density: cli.density,
        color: cli.color.clone(),
        charset: cli.charset.clone(),
        fps: cli.fps,
        crt: if cli.crt { Some(true) } else { None },
        crt_intensity: cli.crt_intensity,
    };

    config_file.presets.insert(name.to_string(), preset);
    save_config_file(&config_file, config_path)?;

    let path = config_path
        .map(PathBuf::from)
        .or_else(config_file_path)
        .unwrap_or_default();
    Ok(path)
}

/// Print all presets from the config file.
pub fn print_presets(cli: &Cli) {
    let config_file = load_config_file(cli.config.as_deref());

    if config_file.presets.is_empty() {
        println!("No presets defined.");
        println!();
        println!("Create one with: digital_rain --save-preset <name> [options]");
        if let Some(path) = config_file_path() {
            println!("Config file: {}", path.display());
        }
        return;
    }

    println!("Available presets:");
    let mut names: Vec<_> = config_file.presets.keys().collect();
    names.sort();
    for name in names {
        let p = &config_file.presets[name];
        let mut parts = Vec::new();
        if let Some(ref e) = p.effect {
            parts.push(format!("effect={}", e));
        }
        if let Some(ref c) = p.color {
            parts.push(format!("color={}", c));
        }
        if let Some(s) = p.speed {
            parts.push(format!("speed={:.1}", s));
        }
        if let Some(d) = p.density {
            parts.push(format!("density={:.1}", d));
        }
        if let Some(ref cs) = p.charset {
            parts.push(format!("charset={}", cs));
        }
        if let Some(true) = p.crt {
            parts.push("crt=on".to_string());
        }
        if let Some(i) = p.crt_intensity {
            parts.push(format!("crt_intensity={:.1}", i));
        }
        if let Some(f) = p.fps {
            parts.push(format!("fps={}", f));
        }
        let desc = if parts.is_empty() {
            "(empty)".to_string()
        } else {
            parts.join(", ")
        };
        println!("  {:<16} {}", name, desc);
    }
}

// ---------- Runtime Config ----------

/// Runtime configuration derived from CLI + config file + presets.
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
    /// Resolve config from CLI args, preset, and config file defaults.
    /// Priority: CLI explicit > preset > config defaults > hardcoded default
    pub fn resolve(cli: &Cli, config_file: &ConfigFile) -> Self {
        let preset = cli
            .preset
            .as_ref()
            .and_then(|name| config_file.presets.get(name));

        Self {
            effect_name: cli
                .effect
                .clone()
                .or_else(|| preset.and_then(|p| p.effect.clone()))
                .or_else(|| config_file.defaults.effect.clone())
                .unwrap_or_else(|| "classic".to_string()),
            speed_multiplier: cli
                .speed
                .or(preset.and_then(|p| p.speed))
                .or(config_file.defaults.speed)
                .unwrap_or(1.0)
                .clamp(0.1, 10.0),
            density_multiplier: cli
                .density
                .or(preset.and_then(|p| p.density))
                .or(config_file.defaults.density)
                .unwrap_or(1.0)
                .clamp(0.1, 10.0),
            palette_name: cli
                .color
                .clone()
                .or_else(|| preset.and_then(|p| p.color.clone()))
                .or_else(|| config_file.defaults.color.clone())
                .unwrap_or_else(|| "classic".to_string()),
            charset_name: cli
                .charset
                .clone()
                .or_else(|| preset.and_then(|p| p.charset.clone()))
                .or_else(|| config_file.defaults.charset.clone())
                .unwrap_or_else(|| "matrix".to_string()),
            target_fps: cli
                .fps
                .or(preset.and_then(|p| p.fps))
                .or(config_file.defaults.fps)
                .unwrap_or(30)
                .clamp(10, 120),
            auto_cycle_secs: cli.timer.map(|t| t.max(1.0)),
            forward: cli.forward,
            crt_enabled: cli.crt
                || preset.and_then(|p| p.crt).unwrap_or(false)
                || config_file.defaults.crt.unwrap_or(false),
            crt_intensity: cli
                .crt_intensity
                .or(preset.and_then(|p| p.crt_intensity))
                .or(config_file.defaults.crt_intensity)
                .unwrap_or(0.7)
                .clamp(0.0, 1.0),
        }
    }

    /// Create a Config from CLI arguments only (backwards-compatible convenience).
    pub fn from_cli(cli: &Cli) -> Self {
        let config_file = load_config_file(cli.config.as_deref());
        Self::resolve(cli, &config_file)
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
            crt_enabled: rng.random_range(0.0..1.0) < 0.07, // ~7% chance
            crt_intensity: 0.7,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_uses_hardcoded_defaults_when_nothing_set() {
        let cli = Cli::parse_from(["digital_rain"]);
        let config_file = ConfigFile::default();
        let config = Config::resolve(&cli, &config_file);

        assert_eq!(config.effect_name, "classic");
        assert!((config.speed_multiplier - 1.0).abs() < 0.01);
        assert!((config.density_multiplier - 1.0).abs() < 0.01);
        assert_eq!(config.palette_name, "classic");
        assert_eq!(config.charset_name, "matrix");
        assert_eq!(config.target_fps, 30);
        assert!(!config.crt_enabled);
        assert!((config.crt_intensity - 0.7).abs() < 0.01);
    }

    #[test]
    fn resolve_cli_overrides_defaults() {
        let cli = Cli::parse_from(["digital_rain", "-e", "fire", "-s", "2.5", "-c", "red"]);
        let mut config_file = ConfigFile::default();
        config_file.defaults.effect = Some("ocean".to_string());
        config_file.defaults.speed = Some(0.5);

        let config = Config::resolve(&cli, &config_file);
        assert_eq!(config.effect_name, "fire");
        assert!((config.speed_multiplier - 2.5).abs() < 0.01);
        assert_eq!(config.palette_name, "red");
    }

    #[test]
    fn resolve_preset_overrides_defaults() {
        let cli = Cli::parse_from(["digital_rain", "--preset", "cyber"]);
        let mut config_file = ConfigFile::default();
        config_file.defaults.effect = Some("classic".to_string());
        config_file.presets.insert(
            "cyber".to_string(),
            PresetConfig {
                effect: Some("glitch".to_string()),
                speed: Some(1.5),
                color: Some("purple".to_string()),
                ..Default::default()
            },
        );

        let config = Config::resolve(&cli, &config_file);
        assert_eq!(config.effect_name, "glitch");
        assert!((config.speed_multiplier - 1.5).abs() < 0.01);
        assert_eq!(config.palette_name, "purple");
    }

    #[test]
    fn resolve_cli_overrides_preset() {
        let cli = Cli::parse_from(["digital_rain", "--preset", "cyber", "-s", "3.0"]);
        let mut config_file = ConfigFile::default();
        config_file.presets.insert(
            "cyber".to_string(),
            PresetConfig {
                speed: Some(1.5),
                effect: Some("glitch".to_string()),
                ..Default::default()
            },
        );

        let config = Config::resolve(&cli, &config_file);
        // CLI speed overrides preset speed
        assert!((config.speed_multiplier - 3.0).abs() < 0.01);
        // Preset effect used since CLI didn't specify
        assert_eq!(config.effect_name, "glitch");
    }

    #[test]
    fn resolve_clamps_values() {
        let cli = Cli::parse_from(["digital_rain", "-s", "100.0", "--fps", "1"]);
        let config_file = ConfigFile::default();
        let config = Config::resolve(&cli, &config_file);

        assert!((config.speed_multiplier - 10.0).abs() < 0.01);
        assert_eq!(config.target_fps, 10);
    }

    #[test]
    fn config_file_roundtrip() {
        let mut config = ConfigFile::default();
        config.defaults.effect = Some("classic".to_string());
        config.defaults.speed = Some(1.5);
        config.presets.insert(
            "test".to_string(),
            PresetConfig {
                effect: Some("fire".to_string()),
                ..Default::default()
            },
        );

        let serialized = toml::to_string_pretty(&config).unwrap();
        let deserialized: ConfigFile = toml::from_str(&serialized).unwrap();

        assert_eq!(deserialized.defaults.effect, Some("classic".to_string()));
        assert_eq!(deserialized.defaults.speed, Some(1.5));
        assert!(deserialized.presets.contains_key("test"));
        assert_eq!(
            deserialized.presets["test"].effect,
            Some("fire".to_string())
        );
    }
}
