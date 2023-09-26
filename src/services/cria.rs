use std::{path::Path, borrow::Cow, time::Duration};

use tokio::process::Command;
use anyhow::Context;

use crate::base::types::{HasName, IsEnsurable, MapStatus, Void, Res, Mode, IsRemovable, TAB};

static NAME: &str = "cria_server";

pub struct Cria {
    model_path: Option<String>,
    data_path: String,
    mode: Mode,
    port: Option<u16>
}

impl HasName for Cria {
    fn name(&self) -> &'static str {
        NAME
    }
}

impl IsEnsurable for Cria {
    async fn is_present(&self) -> Res<bool> {
        let port = self.resolve_port()?;

        // grep for `cria` in `docker ps` output.
        let output = Command::new("curl")
            .arg(format!("http://localhost:{}/v1/models", port))
            .output().await
            .context("Unable to curl the local server.")?;

        Ok(output.status.success())
    }

    async fn make_present(&self) -> Void {
        let _ = self.resolve_port()?;
        let model_path = self.resolve_model_path()?;
        let model_path = Path::new(model_path);

        let path = if cfg!(target_os = "windows") && model_path.is_relative() {
            let interim = model_path.canonicalize()?.to_owned().to_string_lossy().to_string();
            let drive = interim.chars().collect::<Vec<_>>()[4].to_ascii_lowercase();
            let path = format!("//{}{}", drive, &interim[6..].replace('\\', "/"));
            
            Cow::Owned(path)
        } else {
            model_path.to_string_lossy()
        };

        let compose_template = if self.mode.is_local_gpu() {
            GPU_COMPOSE
        } else {
            CPU_COMPOSE
        };

        let compose = compose_template
            .replace("{{port}}", &self.resolve_port()?.to_string())
            .replace("{{model}}", &path);

        let compose_path = format!("{}/docker-compose.yml", self.data_path);

        println!("{}Creating `{}` using model path `{}` ... ", TAB, compose_path, path);

        std::fs::write(&compose_path, &compose)?;

        Command::new("docker-compose")
            .arg("-p")
            .arg("cria")
            .arg("-f")
            .arg(compose_path)
            .arg("up")
            .arg("-d")
            .status().await
            .map_status()
            .context("Unable to run `docker-compose`.")?;

        for _ in 0..10 {
            if self.is_present().await? {
                return Ok(());
            }

            tokio::time::sleep(Duration::from_secs(10)).await;
            println!("{}Waiting for Cria to start ... ", TAB)
        }

        Err(anyhow::Error::msg("Unable to start the Cria server (timed out)."))
    }
}

impl IsRemovable for Cria {
    async fn make_not_present(&self) -> Res<()> {
        let compose_path = format!("{}/docker-compose.yml", self.data_path);

        println!("{}Removing `{}` ... ", TAB, compose_path);

        Command::new("docker-compose")
            .arg("-p")
            .arg("cria")
            .arg("-f")
            .arg(compose_path)
            .arg("down")
            .status().await
            .map_status()
            .unwrap();
            //.context("Unable to run `docker-compose`.")?;

        Ok(())
    }
}

impl Cria {
    pub fn new(model_path: &Option<String>, data_path: &str, mode: Mode, port: Option<u16>) -> Self {
        Self {
            model_path: model_path.clone(),
            data_path: data_path.to_string(),
            mode,
            port
        }
    }

    fn resolve_port(&self) -> Res<u16> {
        self.port.ok_or_else(|| anyhow::Error::msg("No cria port provided.  Please provide a `cria_port` config value."))
    }

    fn resolve_model_path(&self) -> Res<&str> {
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

static CPU_COMPOSE: &str = r#"
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
  zipkin-server:
    image: openzipkin/zipkin
    ports:
      - "9411:9411"
"#;