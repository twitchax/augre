use tokio::process::Command;
use anyhow::{Result, Context};

use crate::base::types::{Nameable, Ensurable, is_binary_present, MapStatus};

static NAME: &str = "git";

#[derive(Default)]
pub struct Git {}

impl Nameable for Git {
    fn name(&self) -> &'static str {
        NAME
    }
}

impl Ensurable for Git {
    async fn is_present(&self) -> Result<bool> {
        is_binary_present(self).await
    }

    async fn make_present(&self) -> Result<()> {
        Command::new("apt-get")
            .arg("update")
            .status().await
            .map_status()
            .context("Unable to update apt-get.")?;
    
        Command::new("apt-get")
            .arg("-y")
            .arg("install")
            .arg("git")
            .status().await
            .map_status()
            .context("Unable to install git via apt-get.  You can install git manually, and try again.")?;

        Command::new("which")
            .arg("git")
            .status().await
            .map_status()
            .context("Unable to verify git installation.")?;

        Ok(())
    }
}

impl Git {
    pub async fn diff() -> Result<String> {
        let output = Command::new("git")
            .arg("diff")
            .arg("HEAD^")
            .output().await
            .context("Unable to run `git diff HEAD^`.")?;

        if !output.status.success() {
            return Err(anyhow::Error::msg("The exit code of the `git diff HEAD^` operation was not successful."));
        }

        let stdout = String::from_utf8(output.stdout)?;

        Ok(stdout)
    }
}