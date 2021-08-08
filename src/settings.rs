use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Source {
    pub path: PathBuf,
    pub enabled: bool,
    pub excludes: HashSet<PathBuf>,
}

impl Default for Source {
    fn default() -> Self {
        Source {
            path: PathBuf::new(),
            enabled: true,
            excludes: HashSet::new(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Settings {
    pub debug: bool,
    pub excludes: HashSet<PathBuf>,
    pub sources: HashMap<String, Source>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            debug: false,
            excludes: HashSet::new(),
            sources: HashMap::new(),
        }
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        let home_dir = dirs::home_dir();
        let config_dir = home_dir.unwrap().join(".config/sprinkle/config");

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name(config_dir.to_str().unwrap()))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("sprinkle"))?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}
