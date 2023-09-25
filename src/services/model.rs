use tokio::process::Command;
use anyhow::Context;

use crate::base::types::{HasName, IsEnsurable, MapStatus, Res};

static NAME: &str = "model";

pub struct Model {
    model_path: Option<String>,
    model_url: Option<String>,
}

impl HasName for Model {
    fn name(&self) -> &'static str {
        NAME
    }
}

impl IsEnsurable for Model {
    async fn is_present(&self) -> Res<bool> {
        let path = self.resolve_path()?;

        Ok(std::fs::try_exists(path)?)
    }

    async fn make_present(&self) -> Res<()> {
        let path = self.resolve_path()?;
        let url = self.resolve_url()?;

        Command::new("curl")
            .arg("--progress-bar")
            .arg("-fSL")
            .arg(url)
            .arg("-o")
            .arg(path)
            .status().await
            .map_status()
            .context("Unable to curl the model.")?;

        Ok(())
    }
}

impl Model {
    pub fn new(model_path: &Option<String>, model_url: &Option<String>) -> Self {
        Self {
            model_path: model_path.clone(),
            model_url: model_url.clone(),
        }
    }

    fn resolve_url(&self) -> Res<&str> {
        self.model_url.as_deref().ok_or_else(|| anyhow::Error::msg("No model URL provided.  Please provide a `model_url` config value, or use the OpenAI mode."))
    }

    fn resolve_path(&self) -> Res<&str> {
        self.model_path.as_deref().ok_or_else(|| anyhow::Error::msg("No model path provided.  Please provide a `model_path` config value, or use the OpenAI mode."))
    }
}