//! The configuration module.

use anyhow::Context;
use config::{Environment, File};
use serde::{Deserialize, Serialize};

use super::types::{Res, Mode};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct OptionalConfig {
    openai_key: Option<String>,
    mode: Option<Mode>,
    model_url: Option<String>,
}

/// The configuration type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub openai_endpoint: String,
    pub mode: Mode,
    pub data_path: String,

    pub openai_key: Option<String>,
    pub model_url: Option<String>,
}

impl Config {
    /// Initializes a new [`Config`] object from the specified configuration path.
    ///
    /// Alternatively, this method will fallback to environment variables with the
    /// prefix `RTZ` (e.g., `RTZ_BIND_ADDRESS`).
    pub fn new(data_path: &str, mode: Mode) -> Res<Self> {
        let builder = config::Config::builder()
            .add_source(File::with_name(&format!("{}/config.toml", data_path)).required(false))
            .add_source(Environment::with_prefix("augre"));

        let optional_config: OptionalConfig = builder.build()?.try_deserialize()?;

        let mode = optional_config.mode.unwrap_or(mode);

        let openai_endpoint = match mode {
            Mode::LocalCpu | Mode::LocalGpu => {
                println!("{}: Running in local mode.", yansi::Paint::yellow("WARN"));
                "http://localhost:3000".to_string()
            },
            Mode::OpenAi => {
                println!("{}: Running in OpenAI mode.", yansi::Paint::yellow("WARN"));
                "https://api.openai.com".to_string()
            },
        };

        let config = Config {
            openai_endpoint,
            openai_key: optional_config.openai_key,
            mode,
            data_path: data_path.to_string(),
            model_url: optional_config.model_url,
        };

        Ok(config)
    }
}
