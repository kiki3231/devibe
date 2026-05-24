use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub theme: Option<String>,
    #[serde(default = "default_days")]
    pub days: u32,
    #[serde(default)]
    pub exclude: Vec<String>,
}

fn default_days() -> u32 {
    14
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: None,
            days: 14,
            exclude: Vec::new(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let path = config_path();
        if path.exists() {
            std::fs::read_to_string(&path)
                .ok()
                .and_then(|s| toml::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            Self::default()
        }
    }

    #[allow(dead_code)]
    pub fn find_up() -> Option<PathBuf> {
        let mut current = std::env::current_dir().ok()?;
        loop {
            let candidate = current.join(".devibe.toml");
            if candidate.exists() {
                return Some(candidate);
            }
            if !current.pop() {
                break;
            }
        }
        None
    }
}

fn config_path() -> PathBuf {
    dirs_config()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("devibe")
        .join("config.toml")
}

fn dirs_config() -> Option<PathBuf> {
    if let Ok(dir) = std::env::var("XDG_CONFIG_HOME") {
        return Some(PathBuf::from(dir));
    }
    if let Ok(home) = std::env::var("HOME") {
        return Some(PathBuf::from(home).join(".config"));
    }
    None
}
