use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub mode: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub app: AppConfig,
}

impl Settings {
    pub fn load() -> Result<Self> {
        Ok(Self {
            app: AppConfig {
                mode: "paper".to_string(),
            },
        })
    }
}
