// TODO: remove allow
#![allow(clippy::derivable_impls, dead_code)]

use std::collections::HashMap;

use config::{Config, ConfigError, File, Source, Value};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {}

impl Settings {
    pub fn new() -> Self {
        let user_config_file = File::from(
            dirs::config_dir()
                .expect("No config directory found")
                .join("rsmpc/config"),
        );

        let settings = Config::builder()
            .add_source(Settings {})
            // TODO: add system config
            // .add_source(system_config_file)
            .add_source(user_config_file)
            .build()
            .unwrap();

        settings.try_deserialize().unwrap()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {}
    }
}

impl Source for Settings {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<HashMap<String, Value>, ConfigError> {
        Ok(HashMap::new())
    }
}
