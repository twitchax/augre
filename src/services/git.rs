use tokio::process::Command;
use anyhow::Context;
use yansi::Paint;

use crate::base::types::{HasName, IsEnsurable, is_binary_present, MapStatus, Res, Void, TAB};

static NAME: &str = "git";

#[derive(Default)]
pub struct Git {}

impl HasName for Git {
    fn name(&self) -> &'static str {
        NAME
    }
}

impl IsEnsurable for Git {
    async fn is_present(&self) -> Res<bool> {
        is_binary_present(self).await
    }

    async fn make_present(&self) -> Void {
        if cfg!(target_os = "windows") {
            println!("{}{}: Please install `{}` manually on Windows.", TAB, Paint::red("✘"), Paint::blue(NAME));
            return Err(anyhow::anyhow!("User skipped required operation."));
        }

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
    pub async fn diff() -> Res<String> {
        let output = Command::new("git")
            .arg("diff")
            .output().await
            .context("Unable to run `git diff`.")?;

        if !output.status.success() {
            return Err(anyhow::Error::msg("The exit code of the `git diff` operation was not successful."));
        }

        let stdout = String::from_utf8(output.stdout)?;

        Ok(stdout)
    }
}