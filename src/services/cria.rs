use std::{path::Path, borrow::Cow};

use tokio::process::Command;
use anyhow::{Result, Context};

use crate::base::types::{HasName, IsEnsurable, is_binary_present, MapStatus, Void, Res};

static NAME: &str = "cria";

#[derive(Default)]
pub struct Cria {
    model_path: Option<String>,
    data_path: String,
    port: Option<u16>
}

impl HasName for Cria {
    fn name(&self) -> &'static str {
        NAME
    }
}

impl IsEnsurable for Cria {
    async fn is_present(&self) -> Res<bool> {
        let _ = self.resolve_port()?;

        // grep for `cria` in `docker ps` output.
        let output = Command::new("docker")
            .arg("ps")
            .output().await
            .context("Unable to run `docker ps`.")?;

        if !output.status.success() {
            return Err(anyhow::Error::msg("The exit code of the `docker ps` operation was not successful."));
        }

        let stdout = String::from_utf8(output.stdout)?;

        Ok(stdout.contains("cria"))
    }

    async fn make_present(&self) -> Void {
        let path = self.resolve_path()?;
        let path = Path::new(path);

        let path = if cfg!(target_os = "windows") && path.is_relative() {
            Cow::Owned(format!("$pwd/{}", path.to_string_lossy()))
        } else {
            path.to_string_lossy()
        };

        let compose = GPU_COMPOSE
            .replace("{{port}}", &self.resolve_port()?.to_string())
            .replace("{{model}}", &path);

        let compose_path = format!("{}/docker-compose.yml", self.data_path);

        std::fs::write(&compose_path, &compose)?;

        Command::new("docker-compose")
            .arg("-f")
            .arg("docker-compose.yml")
            .arg("up")
            .arg("-d")
            .status().await
            .map_status()
            .context("Unable to run `docker-compose`.")?;

        Ok(())
    }
}

impl Cria {
    pub fn new(model_path: &Option<String>, data_path: &str, port: Option<u16>) -> Self {
        Self {
            model_path: model_path.clone(),
            data_path: data_path.to_string(),
            port
        }
    }

    fn resolve_port(&self) -> Res<u16> {
        self.port.ok_or_else(|| anyhow::Error::msg("No cria port provided.  Please provide a `cria_port` config value."))
    }

    fn resolve_path(&self) -> Res<&str> {
        self.model_path.as_deref().ok_or_else(|| anyhow::Error::msg("No model path provided.  Please provide a `model_path` config value, or use the OpenAI mode."))
    }
}

// Statics.

static GPU_COMPOSE: &str = r#"
version: "3.8"

services:
  cria:
    image: twitchax/cria-gpu:2023.09.20
    ports:
      - {{port}}:{{port}}
    volumes:
      - {{model}}:/app/model.bin
    environment:
      - CRIA_SERVICE_NAME=cria
      - CRIA_HOST=0.0.0.0
      - CRIA_PORT={{port}}
      - CRIA_ZIPKIN_ENDPOINT=http://zipkin-server:9411/api/v2/spans
      - CRIA_CONTEXT_SIZE=65536
      - CRIA_USE_GPU=true
      - CRIA_GPU_LAYERS=32
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: 1
              capabilities: [ gpu ]
  zipkin-server:
    image: openzipkin/zipkin
    ports:
      - "9411:9411"
"#;