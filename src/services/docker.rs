use tokio::process::Command;
use anyhow::Context;
use yansi::Paint;

use crate::base::types::{is_binary_present, HasName, IsEnsurable, MapStatus, Res, Void, TAB};

static NAME: &str = "docker";

#[derive(Default)]
pub struct Docker {}

impl HasName for Docker {
    fn name(&self) -> &'static str {
        NAME
    }
}

impl IsEnsurable for Docker {
    async fn is_present(&self) -> Res<bool> {
        is_binary_present(self).await
    }

    async fn make_present(&self) -> Void {
        if cfg!(target_os = "windows") {
            println!("{}{}: Please install `{}` manually on Windows.", TAB, Paint::red("✘"), Paint::blue(NAME));
            return Err(anyhow::anyhow!("User skipped required operation."));
        }
        
        Command::new("curl")
            .arg("-fsSL")
            .arg("https://get.docker.com")
            .arg("-o")
            .arg("get-docker.sh")
            .status().await
            .map_status()
            .context("Unable to curl the docker convenience script.")?;

        Command::new("sh")
            .arg("get-docker.sh")
            .status().await
            .map_status()
            .context("Unable to run the docker install script (might need sudo).")?;

        // Ignore failure: might be in container.
        Command::new("usermod")
            .arg("-aG")
            .arg("docker")
            .arg("$USER")
            .status().await
            .map_status().unwrap_or(());
                
        Command::new("rm")
            .arg("-f")
            .arg("get-docker.sh")
            .status().await
            .map_status()
            .context("Failed to delete the docker install script.")?;

        Ok(())
    }
}