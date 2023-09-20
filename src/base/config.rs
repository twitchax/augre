//! The configuration module.

use config::{Environment, File};
use serde::{Deserialize, Serialize};

use super::types::Res;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct OptionalConfig {
    openai_endpoint: Option<String>,
    openai_key: Option<String>,
}

/// The configuration type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub(crate) openai_endpoint: String,
    pub(crate) openai_key: String,
}

impl Config {
    /// Initializes a new [`Config`] object from the specified configuration path.
    ///
    /// Alternatively, this method will fallback to environment variables with the
    /// prefix `RTZ` (e.g., `RTZ_BIND_ADDRESS`).
    pub fn new(config_path: &str) -> Res<Self> {
        let builder = config::Config::builder()
            .add_source(File::with_name(config_path).required(false))
            .add_source(Environment::with_prefix("rtz"));

        let optional_config: OptionalConfig = builder.build()?.try_deserialize()?;

        let config = Config {
            openai_endpoint: optional_config.openai_endpoint.unwrap_or("https://api.openai.com".to_string()),
            openai_key: optional_config.openai_key.unwrap_or_default(),
        };

        Ok(config)
    }
}
