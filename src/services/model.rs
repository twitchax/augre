use tokio::process::Command;
use anyhow::{Result, Context};

use crate::base::types::{is_binary_present, HasName, IsEnsurable, MapStatus, Mode, Res};

static NAME: &str = "docker";

pub struct Model {
    data_path: String,
    model_url: Option<String>,
}

impl HasName for Model {
    fn name(&self) -> &'static str {
        NAME
    }
}

impl IsEnsurable for Model {
    async fn is_present(&self) -> Res<bool> {
        let url = self.resolve_url()?;

        let file_name = url.split('/').last().ok_or_else(|| anyhow::Error::msg("Unable to get the file name from the model URL."))?;

        Ok(std::fs::try_exists(file_name)?)
    }

    async fn make_present(&self) -> Res<()> {
        let url = self.resolve_url()?;

        Command::new("curl")
            .arg("-fsSL")
            .arg(url)
            .arg("-o")
            .arg(&self.data_path)
            .status().await
            .map_status()
            .context("Unable to curl the model.")?;

        Ok(())
    }
}

impl Model {
    pub fn new(data_path: &str, url: &Option<String>) -> Self {
        Self {
            data_path: data_path.to_string(),
            model_url: url.clone(),
        }
    }

    fn resolve_url(&self) -> Res<&str> {
        self.model_url.as_deref().ok_or_else(|| anyhow::Error::msg("No model URL provided.  Please provide a `model_url` config value, or use the OpenAI mode."))
    }
}